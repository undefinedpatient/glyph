use std::path::PathBuf;

pub struct CreateGlyphDialogState {
    pub is_focused: bool,
    pub hover_index: Option<usize>,
    pub new_glyph_name: String,
    pub path_buf: PathBuf,
}
