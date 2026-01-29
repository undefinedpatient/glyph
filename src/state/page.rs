use std::cell::RefCell;
use std::collections::HashMap;
use rusqlite::Connection;
use std::path::PathBuf;
use std::rc::Rc;
use crate::model::Entry;

pub struct EntrancePageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
}
pub struct CreateGlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub path_to_create: PathBuf,
}
pub struct OpenGlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub path_to_open: PathBuf,
}
pub struct GlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub connection: Connection,
    pub entries: Rc<RefCell<Vec<Entry>>>
}
pub struct GlyphNavigationBarState {
    pub is_focused: bool,
    pub line_height: usize,
    pub hovered_index: Option<usize>,
    pub selected_id: Option<i64>,
    pub offset: usize,
    pub ref_entries: Rc<RefCell<Vec<Entry>>>,
}
