use std::collections::HashMap;
use color_eyre::eyre::Result;
use rusqlite::{params, Connection, Row, Rows, ToSql};
use std::path::PathBuf;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};

pub struct LocalEntryState {
    pub active_entry_id: Option<i64>,
    pub entries: HashMap<i64, Entry>,
    pub updated_entry_ids:  Vec<i64>,
}
impl LocalEntryState {
    pub fn new(c: &Connection) -> Self {
        Self {
            active_entry_id: None,
            entries: EntryRepository::read_all(c).unwrap_or(HashMap::new()),
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
    pub entry_type: EntryType,
    pub sections: HashMap<i64, Section>
}
impl Entry {
    pub fn new(entry_name: String, entry_type: EntryType) -> Self {
        Self {
            entry_name: entry_name,
            entry_type: entry_type,
            sections: HashMap::new()
        }
    }
}
pub enum EntryType {
    Default
}
impl EntryType {
    pub fn to_string(&self) -> String {
        match self {
            EntryType::Default => String::from("default"),
            _ => String::from("default"),
        }
    }
    pub fn from_string(string: &str) -> EntryType {
        match string {
            "default" => EntryType::Default,
            _ => EntryType::Default,
        }
    }
}
impl FromSql for EntryType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::from(value.as_str().map(Self::from_string))
    }
}
impl ToSql for EntryType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            EntryType::Default => Ok(ToSqlOutput::from("default")),
        }
    }
}
pub struct Section {
    pub entry_id: i64,
    pub position: i64,
    pub title: String,
    pub content: String,
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
            entry_name  TEXT NOT NULL,
            entry_type  TEXT NOT NULL DEFAULT 'default'
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

            UNIQUE(position)
        )
        "
            , ())?;
        Ok(c)
    }
}

impl EntryRepository {
    pub fn insert_entry(c: &Connection, entry: Entry) -> Result<i64> {
        c.execute(
            "INSERT INTO entries (entry_name, entry_type) VALUES (?1, ?2)",
            (entry.entry_name, entry.entry_type),
        )?;
        let id = c.last_insert_rowid();
        return Ok(id);
    }

    pub fn read_by_id(c: &Connection, id: i64) -> Result<Option<(i64, Entry)>> {
        let mut stmt = c.prepare("SELECT id, entry_name, entry_type FROM entries WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![id])?;
        rows.next()?.map(|row| {Self::to_entry(c, row)}).transpose()
    }

    pub fn read_all(c: &Connection) -> Result<HashMap<i64, Entry>> {
        let mut stmt = c.prepare("SELECT id, entry_name, entry_type FROM entries")?;
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
                entry_type: row.get(2)?,
                sections: SectionRepository::read_by_entry_id(c, id)?
            }
        )
        )
    }
}
pub struct SectionRepository {}
impl SectionRepository {
    pub fn read_by_entry_id(c: &Connection, entry_id: i64) -> Result<HashMap<i64, Section>> {
        let mut stmt = c.prepare("SELECT id, entry_id, position, title, content FROM sections WHERE entry_id = ?1")?;
        let mut rows: Rows = stmt.query(params![entry_id])?;
        let mut sections: HashMap<i64, Section> = HashMap::new();
        while let Some(row) = rows.next()? {
            let section = Self::to_section(row)?;
            sections.insert(section.0, section.1);
        }
        Ok(sections)

    }
    pub fn to_section(row: &Row) -> Result<(i64, Section)> {
        let id: i64 = row.get(0)?;
        Ok(
            (
                id,
                Section {
                    entry_id: row.get(1)?,
                    position: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                }
            )
        )
    }
}