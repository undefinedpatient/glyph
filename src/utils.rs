use color_eyre::eyre::Result;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

pub mod markdown_renderer;

pub fn cycle_add(value:u16, offset: u16, max: u16) -> u16 {
    if max == 0 {
        return 0;
    }
    return ((value as u32 + offset as u32)%max as u32) as u16;
}
pub fn cycle_sub(value:u16, offset: u16, max: u16) -> u16 {
    if max == 0 {
        return 0;
    }
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

/// Covert from integer number (0-3999] to roman numeral representation.
pub fn number_to_roman(mut num: u16) -> String {
    if num > 3999 || num == 0 {
        return "E".to_string();
    }
    let mut roman: Vec<String> = Vec::new();
    while num > 0 {
         if num / 10 == 0 {
            match num {
                1 => roman.push("I".to_string()),
                2 => roman.push("II".to_string()),
                3 => roman.push("III".to_string()),
                4 => roman.push("IV".to_string()),
                5 => roman.push("V".to_string()),
                6 => roman.push("VI".to_string()),
                7 => roman.push("VII".to_string()),
                8 => roman.push("VIII".to_string()),
                9 => roman.push("IX".to_string()),
                _ => return "E".to_string(),
            }
        }
        else if num / 100 == 0 {
            match num / 10 {
                1 => roman.push("X".to_string()),
                2 => roman.push("XX".to_string()),
                3 => roman.push("XXX".to_string()),
                4 => roman.push("XL".to_string()),
                5 => roman.push("L".to_string()),
                6 => roman.push("LX".to_string()),
                7 => roman.push("LXX".to_string()),
                8 => roman.push("LXXX".to_string()),
                9 => roman.push("XC".to_string()),
                _ => return "E".to_string(),
            }
        } else if num / 100 == 0 {
            match num/100 {
                1 => roman.push("C".to_string()),
                2 => roman.push("CC".to_string()),
                3 => roman.push("CCC".to_string()),
                4 => roman.push("CD".to_string()),
                5 => roman.push("D".to_string()),
                6 => roman.push("DC".to_string()),
                7 => roman.push("DCC".to_string()),
                8 => roman.push("DCCC".to_string()),
                9 => roman.push("CM".to_string()),
                _ => return "E".to_string(),
            }
        } else {
            match num/1000 {
                1 => roman.push("M".to_string()),
                2 => roman.push("MM".to_string()),
                3 => roman.push("MMM".to_string()),
                _ => {
                    return "E".to_string();
                }
            }
        }
        num /= 10;
    }
    roman.join("")
}