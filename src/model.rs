use std::collections::HashMap;
use color_eyre::eyre::Result;
use rusqlite::{params, Connection, Row, Rows};
use std::path::PathBuf;

pub struct Entry {
    pub id: i64,
    pub entry_name: String,
    pub content: String

}
pub struct Glyph {
    pub glyph_name: String,
    pub entries: Vec<Entry>,
}

pub struct GlyphRepository {}

pub struct EntryRepository {}

impl GlyphRepository {
    pub fn init_glyph_db(path_to_db: &PathBuf) -> Result<Connection> {
        let mut c = Connection::open(path_to_db)?;
        c.execute(
            "
        CREATE TABLE IF NOT EXISTS entries (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT NOT NULL,
            content     TEXT NOT NULL DEFAULT ''
        )
        "
            , ())?;
        Ok(c)
    }
}

impl EntryRepository {
    pub fn create_entry(c: &Connection, title: &str, content: &str) -> Result<i64> {
        c.execute(
            "INSERT INTO entries (title, content) VALUES (?1, ?2)",
            (title, content)
        )?;
        let id = c.last_insert_rowid();
        return Ok(id);
    }

    pub fn read_by_id(c: &Connection, id: i64) -> Result<Option<Entry>> {
        let mut stmt = c.prepare("SELECT id, title, content FROM entries WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![id])?;
        rows.next()?.map(|row| {Self::map_row(row)}).transpose()
    }

    pub fn read_all(c: &Connection) -> Result<Vec<Entry>> {
        let mut stmt = c.prepare("SELECT id, title, content FROM entries")?;
        let mut rows: Rows = stmt.query(params![])?;
        let mut entries: Vec<Entry> = Vec::new();
        while let Some(row) = rows.next()? {
            entries.push(Self::map_row(row)?)
        }
        Ok(entries)
    }
    pub fn read_all_hashed(c: &Connection) -> Result<HashMap<i64, Entry>> {
        let mut stmt = c.prepare("SELECT id, title, content FROM entries")?;
        let mut rows: Rows = stmt.query(params![])?;
        let mut entries: HashMap<i64, Entry> = HashMap::new();
        while let Some(row) = rows.next()? {
            let entry = Self::map_row(row)?;
            entries.insert(entry.id, entry);
        }
        Ok(entries)
    }

    fn map_row(row: &Row) -> Result<Entry> {
        Ok(
            Entry {
                id: row.get(0)?,
                entry_name: row.get(1)?,
                content: row.get(2)?
            }
        )
    }
}