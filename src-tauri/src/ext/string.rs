use anyhow::{Context, Result};

use super::char::CharExt;

pub trait StringExt {
    fn extract(&self, start_tag: &str, end_tag: Option<&str>) -> Result<&str>;
    fn contains_ruby(&self) -> bool;
    fn remove_ruby(&self) -> String;
    fn format_ruby(&self) -> String;
}

impl StringExt for String {
    fn extract(&self, start_delim: &str, end_delim: Option<&str>) -> Result<&str> {
        let start_index = self
            .find(start_delim)
            .map(|i| i + start_delim.len())
            .with_context(|| tag_err(start_delim))?;

        let end_index = self
            .get(start_index + 1..)
            .and_then(|s| s.find(end_delim.unwrap_or(start_delim)))
            .map(|i| i + start_index + 1)
            .with_context(|| tag_err(end_delim.unwrap_or(start_delim)))?;

        let ss = self
            .get(start_index..end_index)
            .with_context(|| {
                format!(
                    "Failed to parse delimiter {}. Computed indices: start({}), end({})",
                    start_delim, start_index, end_index
                )
            })?.trim();

        Ok(ss)
    }

    fn contains_ruby(&self) -> bool {
        for c in self.chars() {
            if c == '「' || c == '[' {
                return true;
            }
        }
        false
    }

    fn remove_ruby(&self) -> String {
        let mut open = false;
        let mut buf = String::new();

        for c in self.chars() {
            if c == '「' || c == '[' {
                open = true;
            } else if c == '」' || c == ']' {
                open = false;
            } else if !open {
                buf.push(c);
            }
        }

        buf
    }

    fn format_ruby(&self) -> String {
        let mut buf = String::new();
        let mut separate = true;

        for c in self.chars() {
            if separate && c.is_kanji() {
                buf.push(' ');
                separate = false;
            } else if c.is_closed_ruby_parenthesis() {
                separate = true;
            }
            match c {
                '「' => buf.push('['),
                '」' => buf.push(']'),
                _ => buf.push(c)
            }
        }

        if buf.starts_with(' ') {
            buf.remove(0);
        }

        buf
    }
}

fn tag_err(tag: &str) -> String {
    format!("{} tag not found", tag)
}
