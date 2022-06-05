use app::audio::{fetch_audio, AudioError};
use std::{path::Path, fs};

#[cfg(test)] #[macro_use]
extern crate assert_matches;

#[tokio::test]
async fn fetch_test() {
    let filename = "tests/test-files/audio-test/狂う.mp3";
    fetch_audio(filename, Some("狂う"), "くるう").await.unwrap();
    assert!(Path::new(&filename).exists());
    fs::remove_file(filename).unwrap();
}

#[tokio::test]
async fn fetch_invalid_audio() {
    let res = fetch_audio("tests/test-files/no.mp3", None, "いじる").await;
    let err: AudioError = res.unwrap_err().downcast().unwrap();
    assert_matches!(err, AudioError::UnavailableError(_));
}