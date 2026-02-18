/*
    Entry
 */
use crate::models::layout::Layout;
use crate::models::section::Section;

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
