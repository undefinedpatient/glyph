use color_eyre::eyre::Result;
use color_eyre::Report;
use rusqlite::{params, Connection, Row, Rows, Statement};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

/// This store the fetched entries information, this avoids frequent interaction between application and database.
pub struct LocalEntryState {
    /// All entries in the database.
    pub entries: Vec<(i64, Entry)>,
    pub connection: Connection,
    /// Current Active Entry being read/edited.
    pub active_entry_id: Option<i64>,
    /// Holding all changed entries.
    pub updated_entries:  HashSet<i64>,
    /// Holding entry id and entry name in specific order.
    pub ordered_entries: Vec<(i64, String)>,
}

impl LocalEntryState {
    /// Create new LocalEntryState from connection.
    pub fn new(c: Connection) -> Self {
        let entries = EntryRepository::read_all(&c).unwrap();
        let ordered_entries: Vec<(i64, String)> = entries.iter().map(
            |(id,entry)| {
                (id.clone(), entry.entry_name.clone())
            }
        ).collect();
        Self {
            entries,
            connection: c,
            active_entry_id: None,
            ordered_entries,
            updated_entries: HashSet::new(),
        }
    }

    /// Create New Entry, return Entry ID.
    pub fn get_entry_ref(&self, eid: &i64) -> Option<&Entry> {
        for (_eid, entry) in &self.entries {
            if *_eid == *eid {
                return Some(entry);
            }
        }
        None
    }

    /// Get mut ref Entry by its id, return Some(Entry) if and only if a relevant entry exist in state.
    pub fn get_entry_mut(&mut self, eid: &i64) -> Option<&mut Entry> {
        for (_eid, entry) in &mut self.entries {
            if *_eid == *eid {
                return Some(entry);
            }
        }
        None
    }

    /// Create a default entry in the database, this function interact and update database.
    pub fn create_default_entry_db(&mut self, entry_name: &str) -> Result<i64> {
        let entry_id: i64 = EntryRepository::create_default_entry(&self.connection, entry_name)?;
        let entry_result = EntryRepository::read_by_id(&self.connection, &entry_id);
        match entry_result {
            Ok((eid, entry)) => {
                self.entries.push((eid, entry));
                self.reconstruct_entry_order();
                Ok(eid)
            }
            Err(e) => Err(e)
        }
    }

    /// Update entry's name by its id, this function interact and update database.
    pub fn update_entry_name_db(&mut self, eid: &i64, new_name: &str) -> Result<()> {

        EntryRepository::update_name(&self.connection, &eid, new_name)?;
        let (eid, entry) = EntryRepository::read_by_id(&self.connection, &eid)?;

        let current_entry: &mut Entry = self.get_entry_mut(&eid).unwrap();
        current_entry.update_name(&entry);
        Ok(())
    }

    /// Update the database using the corresponding Entry in local state pointed by the eid parameter.
    pub fn save_entry_db(&mut self, eid: &i64) -> Result<()> {
        let entry: &Entry = self.get_entry_ref(eid).unwrap();
        EntryRepository::update_entry(&self.connection, eid, entry)?;
        Ok(())
    }

    /// Update the database by deleting corresponding Entry pointed by the eid parameter.
    pub fn delete_active_entry_db(&mut self) -> Result<usize> {
        if let Some(id) = self.active_entry_id {
            let result = EntryRepository::delete_by_id(&self.connection, &id);
            let mut remove_index: i8 = -1;
            for (index, entry) in self.entries.iter().enumerate() {
                if (*entry).0 == id {
                    remove_index = index as i8;
                    break;
                }
            }
            if remove_index == -1 {
                return Err(Report::msg("Entry could not be found"));
            }
            self.entries.remove(remove_index as usize);
            self.reconstruct_entry_order();
            self.active_entry_id = None;
            result
        } else {
            Err(Report::msg("No active entry found"))
        }
    }

    /// Update the current active entry eid.
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

    /// This function return ref Entry, that is currently active.
    pub fn get_active_entry_ref(&self) -> Option<&Entry> {
        if let Some(active_entry_id) = self.active_entry_id {
            return self.get_entry_ref(&active_entry_id);
        }
        None
    }
    /// This function return ref mut Entry, that is currently active.
    pub fn get_active_entry_mut(&mut self) -> Option<&mut Entry> {
        if let Some(active_entry_id) = self.active_entry_id {
            return self.get_entry_mut(&active_entry_id);
        }
        None
    }
    /*

        Section Section

     */
    /// Get ref Section by eid and sid.
    pub fn get_section_ref(&self, eid: &i64, sid: &i64) -> Option<&Section> {
        if let Some(entry) = self.get_entry_ref(eid) {
            for (_sid, section) in &entry.sections {
                if *_sid == *sid {
                    return Some(section);
                }
            }
            return None;
        } else {
            None
        }
    }
    /// Get ref mut Section by eid and sid.
    pub fn get_section_mut(&mut self, eid: &i64, sid: &i64) -> Option<&mut Section> {
        if let Some(entry) = self.get_entry_mut(eid) {
            for (_sid, section) in &mut entry.sections {
                if *_sid == *sid {
                    return Some(section);
                }
            }
            None
        } else {
            None
        }
    }

    /// Create a new section and insert it into database.
    pub fn create_section_to_active_entry_db(&mut self, title: &str, content: &str) -> Result<i64> {
        if let Some(eid) = self.active_entry_id {
            let new_section: Section = Section::new(title, content, self.get_max_position_section(&eid)+1);
            let sid: i64 = SectionRepository::create_section(&self.connection, &eid, &new_section)?;
            let active_entry = self.get_entry_mut(&eid).unwrap();
            active_entry.sections.push((sid, new_section));
            Ok(sid)
        } else {
            Err(Report::msg("No active entry found"))
        }
    }

    /// Update the Section specified by its id in the database, update the local state as well.
    pub fn update_section_by_sid_db(&mut self, sid: &i64, section: Section) -> Result<()> {
        let _sid = SectionRepository::update_section(&self.connection, sid, &section)?;
        let (eid, sid, section) = SectionRepository::read_by_id(&self.connection, sid)?.unwrap();
        // Update all entry having the same layout
        let sections: &mut Vec<(i64, Section)> = &mut self.get_entry_mut(&eid).unwrap().sections;

        let mut section_index_to_remove: usize = usize::MAX;
        for (index, item) in sections.iter().enumerate() {
            if (*item).0 == sid {
                section_index_to_remove = index as usize;
            }
        }
        if section_index_to_remove == usize::MAX {
            return Err(Report::msg("No active entry found"));
        }
        sections.remove(section_index_to_remove);
        sections.push((sid, section));
        Ok(())

    }

    /// Delete the corresponding Section in the database specified by the sid.
    pub fn delete_section_db(&mut self, sid: &i64) -> Result<()> {
        let (eid, sid, section) = SectionRepository::read_by_id(&self.connection, sid)?.unwrap();
        SectionRepository::delete_section(&self.connection, &sid)?;
        if let Some(entry) = self.get_entry_mut(&eid) {
            let mut index_of_section: usize = usize::MAX;
            for (index, item) in &mut entry.sections.iter().enumerate() {
                if (*item).0 == sid {
                    index_of_section = index;
                }
            }
            if index_of_section == usize::MAX {
                return Err(Report::msg("Section not found"));
            }
            entry.sections.remove(index_of_section);
            Ok(())
        } else {
            Err(Report::msg("Entry not found"))
        }
    }

    /// Return the total count of Sections under an Entry specified by its id.
    pub fn get_num_sections(&self, eid: &i64) -> usize {
        if let Some(entry) = self.get_entry_ref(eid){
            entry.sections.len()
        } else {
            0
        }
    }

    /// Return a ref Vec<(sid: i64, Section)> under an Entry specified by its id.
    pub fn get_sections_ref(&self, eid: &i64) -> &Vec<(i64, Section)> {
        &self.get_entry_ref(eid).unwrap().sections
    }

    /// Return all section's id under an Entry specified by its id.
    pub fn get_sections_sid(&self, eid: &i64) -> Vec<i64> {
        self.get_entry_ref(eid).unwrap().sections.iter().map(
            |(sid, section)| {
                sid.clone()
            }
        ).collect()
    }

    pub fn sort_sections_by_position(&mut self, eid: &i64) -> () {
        if let Some(entry) = self.get_entry_mut(eid) {
            entry.sections.sort_by(|cur, nex| {
                (*cur).1.position.cmp(&(*nex).1.position)
            })
        }
    }

    /// Filter all reachable entries and overwrite the old ordered_entries
    /// It does not automatically sort the result.
    pub fn filter_entry_order_by(&mut self, predicate: &dyn Fn(&str)->bool) -> () {
        let mut new_ordered_entries: Vec<(i64, String)> = Vec::new();
        for (eid, entry) in &self.entries {
            if predicate(entry.entry_name.as_str()) {
                new_ordered_entries.push((*eid, entry.entry_name.clone()));
            }
        }
        self.ordered_entries = new_ordered_entries;
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


    /// Return the max. section position designated by the user.
    fn get_max_position_section(&self, eid: &i64) -> i64 {
        let sections = &self.get_entry_ref(eid).unwrap().sections;
        let mut max: i64 = 0;
        for (sid, section) in sections {
            if section.position > max {
                max = section.position;
            }
        }
        max
    }

    /// Return the min. section position designated by the user.
    fn get_min_position_section(&self, eid: &i64) -> i64 {
        let sections = &self.get_entry_ref(eid).unwrap().sections;
        let mut min: i64 = 0;
        for (sid, section) in sections {
            if section.position > min {
                min = section.position;
            }
        }
        min
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
    pub sections: Vec<(i64, Section)>,
    pub layout: Layout,
}

impl Entry {
    pub fn update_name(&mut self, other: &Entry) -> () {
        if self.entry_name != other.entry_name {
            self.entry_name = other.entry_name.clone();
        }
    }
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
    pub details: LayoutDetails,
}
impl Layout {
    pub fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
            section_index: None,
            sub_layouts: Vec::new(),
            details: LayoutDetails::new(),
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
            let coor = coordinates.clone();
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
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SizeMode {
    Length,
    Flex,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum BorderMode {
    None,
    Plain,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutDetails {
    pub size_mode: SizeMode,
    pub border_mode: BorderMode,
    pub length: u16, // Describe Self
    pub flex: u16, // Describe Self

    pub orientation: LayoutOrientation, // Describing orientation main axis for the children
}



impl LayoutDetails {
    pub fn new() -> Self {
        Self {
            size_mode: SizeMode::Flex,
            border_mode: BorderMode::None,
            length: 42,
            flex: 1,

            orientation: LayoutOrientation::Vertical,
        }
    }
}


pub struct GlyphRepository {}

struct EntryRepository {}

impl GlyphRepository {
    pub fn init_glyph_db(path_to_db: &PathBuf) -> Result<Connection> {
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
    pub fn create_default_entry(c: &Connection, entry_name: &str) -> Result<i64> {
        c.execute(
            "INSERT INTO entries (entry_name, layout) VALUES (?1, ?2)",
            params![entry_name, serde_json::to_string(&Layout::new("Root"))?],
        )?;
        let eid: i64 = c.last_insert_rowid();
        return Ok(eid);
    }
    pub fn update_entry(c: &Connection, eid: &i64, entry: &Entry) -> Result<(i64)> {
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
    pub fn update_name(c: &Connection, eid: &i64, new_name: &str) -> Result<()> {
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

    pub fn delete_by_id(c: &Connection, eid: &i64) -> Result<usize> {
        let num_of_row_deleted = c.execute( "DELETE FROM entries WHERE id = ?1",
            params![eid],
        )?;
        Ok(num_of_row_deleted)
    }

    pub fn read_by_id(c: &Connection, id: &i64) -> Result<(i64, Entry)> {
        let mut stmt = c.prepare("SELECT id, entry_name, layout FROM entries WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![*id])?;
        Self::map_row_to_eid_entry(c, rows.next()?.unwrap())
    }

    pub fn read_all(c: &Connection) -> Result<Vec<(i64, Entry)>> {
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
    fn map_row_to_eid_entry(c: &Connection, row: &Row) -> Result<(i64, Entry)> {
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
struct SectionRepository {}
impl SectionRepository {
    pub fn read_by_entry_id(c: &Connection, entry_id: &i64) -> Result<Vec<(i64, Section)>> {
        let mut stmt = c.prepare("SELECT id, entry_id, position, title, content FROM sections WHERE entry_id = ?1 ORDER BY position ASC")?;
        let mut rows: Rows = stmt.query(params![*entry_id])?;
        let mut sections: Vec<(i64, Section)> = Vec::new();
        while let Some(row) = rows.next()? {
            let section = Self::map_row_to_id_section(row)?;
            sections.push((section.1, section.2));
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
    pub fn update_section(c: &Connection, sid: &i64, section: &Section) -> Result<i64> {
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

    pub fn delete_section(c: &Connection, sid: &i64) -> Result<usize> {
        let num_of_row_deleted = c.execute( "DELETE FROM sections WHERE id = ?1",
                                            params![sid],
        )?;


        Ok(num_of_row_deleted)
    }

    /*
        Return (eid, sid, section)
     */
    pub fn read_by_id(c: &Connection, id: &i64) -> Result<Option<(i64, i64, Section)>> {
        let mut stmt = c.prepare("SELECT id, entry_id, position, title, content FROM sections WHERE id = ?1")?;
        let mut rows: Rows = stmt.query(params![*id])?;
        rows.next()?.map(|row| {Self::map_row_to_id_section(row)}).transpose()
    }
    pub fn map_row_to_id_section(row: &Row) -> Result<(i64, i64, Section)> {
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

