use std::any::Any;
use ratatui::widgets::ListState;

pub trait Focusable: Any {
    fn set_focused(&mut self, focused: bool) -> ();
    fn is_focused(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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










/*
    List
*/
pub struct GListState {
    is_focused: bool,
    list_state: ListState,
}
impl GListState {
    pub fn new() -> Self {
        Self{
            is_focused: false,
            list_state: ListState::default(),
        }
    }
}
impl Focusable for GListState {
    fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }
    fn is_focused(&self) -> bool {
        self.is_focused
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/*
    Buttons
*/
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/*
   TextFields
*/

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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
