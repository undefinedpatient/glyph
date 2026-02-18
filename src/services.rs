use crate::db::{EntryRepository, SectionRepository};
use std::collections::HashSet;
use color_eyre::Report;
use rusqlite::Connection;
use crate::models::entry::Entry;
use crate::models::section::Section;

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
    pub fn create_default_entry_db(&mut self, entry_name: &str) -> color_eyre::Result<i64> {
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
    pub fn update_entry_name_db(&mut self, eid: &i64, new_name: &str) -> color_eyre::Result<()> {

        EntryRepository::update_name(&self.connection, &eid, new_name)?;
        let (eid, entry) = EntryRepository::read_by_id(&self.connection, &eid)?;

        let current_entry: &mut Entry = self.get_entry_mut(&eid).unwrap();
        current_entry.update_name(&entry);
        Ok(())
    }

    /// Update the database using the corresponding Entry in local state pointed by the eid parameter.
    pub fn save_entry_db(&mut self, eid: &i64) -> color_eyre::Result<()> {
        let entry: &Entry = self.get_entry_ref(eid).unwrap();
        EntryRepository::update_entry(&self.connection, eid, entry)?;
        Ok(())
    }

    /// Update the database by deleting corresponding Entry pointed by the eid parameter.
    pub fn delete_active_entry_db(&mut self) -> color_eyre::Result<usize> {
        if let Some(id) = self.active_entry_id {
            let result = crate::db::EntryRepository::delete_by_id(&self.connection, &id);
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
    /// Update section's name by its id, this function interact and update database.
    pub fn update_section_name_db(&mut self, sid: &i64, new_name: &str) -> color_eyre::Result<()> {

        SectionRepository::update_name(&self.connection, &sid, new_name)?;
        let (eid, sid, section): (i64, i64, Section) = SectionRepository::read_by_id(&self.connection, &sid)?.unwrap();

        let current_section: &mut Section = self.get_section_mut(&eid, &sid).unwrap();
        current_section.title = section.title;
        Ok(())
    }

    /// Create a new section and insert it into database.
    pub fn create_section_to_active_entry_db(&mut self, title: &str, content: &str) -> color_eyre::Result<i64> {
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
    pub fn update_section_by_sid_db(&mut self, sid: &i64, section: Section) -> color_eyre::Result<()> {
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
    pub fn delete_section_db(&mut self, sid: &i64) -> color_eyre::Result<()> {
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
