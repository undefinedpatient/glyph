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
