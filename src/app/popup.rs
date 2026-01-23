use crate::app::Container;
use crate::event_handler::Focusable;

pub struct MessagePopup {
    pub is_focused: bool,
    pub message: String,
}
impl MessagePopup {
    pub fn new(message: &str) -> Self {
        Self { 
            is_focused: false,
            message: String::from(message),
        }
    }
}
impl Focusable for MessagePopup {
    fn is_focused(&self) -> bool {
        self.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
}
pub struct ExitConfirmPopup {
    pub is_focused: bool,
    pub focus_index: usize,
}
impl ExitConfirmPopup {
    pub fn new(is_focused: bool) -> Self {
        Self {
            is_focused,
            focus_index: 0,
        }
    }
}
impl Focusable for ExitConfirmPopup {
    fn is_focused(&self) -> bool {
        self.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
}