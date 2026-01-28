use std::path::PathBuf;

pub struct CreateGlyphDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub text_input: String,
    pub path_buf: PathBuf,
}
pub struct TextInputDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub text_input: String,
}