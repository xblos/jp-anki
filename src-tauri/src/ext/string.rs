use anyhow::{Context, Result};

pub trait StringExt {
    fn extract(&self, start_tag: &str, end_tag: Option<&str>) -> Result<&str>;
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
}

fn tag_err(tag: &str) -> String {
    format!("{} tag not found", tag)
}
