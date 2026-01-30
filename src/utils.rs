use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use rusqlite::Connection;
use crate::model::{Entry, EntryRepository};

pub fn cycle_add(value:u16, offset: u16, max: u16) -> u16 {
    return ((value as u32 + offset as u32)%max as u32) as u16;
}
pub fn cycle_sub(value:u16, offset: u16, max: u16) -> u16 {
    let offset = offset % max;
    return if offset > value {
        max - (offset - value)
    } else {
        value - offset
    }
}
pub fn cycle_offset(value:u16, offset: i16, max: u16) -> u16 {
    if offset.is_negative() {
        cycle_sub(value, offset.abs() as u16, max)
    } else {
        cycle_add(value, offset as u16, max)
    }
}
pub fn get_file_names(path: &Path) -> Result<Vec<String>> {
    let mut file_names: Vec<String> = Vec::new();

    for entry_result in fs::read_dir(path)? {
        let entry: DirEntry = entry_result?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
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


