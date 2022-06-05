use crate::audio::AudioError;
use crate::audio;
use crate::file;
use std::path::Path;
use std::convert::AsRef;
use strum_macros::{AsRefStr, Display};
use anyhow::{Context, Result, bail, anyhow};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use if_chain::if_chain;
use tokio::fs;
use tokio::task::{self, JoinHandle};

pub static AUDIO_SUFFIX: &str = "r.mp3";
pub static VIDEO_EXT: Lazy<Vec<&str>> = Lazy::new(|| vec!["webm", "mp4", "ogg"]);
pub static AUDIO_EXT: Lazy<Vec<&str>> = Lazy::new(|| vec!["mp3", "m4a"]);
pub static IMAGE_EXT: Lazy<Vec<&str>> = Lazy::new(|| vec!["jpg", "jpeg", "png", "gif"]);

pub static MEDIA_EXT: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut media: Vec<&str> = Vec::new();

    for ext in (*VIDEO_EXT).iter() {
        media.push(*ext);
    }
    for ext in (*AUDIO_EXT).iter() {
        media.push(*ext);
    }
    for ext in (*IMAGE_EXT).iter() {
        media.push(*ext);
    }

    media
});

#[derive(AsRefStr, Display)]
pub enum AudioState {
    OK,
    UNAVAILABLE,
    RETRY,
}

pub enum MediaType {
    VIDEO,
    IMAGE,
    AUDIO,
    INVALID,
}

pub struct Package {
    pub deck: Deck,
    pub template: Template,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(default)]
    pub id: Option<u16>,
    pub word: String,
    pub reading: Option<String>,
    pub definition: String,
    pub transcription: String,
    #[serde(default)]
    pub audio_state: Option<String>,
    #[serde(default)]
    pub use_reading: bool
}

pub struct Template {
    pub front: String,
    pub back: String,
    pub css: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deck {
    pub id: usize,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub notes: Vec<Note>,
}

impl Deck {
    pub async fn from_file(path: &str) -> Result<Deck> {
        if !Path::new(path).exists() {
            bail!("{} does not exist", path);
        }

        let data = fs::read_to_string(path).await
            .with_context(|| format!("Cannot read {}", path))?;
        let mut deck: Deck = serde_json::from_str(&data)?;
        deck.assign_ids();
        Ok(deck)
    }

    pub fn from_json(json: &str) -> Result<Deck> {
        let mut deck: Deck = serde_json::from_str(json).with_context(|| "Failed to deserialize deck")?;
        deck.assign_ids();
        Ok(deck)
    }

    pub async fn write(&self, dest: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)
            .with_context(|| "Failed to serialize deck.json")?;

        fs::write(dest, json).await
            .with_context(|| "Failed to write deck.json")?;

        Ok(())
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn assign_ids(&mut self) {
        let mut max_id = 0;

        for note in self.notes.iter() {
            if_chain! {
                if let Some(id) = note.id;
                if id > max_id;
                then {
                    max_id = id;
                }
            }
        }

        for mut note in self.notes.iter_mut() {
            if note.id.is_none() {
                note.id = Some(max_id + 1);
                max_id += 1;
            }
        }
    }
}

impl Template {
    pub async fn from_dir(path: &str) -> Result<Template> {
        let front = fs::read_to_string(format!("{}/front.html", path)).await
            .with_context(|| format!("Failed to read front template {}", path))?;

        let back = fs::read_to_string(format!("{}/back.html", path)).await
            .with_context(|| format!("Failed to read back template {}", path))?;

        let css = fs::read_to_string(format!("{}/style.css", path)).await
            .unwrap_or(String::new());

        Ok(Template { front, back, css })
    }

    pub fn to_model(&self, id: usize, name: &str) -> genanki_rs::Model {
        genanki_rs::Model::new_with_options(
            id,
            name,
            vec![
                genanki_rs::Field::new("ID"),
                genanki_rs::Field::new("Word"),
                genanki_rs::Field::new("Reading"),
                genanki_rs::Field::new("Definition"),
                genanki_rs::Field::new("Pronunciation"),
                genanki_rs::Field::new("Media"),
                genanki_rs::Field::new("Transcription"),
            ],
            vec![genanki_rs::Template::new("Default").qfmt(&self.front).afmt(&self.back)],
            Some(&self.css), None, None, None, Some(0),
        )
    }
}

impl Package {
    pub async fn new(deck: Deck, template: Template) -> Result<Package> {
        Ok(Package { deck, template })
    }

    pub async fn write(&mut self, dest: &str, media_dir: &str, static_dir: Option<&str>) -> Result<()> {
        let mut deck = genanki_rs::Deck::new(
            self.deck.id,
            &self.deck.name,
            &self.deck.description.as_ref().unwrap_or(&"".to_string())
        );

        let mut media_list: Vec<String> = Vec::new();

        for (index, note) in self.deck.notes.iter_mut().enumerate() {
            let reading = if_chain! {
                if !note.use_reading;
                if let Some(note_reading) = &note.reading;
                then { format!("{}[{}]", note.word, note_reading) }
                else {
                    if let Some(note_reading) = &note.reading {
                        note_reading.to_string()
                    } else {
                        String::new()
                    }
                }
            };

            let id = note.id.with_context(|| format!("Note {} has no ID", note.word))?;
            let filename = format!("{}{}", self.deck.id, id);
            let media_base_path = format!("{}/{}", media_dir, filename);
            let pronun_path = format!("{}{}", media_base_path, AUDIO_SUFFIX);
            let media_ext = file::find_extension(&media_base_path, &(*MEDIA_EXT));

            let media = if let Some(ext) = media_ext {
                media_list.push(format!("{}.{}", media_base_path, ext));
                format!("[sound:{}.{}]", filename, ext)
            } else {
                String::new()
            };

            let pronunciation = format!("[sound:{}{}]", filename, AUDIO_SUFFIX);
            let has_pronunciation: bool = is_audio_ok(&note.audio_state);

            if has_pronunciation {
                media_list.push(pronun_path);
            }
            
            let guid = format!("{}{}", filename, self.deck.id);

            deck.add_note(genanki_rs::Note::new_with_options(
                self.template.to_model(self.deck.id, &self.deck.name),
                vec![
                    &(index + 1).to_string(),
                    if note.use_reading { &reading } else { &note.word },
                    &reading,
                    &note.definition,
                    &pronunciation,
                    &media,
                    &note.transcription,
                ],
                None,
                None,
                Some(&guid)
            )?);
        }

        if_chain! {
            if let Some(static_dir) = static_dir;
            if Path::new(&static_dir).exists();
            then {
                let mut static_files = fs::read_dir(&static_dir).await?;
    
                while let Some(file) = static_files.next_entry().await? {
                    let path = file.path().into_os_string().into_string()
                        .map_err(|_| anyhow!("Cannot read static file: {}", file.path().display()))?;
                    media_list.push(path);
                }
            }
        }

        let dest = dest.to_string();

        let pkg_task: JoinHandle<Result<()>> = task::spawn_blocking(move || {
            let mut pkg = genanki_rs::Package::new(vec![deck], media_list.iter().map(|s| &**s).collect())?;
            Ok(pkg.write_to_file(&dest)?)
        });

        pkg_task.await
            .with_context(|| "Error while creating the package")? // task error
            .with_context(|| "Failed to write the package")?;     // file write error
        
        Ok(())
    }
    
    pub fn to_deck(self) -> Deck {
        return self.deck
    }
}

pub async fn fetch_all_audio(deck: &mut Deck, dest_dir: &str) -> Result<()> {
    for mut note in deck.notes.iter_mut() {
        let id = note.id.with_context(|| "ID must be defined")?;
        let audio_path = format!("{}/{}{}{}", dest_dir, deck.id, id, AUDIO_SUFFIX);

        if Path::new(&audio_path).exists() {
            note.audio_state = Some(AudioState::OK.to_string());
        } else if !is_audio_unavailable(&note.audio_state) {
            fetch_audio(&mut note, &audio_path).await;
        }
    }
    Ok(())
}

pub async fn fetch_audio(note: &mut Note, dest: &str) {
    if Path::new(dest).exists() {
        return;
    }

    let is_kanji = note.reading.is_some();
    let kanji = if is_kanji { Some(note.word.as_str()) } else { None };
    let kana = &note.reading.as_ref().unwrap_or(&note.word);
    let res = audio::fetch_audio(&dest, kanji, kana).await;

    match res {
        Ok(_) => {
            note.audio_state = Some(AudioState::OK.to_string());
        },
        Err(err) => {
            if_chain! {
                if let Ok(audio_err) = err.downcast::<AudioError>();
                if let AudioError::UnavailableError(_) = audio_err;
                then {
                    eprintln!("{}", audio_err);
                    note.audio_state = Some(AudioState::UNAVAILABLE.to_string());
                } else {
                    note.audio_state = Some(AudioState::RETRY.to_string());
                }
            };
        }
    }
}

pub fn is_valid_extension(ext: &str) -> bool {
    is_extension_in(ext, &MEDIA_EXT)
}

pub fn is_valid_audio_extension(ext: &str) -> bool {
    is_extension_in(ext, &AUDIO_EXT)
}

pub fn is_valid_video_extension(ext: &str) -> bool {
    is_extension_in(ext, &VIDEO_EXT)
}

pub fn is_valid_image_extension(ext: &str) -> bool {
    is_extension_in(ext, &IMAGE_EXT)
}

pub fn media_type(ext: &Option<&str>) -> MediaType {
    if ext.is_none() { return MediaType::INVALID }
    let ext = ext.unwrap();
    if is_valid_video_extension(ext) { MediaType::VIDEO }
    else if is_valid_image_extension(ext) { MediaType::IMAGE }
    else if is_valid_audio_extension(ext) { MediaType::AUDIO }
    else { MediaType::INVALID }
}

fn is_extension_in(ext: &str, list: &Lazy<Vec<&str>>) -> bool {
    for el in (*list).iter() {
        if *el == ext {
            return true;
        }
    }
    false
}

fn is_audio_ok(audio_state: &Option<String>) -> bool {
    is_audio(audio_state, AudioState::OK)
}

fn is_audio_unavailable(audio_state: &Option<String>) -> bool {
    is_audio(audio_state, AudioState::UNAVAILABLE)
}

fn is_audio(audio_state: &Option<String>, other: AudioState) -> bool {
    if audio_state.is_none() { return false; }
    audio_state.as_ref().unwrap() == other.as_ref()
}