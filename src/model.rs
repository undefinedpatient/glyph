use std::collections::HashMap;
use color_eyre::eyre::Result;
use rusqlite::{params, Connection, Row, Rows, ToSql};
use std::path::PathBuf;
use color_eyre::Report;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

pub struct LocalEntryState {
    pub active_entry_id: Option<i64>,
    pub entries: HashMap<i64, Entry>,
    pub updated_entry_ids:  Vec<i64>,
}
impl LocalEntryState {
    pub fn new(c: &Connection) -> Self {
        Self {
            active_entry_id: None,
            entries: EntryRepository::read_all(c).unwrap(),
            updated_entry_ids: Vec::new(),
        }
    }
}

pub struct Glyph {
    pub glyph_name: String,
    pub entries: Vec<Entry>,
}


pub struct Entry {
    pub entry_name: String,
    pub sections: HashMap<i64, Section>,
    pub layout: (i64, Layout),
}
pub struct Section {
    // pub entry_id: i64,
    pub position: i64,
    pub title: String,
    pub content: String,
}

impl Section {
    pub fn new(title: &str, default: &str) -> Self {
        Self {
            position: 0,
            title: title.to_string(),
            content: default.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Layout {
    pub label: String,
    pub section_index: Option<u16>,
    pub sub_layouts: Vec<Layout>,
    pub details: SubLayoutDetails
}

impl Layout {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            section_index: None,
            sub_layouts: Vec::new(),
            details: SubLayoutDetails::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SubLayoutDetails {
    spacing: u16,
    padding: u16,
}

impl SubLayoutDetails {
    pub fn new() -> Self {
        Self {
            spacing: 0,
            padding: 0,
        }
    }
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
            entry_name  TEXT NOT NULL
        )
        "
            , ())?;
        c.execute(
            "
        CREATE TABLE IF NOT EXISTS sections (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            entry_id    INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
            position    INTEGER NOT NULL,
            title       TEXT NOT NULL DEFAULT '',
            content     TEXT NOT NULL DEFAULT '',

            UNIQUE(entry_id, position)
        )
        "
            , ())?;
        c.execute(
            "
        CREATE TABLE IF NOT EXISTS layouts (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            entry_id    INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
            content     TEXT NOT NULL DEFAULT ''
        )
        "
            , ())?;
        Ok(c)
    }
}

impl EntryRepository {
    pub fn create_entry(c: &Connection, entry_name: String) -> Result<i64> {
        c.execute(
            "INSERT INTO entries (entry_name) VALUES (?1)",
            params![entry_name],
        )?;
        let id: i64 = c.last_insert_rowid();
        c.execute(
            "INSERT INTO layouts (entry_id, content) VALUES (?1, ?2)",
            params![id,  serde_json::to_string(&Layout::new())?],
        )?;
        return Ok(id);
    }
    pub fn update_entry(c: &Connection, eid: &i64, entry: &Entry) -> Result<(i64)> {
        c.execute(
            "
                INSERT INTO entries (id, entry_name)
                VALUES (?1, ?2)
                ON CONFLICT (?1)
                DO UPDATE SET
                    entry_name = ?2,
            ",
            params![eid, entry.entry_name],
        )?;
        let id: i64 = c.last_insert_rowid();
        for (sid, section) in &entry.sections {
            SectionRepository::update_section(c, eid, sid, section)?;
        }
        LayoutRepository::update_layout(c, eid, &entry.layout.0, &entry.layout.1)?;
        return Ok(id);
    }

    pub fn read_by_id(c: &Connection, id: &i64) -> Result<Option<(i64, Entry)>> {
        let mut stmt = c.prepare("SELECT id, entry_name FROM entries WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![*id])?;
        rows.next()?.map(|row| {Self::to_entry(c, row)}).transpose()
    }

    pub fn read_all(c: &Connection) -> Result<HashMap<i64, Entry>> {
        let mut stmt = c.prepare("SELECT id, entry_name FROM entries")?;
        let mut rows: Rows = stmt.query(params![])?;
        let mut entries: HashMap<i64, Entry> = HashMap::new();
        while let Some(row) = rows.next()? {
            let entry = Self::to_entry(c, row)?;
            entries.insert(entry.0, entry.1);
        }
        Ok(entries)
    }

    fn to_entry(c: &Connection, row: &Row) -> Result<(i64, Entry)> {
        let id: i64 = row.get(0)?;
        Ok((
            id,
            Entry {
                entry_name: row.get(1)?,
                sections: SectionRepository::read_by_entry_id(c, &id)?,
                layout: LayoutRepository::read_by_entry_id(c, &id)?
            }
        )
        )
    }
}
pub struct SectionRepository {}
impl SectionRepository {
    pub fn read_by_entry_id(c: &Connection, entry_id: &i64) -> Result<HashMap<i64, Section>> {
        let mut stmt = c.prepare("SELECT id, entry_id, position, title, content FROM sections WHERE entry_id = ?1")?;
        let mut rows: Rows = stmt.query(params![*entry_id])?;
        let mut sections: HashMap<i64, Section> = HashMap::new();
        while let Some(row) = rows.next()? {
            let section = Self::to_section(row)?;
            sections.insert(section.0, section.1);
        }
        Ok(sections)
    }
    pub fn create_section(c: &Connection, eid:&i64, section: &Section) -> Result<i64> {
        c.execute(
            "
                INSERT INTO sections (entry_id, position, title, content) VALUES (?1, ?2, ?3, ?4)
            ",
            params![eid, section.position, section.title, section.content],
        )?;
        let id = c.last_insert_rowid();
        return Ok(id);
    }
    pub fn update_section(c: &Connection, eid: &i64, sid: &i64, section: &Section) -> Result<i64> {
        c.execute(
            "
                INSERT INTO sections (id, entry_id, position, title, content)
                VALUES (?1, ?2, ?3, ?4, ?5)
                ON CONFLICT (?1)
                DO UPDATE SET
                    entry_id = ?2,
                    position = ?3,
                    title = ?4,
                    content = ?5
            ",
            params![sid, eid, section.position, section.title, section.content],
        )?;
        let id = c.last_insert_rowid();
        return Ok(id);

    }
    pub fn read_by_id(c: &Connection, id: &i64) -> Result<Option<(i64, Section)>> {
        let mut stmt = c.prepare("SELECT id, entry_id, position, title, content FROM sections WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![*id])?;
        rows.next()?.map(|row| {Self::to_section(row)}).transpose()
    }
    pub fn to_section(row: &Row) -> Result<(i64, Section)> {
        let id: i64 = row.get(0)?;
        Ok(
            (
                id,
                Section {
                    // entry_id: row.get(1)?,
                    position: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                }
            )
        )
    }
}

pub struct LayoutRepository {}
impl LayoutRepository {
    pub fn read_by_entry_id(c: &Connection, entry_id: &i64) -> Result<(i64, Layout)> {
        let mut stmt = c.prepare("SELECT id, entry_id, content FROM layouts WHERE entry_id = ?1")?;
        let mut rows: Rows = stmt.query(params![*entry_id])?;

        match rows.next() {
            Ok(row) => {
                if let Some(row) = row {
                    Self::to_layout(row)
                } else {
                    Err(Report::msg("Layout does not exists!"))
                }
            }
            Err(e) => {
                Err(Report::msg("Failed to read layout"))
            }
        }

    }
    pub fn update_layout(c: &Connection, eid: &i64, lid: &i64, layout: &Layout) -> Result<i64> {
        c.execute(
            "
                INSERT INTO layouts (id, entry_id, content)
                VALUES (?1, ?2, ?3)
                ON CONFLICT (?1)
                DO UPDATE SET
                    entry_id = ?2,
                    content = ?3,
            ",
            params![lid, eid, serde_json::to_string(layout).unwrap()],
        )?;
        let id = c.last_insert_rowid();
        return Ok(id);
    }
    pub fn to_layout(row: &Row) -> Result<(i64, Layout)> {
        let id: i64 = row.get(0)?;
        let layout_data: String = row.get(2)?;
        Ok(
            (
                id,
                serde_json::from_str(layout_data.as_str())?,
            )
        )
    }
}
