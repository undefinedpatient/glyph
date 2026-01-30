use crate::model::{Entry, LocalEntryState};
use rusqlite::Connection;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use crate::state::widget::EditorState;

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
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}





pub struct GlyphNavigationBarState {
    pub is_focused: bool,
    pub line_height: usize,
    pub hovered_index: Option<usize>,
    pub offset: usize,
    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
pub enum GlyphMode {
    READ,
    LAYOUT
}
pub struct GlyphViewerState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub mode: GlyphMode,
    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}