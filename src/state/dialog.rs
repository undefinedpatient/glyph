pub struct TextInputDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub text_input: String,
}
pub struct ConfirmDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
}
pub struct NumberInputDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub number_input: i16,
}
pub struct EditLayoutDialogState{
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub input_width: u16,
    pub input_height: u16,
}
