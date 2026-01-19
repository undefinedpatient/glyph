use std::error::Error;
use std::fs::DirEntry;
use std::path::{PathBuf, Path};
use std::{fs,io};

use color_eyre::eyre::Result;

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

pub fn create_glyph(path: &Path) -> Result<()> {
    fs::create_dir(path)?;
    fs::write(path.join("main.toml"), "Hi")?;
    Ok(())
}

pub fn is_glyph(path: &Path) -> Result<bool> {
    Ok(false)
}
// 20260119 
// - Add read/write json, set up basic structure of the Glyph e.g. a folder with correct toml file inside.
// - Make main screen with menu like blender e.g. New Glyph / Open Glyph files