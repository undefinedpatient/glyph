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

    pub fn toggle_local_active_entry_id(&mut self, id: i64) {
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
    /*
        Create Section to active_entry
     */
    pub fn create_section_to_active_entry(&mut self, title: &str, content: &str) -> Result<i64> {
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
    pub fn get_num_sections(&self, eid: &i64) -> usize {
        if let Some(entry) = self.entries.get(eid){
            entry.sections.len()
        } else {
            0
        }
    }
    pub fn get_sections_ref(&self, eid: &i64) -> &HashMap<i64, Section> {
        let entry: &Entry = self.entries.get(eid).unwrap();
        &entry.sections
    }



    /*

        Layout Section

     */

    /*
        Get reference to Layout by lid via searching in Local Entries
     */
    pub fn get_layout_ref(&self, lid: &i64) -> Option<&Layout> {
         self.entries.iter().find_map(
            |item| {
                if item.1.layout.0 == *lid {
                    return Some(&item.1.layout.1);
                }
                None
            }
        )
    }
    pub fn get_layout_mut(&mut self, lid: &i64) -> Option<&mut Layout> {
        self.entries.iter_mut().find_map(
            |item| {
                if item.1.layout.0 == *lid {
                    return Some(&mut item.1.layout.1);
                }
                None
            }
        )
    }
    pub fn get_entry_layout_ref(&self, eid: &i64) -> Option<&Layout> {
        match self.entries.get(eid) {
            Some(e) => Some(&e.layout.1),
            None => None,
        }
    }
    pub fn get_entry_layout_mut(&mut self, eid: &i64) -> Option<&mut Layout> {
        match self.entries.get_mut(eid) {
            Some(e) => Some(&mut e.layout.1),
            None => None,
        }
    }
    pub fn get_entry_lid(&self, eid: &i64) -> Option<i64> {
        match self.entries.get(eid) {
            Some(e) => Some(e.layout.0),
            None => None
        }
    }
    pub fn get_active_entry_lid(&self) -> Option<i64> {
        match self.get_active_entry_ref() {
            Some(e) => Some(e.layout.0),
            None => None
        }
    }

    /*
       Directly Update a layout.
     */
    pub fn update_layout_by_lid(&mut self, lid: &i64, layout: Layout) -> Result<()> {
        LayoutRepository::update_layout_by_lid(&self.connection, &lid, &layout)?;

        // Update all entry having the same layout
        self.entries.iter_mut().for_each(
            |(eid, entry)| {
                if (entry.layout.0 == *lid) {
                    entry.layout.1 = layout.clone();
                }
            }
        );
        Ok(())
    }
    /*

        Helpers

     */



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


/*

    Models

 */

pub struct Glyph {
    pub glyph_name: String,
    pub entries: Vec<Entry>,
}


/*
    Entry
 */
pub struct Entry {
    pub entry_name: String,
    pub sections: HashMap<i64, Section>,
    pub layout: (i64, Layout),
}

/*
    Section
 */
#[derive(Clone)]
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

/*
    Layout
 */
#[derive(Serialize, Deserialize, Clone)]
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
    pub fn get_layout_at_ref(&self, coordinates: &Vec<usize>) -> Option<&Layout> {
        let mut coor = coordinates.clone();
        coor.reverse();
        if coor.len() == 0 {
            return Some(self);
        }
        let mut temp_layout: &Layout = self;
        while let Some(index) =  coor.pop() {
            temp_layout = &(*temp_layout).sub_layouts[index];
        }
        if coor.is_empty() {
            Some(temp_layout)
        } else {
            None
        }
    }
    pub fn get_layout_at_mut(&mut self, coordinates: &Vec<usize>) -> Option<&mut Layout> {
        let mut coor = coordinates.clone();
        coor.reverse();
        if coor.len() == 0 {
            return Some(self);
        }
        let mut temp_layout: &mut Layout = self;
        while let Some(index) =  coor.pop() {
            temp_layout = &mut (*temp_layout).sub_layouts[index];
        }
        if coor.is_empty() {
            Some(temp_layout)
        } else {
            None
        }
    }
    pub fn update_layout_at(&mut self, layout: &Layout, coordinates: &Vec<usize>) {
        if let Some(target) = self.get_layout_at_mut(coordinates){
            target.label = layout.label.clone();
            target.details = layout.details.clone();
        }
    }
    pub fn insert_sublayout_under(&mut self, layout: Layout, coordinates: &Vec<usize>) {
        if coordinates.is_empty() {
            self.sub_layouts.push(layout);
        } else {
            let mut coor = coordinates.clone();
            if let Some(target) = self.get_layout_at_mut(&coor) {
                target.sub_layouts.push(layout);
            }
        }
    }

    pub fn remove_sublayout(&mut self, coordinates: &Vec<usize>) -> Result<()> {
        let mut coor = coordinates.clone();
        let index = coor.pop().unwrap();
        if let Some(target) = self.get_layout_at_mut(&coor) {
            target.sub_layouts.remove(index);
            Ok(())
        } else {
            Err(Report::msg(format!("Tried to remove a layout that does not exist. At {:?}", coor)))
        }

    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LayoutOrientation {
    Horizontal,
    Vertical,
}
#[derive(Serialize, Deserialize, Clone)]
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
                ON CONFLICT (id)
                DO UPDATE SET
                    entry_name = ?2,
            ",
            params![eid, entry.entry_name],
        )?;
        let id: i64 = c.last_insert_rowid();
        for (sid, section) in &entry.sections {
            SectionRepository::update_section(c, eid, sid, section)?;
        }
        LayoutRepository::update_layout_by_lid(c, &entry.layout.0, &entry.layout.1)?;
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
                layout: LayoutRepository::read_by_eid(c, &id)?
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
                ON CONFLICT (id)
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
    pub fn read_by_eid(c: &Connection, entry_id: &i64) -> Result<(i64, Layout)> {
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
    pub fn read_by_lid(c: &Connection, lid: &i64) -> Result<(i64, Layout)> {
        let mut stmt = c.prepare("SELECT id, entry_id, content FROM layouts WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![*lid])?;

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
    pub fn update_layout_eid(c: &Connection, eid: &i64, lid: &i64) -> Result<i64> {
        c.execute(
            "
                UPDATE layouts
                SET
                    entry_id = ?1,
                WHERE id = ?2
            ",
            params![eid, lid],
        )?;
        return Ok(*lid);
    }
    /*
        Create new layout, return the layout ID
     */
    pub fn create_layout(c: &Connection, eid: &i64, layout: &Layout) -> Result<i64> {
        c.execute(
            " INSERT INTO layouts (entry_id, content) VALUES (?1, ?2) ",
            params![eid, serde_json::to_string(layout).unwrap()],
        )?;
        let lid = c.last_insert_rowid();
        return Ok(lid);
    }
    pub fn update_layout_by_lid(c: &Connection, lid: &i64, layout: &Layout) -> Result<i64> {
        c.execute(
            "
                UPDATE layouts
                SET
                    content = ?2
                WHERE id = ?1
            ",
            params![lid, serde_json::to_string(layout).unwrap()],
        )?;
        return Ok(*lid);
    }
    pub fn update_layout_by_eid(c: &Connection, eid: &i64, layout: &Layout) -> Result<i64> {
        c.execute(
            "
                UPDATE layouts
                SET
                    content = ?2,
                WHERE entry_id = ?1
            ",
            params![eid, serde_json::to_string(layout).unwrap()],
        )?;
        return Ok(*eid);
    }

    /*
        Read a whole row, return (lid, Layout)
     */
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
