pub struct TextFieldWidgetState{
    pub is_focused: bool,
    pub label: String,
    pub chars: Vec<char>,
    pub cursor_index: usize,
}