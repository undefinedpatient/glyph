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