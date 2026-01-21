use crate::app::states::Focusable;
pub struct ButtonState {
    is_focused: bool,
}

impl ButtonState {
    pub fn new() -> Self {
        ButtonState {
            is_focused: false,
        }
    }
}

impl Focusable for ButtonState {
    fn set_focused(&mut self,is_focused: bool) -> () {
        self.is_focused = is_focused;
    }
    fn is_focused(&self) -> bool {
        self.is_focused
    }
}
pub enum TextInputMode{
    Normal,
    Edit
}

pub struct TextFieldState {
    is_focused: bool,
    char_index: u8,
    input_mode: TextInputMode,
    input_string: Vec<char>
}

impl TextFieldState {
    pub fn new() -> Self {
        TextFieldState {
            is_focused: false,
            char_index: 0,
            input_mode: TextInputMode::Normal,
            input_string: Vec::new()
        }
    }

    pub fn text_as_string(&self) -> String {
        self.input_string.iter().collect()
    }
}

impl Focusable for TextFieldState {
    fn set_focused(&mut self, focused: bool) -> () {
        self.is_focused = focused;
    }

    fn is_focused(&self) -> bool {
        self.is_focused
    }
}
