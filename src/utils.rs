use std::error::Error;
use std::fs::DirEntry;
use std::path::{PathBuf, Path};
use std::{fs,io};
use serde::{Deserialize, Serialize};
use toml;
use color_eyre::eyre::Result;

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

pub fn create_glyph(path: &Path, glyph_name: &str) -> Result<()> {
    fs::create_dir(path)?;
    let glyph_config: String = toml::to_string(&GlyphConfig {
        name: glyph_name.to_string(),
    })?;
    fs::write(path.join("glyph.toml"), glyph_config)?;
    Ok(())
}

pub fn is_glyph(path: &Path) -> Result<bool> {
    Ok(false)
}