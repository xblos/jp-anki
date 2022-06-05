use std::path::Path;
use tokio::fs;

use anyhow::Result;

pub fn find_extension<'a>(partial_path: &str, extensions: &Vec<&'a str>) -> Option<&'a str> {
    for ext in extensions {
        let filename = format!("{}.{}", partial_path, ext);
        let path = Path::new(&filename);
        if path.exists() {
            return Some(ext);
        }
    }

    None
}

pub async fn create_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    Ok(())
}