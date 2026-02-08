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

pub enum EditMode {
    Normal,
    Insert,
    Visual,
    VisualLine
}
pub struct TextEditorState {
    pub is_focused: bool,
    pub label: String,
    
    pub mode: EditMode,
    pub lines: Vec<Vec<char>>,
    pub scroll_offset: (usize,usize),
    pub cursor_index: usize,
    pub cursor_line_index: usize,

    pub anchor: (usize, usize),

    pub copy_buffer: Vec<Vec<char>>, // First line insert char, the rest directly insert line.
}

pub struct OptionMenuState {
    pub current_index: u8,
    pub options: Vec<(String, u8)>
}