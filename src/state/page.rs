use crate::model::Entry;
use rusqlite::Connection;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use edtui::EditorState;

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

    // Shared Data
    pub active_entry_id: Rc<RefCell<Option<i64>>>,
    pub entries: Rc<RefCell<HashMap<i64, Entry>>>,
    pub updated_entry_ids: Rc<RefCell<Vec<i64>>>, 
}
pub struct GlyphNavigationBarState {
    pub is_focused: bool,
    pub line_height: usize,
    pub hovered_index: Option<usize>,
    pub offset: usize,
    // Shared Data
    pub active_entry_id: Rc<RefCell<Option<i64>>>,
    pub ref_entries: Rc<RefCell<HashMap<i64, Entry>>>,
    pub updated_entry_ids: Rc<RefCell<Vec<i64>>>,
}

pub struct GlyphReaderState{
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    // Shared Data
    pub active_entry_id: Rc<RefCell<Option<i64>>>,
    pub ref_entries: Rc<RefCell<HashMap<i64, Entry>>>,
    pub updated_entry_ids: Rc<RefCell<Vec<i64>>>,
}

pub struct GlyphEditorState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    // Shared Data
    pub active_entry_id: Rc<RefCell<Option<i64>>>,
    pub ref_entries: Rc<RefCell<HashMap<i64, Entry>>>,
    pub updated_entry_ids: Rc<RefCell<Vec<i64>>>,
    
    pub editor_state: EditorState,
}