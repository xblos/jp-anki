use anyhow::{Result, Context};

use crate::ext::char::CharExt;

/* Adapted from: https://github.com/zacharied/autofurigana */
pub fn generate_furigana(kanji_str: &str, kana_str: &str) -> Result<Vec<Vec<String>>> {
    let mut pairs: Vec<Vec<String>> = Vec::new();

    let mut kanji_buf: Vec<char> = Vec::new();
    let mut kana_buf: Vec<char> = Vec::new();

    let kanji_vec: Vec<char> = kanji_str.chars().collect();
    let kana_vec: Vec<char> = kana_str.chars().collect();

    let mut j = 0;

    for i in 0..kanji_vec.len() {
        kanji_buf.push(kanji_vec[i]);

        if i == kanji_vec.len() - 1 {
            if kanji_vec[i].is_kanji() {
                while j < kana_vec.len() {
                    kana_buf.push(kana_vec[j]);
                    j += 1;
                }
            } else {
                kana_buf.clear();
            }
            add_pair(&mut pairs, &mut kanji_buf, &mut kana_buf);
        } else if kanji_vec[i].is_kanji() && !kanji_vec[i + 1].is_kanji() {
            loop {
                let kana = kana_vec.get(j).with_context(|| "Kanji and Kana sentences do not match")?;
                if *kana == kanji_vec[i + 1] && kana_buf.len() >= kanji_buf.len() {
                    break;
                }
                kana_buf.push(*kana);
                j += 1;
            }
            add_pair(&mut pairs, &mut kanji_buf, &mut kana_buf);
        } else if !kanji_vec[i].is_kanji() && kanji_vec[i + 1].is_kanji() {
            kana_buf.clear();
            j += kanji_buf.len();
            add_pair(&mut pairs, &mut kanji_buf, &mut kana_buf);
        }
    }

    Ok(pairs)
}

pub fn generate_ruby_string(kanji_str: &str, kana_str: &str) -> Result<String> {
    let pairs = generate_furigana(kanji_str, kana_str)?;
    let mut s = String::new();

    for i in 0..pairs.len() {
        if !pairs[i][1].is_empty() {
            if i != 0 {
                s.push(' ');
            }
            s.push_str(&format!("{}[{}]", pairs[i][0], pairs[i][1]));
        } else {
            s.push_str(&pairs[i][0]);
        }
    }

    Ok(s)
}

fn add_pair(pairs: &mut Vec<Vec<String>>, kanji_buf: &mut Vec<char>, kana_buf: &mut Vec<char>) {
    pairs.push(vec![kanji_buf.iter().collect(), kana_buf.iter().collect()]);
    kanji_buf.clear();
    kana_buf.clear();
}