use std::path::PathBuf;
use color_eyre::Report;
use rusqlite::{params, Connection, Row, Rows, Statement};
use crate::models::entry::Entry;
use crate::models::layout::Layout;
use crate::models::section::Section;

pub struct GlyphRepository {}

pub(crate) struct EntryRepository {}

impl GlyphRepository {
    pub fn init_glyph_db(path_to_db: &PathBuf) -> color_eyre::Result<Connection> {
        let mut c = Connection::open(path_to_db)?;
        c.execute(
            "
        CREATE TABLE IF NOT EXISTS entries (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            entry_name  TEXT NOT NULL,
            layout      TEXT NOT NULL DEFAULT ''
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
            content     TEXT NOT NULL DEFAULT ''

        )
        "
            // UNIQUE (entry_id, position)
            , ())?;
        Ok(c)
    }
}

impl EntryRepository {
    pub fn create_default_entry(c: &Connection, entry_name: &str) -> color_eyre::Result<i64> {
        c.execute(
            "INSERT INTO entries (entry_name, layout) VALUES (?1, ?2)",
            params![entry_name, serde_json::to_string(&Layout::new("Root"))?],
        )?;
        let eid: i64 = c.last_insert_rowid();
        return Ok(eid);
    }
    pub fn update_entry(c: &Connection, eid: &i64, entry: &Entry) -> color_eyre::Result<(i64)> {
        c.execute(
            "
                UPDATE entries
                SET
                    entry_name = ?2,
                    layout = ?3
                WHERE id = ?1
            ",
            params![eid, entry.entry_name, serde_json::to_string(&entry.layout)?],
        )?;
        let id: i64 = c.last_insert_rowid();
        for (sid, section) in &entry.sections {
            SectionRepository::update_section(c, sid, section)?;
        }

        return Ok(id);
    }
    pub fn update_name(c: &Connection, eid: &i64, new_name: &str) -> color_eyre::Result<()> {
        if c.execute(
            "
                UPDATE entries
                SET
                    entry_name = ?2
                WHERE id = ?1
            ",
            params![eid, new_name],
        )? != 1 {
            return Err(Report::msg("Tried to update name but there is no entry"));
        }
        return Ok(());
    }

    pub fn delete_by_id(c: &Connection, eid: &i64) -> color_eyre::Result<usize> {
        let num_of_row_deleted = c.execute( "DELETE FROM entries WHERE id = ?1",
                                            params![eid],
        )?;
        Ok(num_of_row_deleted)
    }

    pub fn read_by_id(c: &Connection, id: &i64) -> color_eyre::Result<(i64, Entry)> {
        let mut stmt = c.prepare("SELECT id, entry_name, layout FROM entries WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![*id])?;
        Self::map_row_to_eid_entry(c, rows.next()?.unwrap())
    }

    pub fn read_all(c: &Connection) -> color_eyre::Result<Vec<(i64, Entry)>> {
        let mut stmt: Statement = c.prepare("SELECT id, entry_name, layout FROM entries")?;
        let mut rows: Rows = stmt.query(params![])?;
        let mut entries: Vec<(i64, Entry)> = Vec::new();
        while let Some(row) = rows.next()? {
            let entry = Self::map_row_to_eid_entry(c, row)?;
            entries.push(entry);
        }
        Ok(entries)
    }


    // Expecting row (id, entry_name, layout)
    fn map_row_to_eid_entry(c: &Connection, row: &Row) -> color_eyre::Result<(i64, Entry)> {
        let id: i64 = row.get(0)?;
        let layout_string: String = row.get(2)?;
        Ok((
            id,
            Entry {
                entry_name: row.get(1)?,
                sections: SectionRepository::read_by_entry_id(c, &id)?,
                layout: serde_json::from_str(layout_string.as_str()).unwrap_or(Layout::new("")),
            }
        )
        )
    }
}
pub(crate) struct SectionRepository {}
impl SectionRepository {
    pub fn read_by_entry_id(c: &Connection, entry_id: &i64) -> color_eyre::Result<Vec<(i64, Section)>> {
        let mut stmt = c.prepare("SELECT id, entry_id, position, title, content FROM sections WHERE entry_id = ?1 ORDER BY position ASC")?;
        let mut rows: Rows = stmt.query(params![*entry_id])?;
        let mut sections: Vec<(i64, Section)> = Vec::new();
        while let Some(row) = rows.next()? {
            let section = Self::map_row_to_id_section(row)?;
            sections.push((section.1, section.2));
        }
        Ok(sections)
    }
    pub fn create_section(c: &Connection, eid:&i64, section: &Section) -> color_eyre::Result<i64> {
        c.execute(
            "
                INSERT INTO sections (entry_id, position, title, content) VALUES (?1, ?2, ?3, ?4)
            ",
            params![eid, section.position, section.title, section.content],
        )?;
        let id = c.last_insert_rowid();
        return Ok(id);
    }
    pub fn update_name(c: &Connection, sid: &i64, new_name: &str) -> color_eyre::Result<()> {
        if c.execute(
            "
                UPDATE sections
                SET
                    title = ?2
                WHERE id = ?1
            ",
            params![sid, new_name],
        )? != 1 {
            return Err(Report::msg("Tried to update name but there is no entry"));
        }
        return Ok(());
    }
    pub fn update_section(c: &Connection, sid: &i64, section: &Section) -> color_eyre::Result<i64> {
        c.execute(
            "
                UPDATE sections
                SET
                    position = ?2,
                    title = ?3,
                    content = ?4
                WHERE id = ?1
            ",
            params![sid, section.position, section.title, section.content],
        )?;
        let id = c.last_insert_rowid();
        return Ok(id);
    }

    pub fn delete_section(c: &Connection, sid: &i64) -> color_eyre::Result<usize> {
        let num_of_row_deleted = c.execute( "DELETE FROM sections WHERE id = ?1",
                                            params![sid],
        )?;


        Ok(num_of_row_deleted)
    }

    /*
        Return (eid, sid, section)
     */
    pub fn read_by_id(c: &Connection, id: &i64) -> color_eyre::Result<Option<(i64, i64, Section)>> {
        let mut stmt = c.prepare("SELECT id, entry_id, position, title, content FROM sections WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![*id])?;
        rows.next()?.map(|row| {Self::map_row_to_id_section(row)}).transpose()
    }
    pub fn map_row_to_id_section(row: &Row) -> color_eyre::Result<(i64, i64, Section)> {
        let eid: i64 = row.get(1)?;
        let sid: i64 = row.get(0)?;
        Ok(
            (
                eid,
                sid,
                Section {
                    position: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                }
            )
        )
    }
}

