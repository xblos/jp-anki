use anyhow::{bail, Result};
use bytes::Bytes;
use hex_literal::hex;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, Cursor},
    path::Path,
};
use thiserror::Error;

use crate::file::create_parent_dir;

static URL: &str =
    "https://assets.languagepod101.com/dictionary/japanese/audiomp3.php?";

const INVALID_AUDIO_HASH: [u8; 32] =
    hex!("ae6398b5a27bc8c0a771df6c907ade794be15518174773c58c7c7ddd17098906");

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Kana cannot be empty")]
    KanaError,
    #[error("Audio not available for {0}")]
    UnavailableError(String),
    #[error("The given path is not valid: {0}")]
    PathError(String),
}

pub async fn fetch_audio(dest: &str, opt_kanji: Option<&str>, kana: &str) -> Result<()> {
    if kana.is_empty() {
        bail!(AudioError::KanaError);
    }

    let url = format!(
        "{}kanji={}&kana={}",
        URL,
        opt_kanji.unwrap_or(""),
        kana,
    );

    create_parent_dir(&Path::new(dest)).await?;

    let res = reqwest::get(url).await?;
    let bytes = res.bytes().await?;

    if is_audio_invalid(&bytes) {
        let err = (opt_kanji.unwrap_or(kana)).to_string();
        bail!(AudioError::UnavailableError(err));
    }

    let mut file = File::create(dest)?;
    let mut content = Cursor::new(bytes);
    io::copy(&mut content, &mut file)?;

    Ok(())
}

fn is_audio_invalid(bytes: &Bytes) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(bytes.to_vec());
    let hash = hasher.finalize();
    hash[..] == INVALID_AUDIO_HASH
}
