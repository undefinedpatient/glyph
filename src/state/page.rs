use std::path::PathBuf;

pub struct EntrancePageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hover_index: Option<usize>,
}
pub struct CreateGlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hover_index: Option<usize>,
    pub path_to_create: PathBuf,
}
pub struct OpenGlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hover_index: Option<usize>,
    pub path_to_open: PathBuf,
}
pub struct GlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hover_index: Option<usize>,
    pub root_path: PathBuf,
    pub current_gpath: String,
}
