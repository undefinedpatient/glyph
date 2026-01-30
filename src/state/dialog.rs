use std::path::PathBuf;

pub struct TextInputDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub text_input: String,
}
pub struct ConfirmDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
}