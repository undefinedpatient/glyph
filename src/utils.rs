use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io::Error;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use rusqlite::Connection;
use crate::model::{Entry, Glyph};


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

pub fn init_glyph_db(path_to_db: &PathBuf) -> Result<Connection> {
    let mut c = Connection::open(path_to_db)?;
    c.execute_batch(
    "
            CREATE TABLE IF NOT EXISTS entries (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT NOT NULL COLLATE NOCASE,
            content     TEXT NOT NULL DEFAULT '',
            created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
            updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
        );

        END;

        "
    )?;
    Ok(c)
}
pub fn create_entry(c: &Connection, title: &str, content: &str) -> Result<i64> {
    c.execute(
        "INSERT INTO entries (title, content) VALUES (?1, ?2)",
        (title, content)
    )?;
    let id = c.last_insert_rowid();
    return Ok(id);
}

pub fn load_entry(path_buf: &PathBuf) -> Entry {
    todo!()
}

pub fn load_glyph(path_buf: &PathBuf) -> Glyph {
    todo!()
}
