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
    pub connection: Rc<Connection>,
    pub entries: Rc<Vec<Entry>>
}
pub struct GlyphNavigationBarState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub ref_entries: Rc<Vec<Entry>>
}
