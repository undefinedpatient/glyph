use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize)]
struct GlyphConfig {
    name: String,
}

pub fn get_file_names(path: &Path) -> Result<Vec<String>> {
    let mut file_names: Vec<String> = Vec::new();

    for entry_result in fs::read_dir(path)? {
        let entry: DirEntry = entry_result?;
        let path: PathBuf = entry.path();
        if let Some(file_name) = path.file_name() {
            if let Some(name) = file_name.to_str() {
                file_names.push(name.to_string());
            }
        }
    }

    Ok(file_names)
}

pub fn get_dir_names(path: &Path) -> Result<Vec<String>> {
    let mut file_names: Vec<String> = vec!["..".to_string()];

    for entry_result in fs::read_dir(path)? {
        let entry: DirEntry = entry_result?;
        let path: PathBuf = entry.path();
        if !path.is_dir() {
            continue;
        }
        if let Some(file_name) = path.file_name() {
            if let Some(name) = file_name.to_str() {
                file_names.push(name.to_string());
            }
        }
    }

    Ok(file_names)
}

pub fn create_glyph(path_buf: &PathBuf, glyph_name: &str) -> Result<()> {
    fs::create_dir(path_buf.join(glyph_name))?;
    let glyph_config: String = toml::to_string(&GlyphConfig {
        name: glyph_name.to_string(),
    })?;
    fs::write(path_buf.join(glyph_name).join("glyph.toml"), glyph_config)?;
    Ok(())
}

pub fn is_valid_glyph(path_buf: &PathBuf) -> Result<bool> {
    Ok(false)
}
