use crate::model::Entry;
use rusqlite::Connection;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

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

pub struct GlyphReaderState{
    pub is_focused: bool,
    pub entry_id: Option<i64>,
    pub hovered_index: Option<usize>,
}