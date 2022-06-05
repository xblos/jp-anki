#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;

use anyhow::Context;
use app::{
    deck::{Deck, Package, Template, fetch_all_audio, MEDIA_EXT, AUDIO_SUFFIX, is_valid_extension},
    file::{create_parent_dir, find_extension},
};
use tokio::fs;

macro_rules! catch {
    ($a:expr) => {
        $a.map_err(|e| format!("{:#}", e))?
    };
}

#[tauri::command]
async fn open_deck(dir: String) -> Result<String, String> {
    if !Path::new(&dir).is_dir() {
        return Err(format!("{} is not a valid directory", dir))
    }
    let deck = catch!(Deck::from_file(&deck_path(&dir)).await);
    Ok(catch!(deck.to_json()))
}

#[tauri::command]
async fn restore_backup(dir: String) -> Result<String, String> {
    let backup = backup_path(&dir);
    if !Path::new(&backup).exists() {
        return Err("Backup file not found".to_string());
    }
    let deck = catch!(Deck::from_file(&backup_path(&dir)).await);
    Ok(catch!(deck.to_json()))
}

#[tauri::command]
async fn write_deck(dir: String, json: String) -> Result<(), String> {
    let deck = catch!(Deck::from_json(&json));
    catch!(deck.write(&deck_path(&dir)).await);
    Ok(())
}

#[tauri::command]
async fn write_backup(dir: String, json: String) -> Result<(), String> {
    catch!(fs::write(&backup_path(&dir), json).await);
    Ok(())
}

#[tauri::command]
async fn fetch_audio(dir: String, json: String) -> Result<String, String> {
    let mut deck = catch!(Deck::from_json(&json));
    catch!(fetch_all_audio(&mut deck, &media_path(&dir)).await);
    Ok(catch!(deck.to_json()))
}

#[tauri::command]
async fn write_apkg(dest: String, dir: String, json: String) -> Result<(), String> {
    let template = catch!(Template::from_dir(&dir).await);
    let deck = catch!(Deck::from_json(&json));
    catch!(deck.write(&deck_path(&dir)).await);
    let mut pkg = catch!(Package::new(deck, template).await);
    catch!(pkg.write(&dest, &media_path(&dir), Some(&static_path(&dir))).await);
    Ok(())
}

#[tauri::command]
async fn check_template(dir: String) -> Result<bool, ()> {
    Ok(
        Path::new(&front_path(&dir)).exists() &&
        Path::new(&back_path(&dir)).exists() &&
        Path::new(&css_path(&dir)).exists()
    )
}

#[tauri::command]
async fn write_template(dir: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let resolver = app_handle.path_resolver();
    let front_res = resolver.resolve_resource("res/front.html").unwrap();
    let back_res = resolver.resolve_resource("res/back.html").unwrap();
    let css_res = resolver.resolve_resource("res/style.css").unwrap();
    let font_res = resolver.resolve_resource("res/_jpaf.ttf").unwrap();
    let front_dest = front_path(&dir);
    let back_dest = back_path(&dir);
    let css_dest = css_path(&dir);
    let font_dest = format!("{}/_jpaf.ttf", static_path(&dir));

    if Path::new(&front_dest).exists() {
        catch!(fs::copy(&front_dest, format!("{}.bk", front_dest)).await);
    }
    if Path::new(&back_dest).exists() {
        catch!(fs::copy(&front_dest, format!("{}.bk", back_path(&dir))).await);
    }
    if Path::new(&css_dest).exists() {
        catch!(fs::copy(&front_dest, format!("{}.bk", css_path(&dir))).await);
    }

    catch!(create_parent_dir(Path::new(&font_dest)).await);

    let front = catch!(fs::read(front_res).await);
    let back = catch!(fs::read(back_res).await);
    let css = catch!(fs::read(css_res).await);
    let font = catch!(fs::read(font_res).await);

    catch!(fs::write(front_dest, front).await);
    catch!(fs::write(back_dest, back).await);
    catch!(fs::write(css_dest, css).await);
    catch!(fs::write(font_dest, font).await);

    Ok(())
}

#[tauri::command]
async fn move_media(dest_dir: String, src_file: String, deck_id: usize, note_id: u16) -> Result<(), String> {
    let ext = catch!(Path::new(&src_file)
        .extension()
        .and_then(|it| it.to_str())
        .with_context(|| format!("Failed to parse file extension: {}", src_file)));

    if !is_valid_extension(ext) {
        return Err(format!("{} is not supported ({})", ext, src_file));
    }

    let dest = format!("{}/{}{}.{}", media_path(&dest_dir), deck_id, note_id, ext);
    let dest_path = Path::new(&dest);

    if dest_path.exists() {
        catch!(fs::remove_file(dest_path).await);
    } else {
        catch!(create_parent_dir(&dest_path).await);
    }

    catch!(fs::rename(src_file, dest).await);

    Ok(())
}

#[tauri::command]
async fn upgrade_media_naming(dir: String, json: String) -> Result<(), String> {
    let deck = catch!(Deck::from_json(&json));

    let media = media_path(&dir);

    catch!(fs::create_dir_all(&media).await);

    for note in deck.notes.iter() {
        let opt_ext = find_extension(&format!("{}/{}", media, note.word), &(*MEDIA_EXT));
        let id = catch!(note.id.with_context(|| "Note ID must be defined"));
        if let Some(ext) = opt_ext {
            let src = format!("{}/{}.{}", media, note.word, ext);
            move_media(dir.to_string(), src, deck.id, id).await?;
        }
        let audio_src = format!("{}/{}_r.mp3", media, note.word);
        if Path::new(&audio_src).exists() {
            catch!(fs::rename(audio_src, format!("{}/{}{}{}", media, deck.id, id, AUDIO_SUFFIX)).await);
        }
    }

    Ok(())
}

fn deck_path(dir: &str) -> String {
    format!("{}/deck.json", dir)
}

fn backup_path(dir: &str) -> String {
    format!("{}/deck.json.bk", dir)
}

fn media_path(dir: &str) -> String {
    format!("{}/media", dir)
}

fn static_path(dir: &str) -> String {
    format!("{}/static", dir)
}

fn front_path(dir: &str) -> String {
    format!("{}/front.html", dir)
}

fn back_path(dir: &str) -> String {
    format!("{}/back.html", dir)
}

fn css_path(dir: &str) -> String {
    format!("{}/style.css", dir)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_deck,
            restore_backup,
            write_deck,
            write_backup,
            write_apkg,
            check_template,
            write_template,
            move_media,
            fetch_audio,
            upgrade_media_naming,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
