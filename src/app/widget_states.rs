use std::any::Any;

pub trait Focusable: Any {
    fn set_focused(&mut self, focused: bool) -> ();
    fn is_focused(&self) -> bool;
}
// Can only be called on a dyn Focusable
impl dyn Focusable {
    fn downcast_ref<T: Focusable>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref::<T>(self)
    }
    fn downcast_mut<T: Focusable>(&mut self) -> Option<&mut T> {
        <dyn Any>::downcast_mut::<T>(self)
    }

}
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
    pub fn from_str(default_string: &str) -> Self {
        TextFieldState {
            is_focused: false,
            char_index: 0,
            input_mode: TextInputMode::Normal,
            input_string: Vec::from(default_string.to_string().chars().collect::<Vec<char>>())
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
