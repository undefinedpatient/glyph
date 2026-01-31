use std::path::PathBuf;

pub struct DirectoryListState {
    pub is_focused: bool,
    pub label: String,
    pub line_height: usize,
    pub current_path: PathBuf,
    pub selected_file_path: Option<PathBuf>,
    pub hovered_index: Option<usize>,
    pub selected_index: Option<usize>,
    pub offset: usize,
    pub show_files: bool,
    pub select_dir: bool,
}
pub struct TextFieldState {
    pub is_focused: bool,
    pub label: String,
    pub chars: Vec<char>,
    pub cursor_index: usize,
}
pub struct NumberFieldState {
    pub is_focused: bool,
    pub label: String,
    pub chars: Vec<char>,
    pub cursor_index: usize,
}
pub struct EditorState {
    pub is_focused: bool,
    pub label: String,
    pub chars: Vec<Vec<char>>,
    pub cursor_index: usize,
    pub cursor_row: usize,
}
impl EditorState  {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            is_focused: false,
            label: String::from(label),
            chars: Vec::new(),
            cursor_index: 0,
            cursor_row: 0,
        }
    }
}