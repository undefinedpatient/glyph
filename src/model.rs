use color_eyre::eyre::Result;
use color_eyre::Report;
use rusqlite::{params, Connection, Row, Rows};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct LocalEntryState {
    pub entries: HashMap<i64, Entry>,
    pub connection: Connection,
    pub active_entry_id: Option<i64>,
    pub updated_entry_ids:  Vec<i64>,
    pub ordered_entries: Vec<(i64, String)>,
}
impl LocalEntryState {
    pub fn new(c: Connection) -> Self {
        let entries = EntryRepository::read_all(&c).unwrap();
        let ordered_entries: Vec<(i64, String)> = entries.iter().map(
            |(id,entry)| {
                (id.clone(), entry.entry_name.clone())
            }
        ).collect();
        Self {
            entries: entries,
            connection: c,
            active_entry_id: None,
            updated_entry_ids: Vec::new(),
            ordered_entries,
        }
    }

    /*
        Create New Entry, return Entry ID
     */
    pub fn create_new_entry(&mut self, entry_name: &str) -> Result<i64> {
        let entry_id: i64 = EntryRepository::create_entry(&self.connection, entry_name)?;
        let result = EntryRepository::read_by_id(&self.connection, &entry_id);
        match result {
            Ok(succ) => {
                if let Some((id, entry)) = succ {
                    self.entries.insert(id, entry);
                    self.reconstruct_entry_order();
                    Ok(id)
                } else {
                    Err(Report::msg("Entry Created, but could not be found"))
                }
            }
            Err(e) => Err(e)
        }
    }
    /*
        Delete Entry by ID
     */
    pub fn delete_active_entry(&mut self) -> Result<usize> {
        if let Some(id) = self.active_entry_id {
            let result = EntryRepository::delete_by_id(&self.connection, &id);
            self.entries.remove(&id);
            self.reconstruct_entry_order();
            self.active_entry_id = None;
            result
        } else {
            Err(Report::msg("No active entry found"))
        }
    }

    /*
        Create Section to active_entry
     */
    pub fn create_section_to_active_entry(&mut self, title: &str, content: &str) -> Result<(i64)> {
        if let Some(id) = self.active_entry_id {
            let new_section: Section = Section::new(title, content, self.get_max_position(id)+1);
            let sid: i64 = SectionRepository::create_section(&self.connection, &id, &new_section)?;
            let active_entry = self.entries.get_mut(&id).unwrap();
            active_entry.sections.insert(sid, new_section);
            Ok(sid)
        } else {
            Err(Report::msg("No active entry found"))
        }
    }
    /*
        Create Layout to active_entry
     */
    pub fn insert_layout_to_active_entry(&mut self, layout: Layout, coordinate: &Vec<usize>) -> Result<()> {
        let layout_id = self.get_active_entry_ref().unwrap().layout.0;
        if let Some(id) = self.active_entry_id {
            self.get_layout_at_mut(&id,coordinate).unwrap().sub_layouts.push(layout);
            let layout = self.get_layout_at_ref(&id, coordinate).unwrap();
            LayoutRepository::update_layout(&self.connection, &id, &layout_id, &layout)?;
            let active_entry = self.entries.get_mut(&id).unwrap();
            active_entry.layout.1 = LayoutRepository::read_by_entry_id(&self.connection, &id)?.1;
            Ok(())
        } else {
            Err(Report::msg("No active entry found"))
        }

    }
    pub fn toggle_active_entry_id(&mut self, id: i64) {
        if let Some(oid) = self.active_entry_id {
            if oid == id {
                self.active_entry_id = None;
            } else {
                self.active_entry_id = Some(id);
            }
        } else {
            self.active_entry_id = Some(id);
        }
    }
    pub fn get_active_entry_ref(&self) -> Option<&Entry> {
        if let Some(active_entry_id) = self.active_entry_id {
            return self.entries.get(&active_entry_id);
        }
        None
    }
    pub fn get_active_entry_mut(&mut self) -> Option<&mut Entry> {
        if let Some(active_entry_id) = self.active_entry_id {
            return self.entries.get_mut(&active_entry_id);
        }
        None
    }

    pub fn get_layout_at_mut(&mut self, eid: &i64, coordinates: &Vec<usize>) -> Option<&mut Layout> {
        let mut coor = coordinates.clone();
        let root_entry = self.entries.get_mut(&eid).unwrap();
        coor.reverse();
        if coor.len() == 0 {
            return None;
        }
        let mut temp_layout: &mut Layout = &mut root_entry.layout.1;
        coor.pop();
        while let Some(index) =  coor.pop() {
            temp_layout = &mut (*temp_layout).sub_layouts[index];
        }
        Some(temp_layout)
    }

    pub fn get_layout_at_ref(&self, eid: &i64, coordinates: &Vec<usize>) -> Option<&Layout> {
        let mut coor = coordinates.clone();
        let root_entry = self.entries.get(&eid).unwrap();
        coor.reverse();
        if coor.len() == 0 {
            return None;
        }
        let mut temp_layout: &Layout = &root_entry.layout.1;
        coor.pop();

        while let Some(index) =  coor.pop() {
            temp_layout = &(*temp_layout).sub_layouts[index];
        }
        Some(temp_layout)
    }
    pub fn get_num_sublayout_at(&self, eid: &i64, coordinates: &Vec<usize>) -> usize {
        if let Some(layout) =  self.get_layout_at_ref(&eid, coordinates) {
            layout.sub_layouts.len()
        } else {
            1 // root always has one layout
        }
    }

    fn reconstruct_entry_order(&mut self) -> () {
        self.ordered_entries = self.entries.iter().map(
            |(id,entry)| {
                (id.clone(), entry.entry_name.clone())
            }
        ).collect();
    }
    fn get_max_position(&self, eid: i64) -> i64 {
        let sections = &self.entries.get(&eid).unwrap().sections;
        let mut max: i64 = 0;
        for (sid, section) in sections {
            if section.position > max {
                max = section.position;
            }
        }
        max
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
    pub fn new(title: &str, default: &str, position: i64) -> Self {
        Self {
            position: position,
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
    pub details: LayoutDetails
}

impl Layout {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            section_index: None,
            sub_layouts: Vec::new(),
            details: LayoutDetails::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum LayoutOrientation {
    Horizontal,
    Vertical,
}
#[derive(Serialize, Deserialize)]
pub struct LayoutDetails {
    pub length: u16, // Describe Self
    pub flex: u16, // Describe Self
    pub orientation: LayoutOrientation, // Describing orientation main axis for the children
}

impl LayoutDetails {
    pub fn new() -> Self {
        Self {
            length: 0,
            flex: 1,
            orientation: LayoutOrientation::Vertical,
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

            UNIQUE (entry_id, position)
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
    pub fn create_entry(c: &Connection, entry_name: &str) -> Result<i64> {
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


    pub fn delete_by_id(c: &Connection, eid: &i64) -> Result<usize> {
        let num_of_row_deleted = c.execute( "DELETE FROM entries WHERE id = ?1",
            params![eid],
        )?;
        Ok(num_of_row_deleted)
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
