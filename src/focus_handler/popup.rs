use crate::app::Container;
use crate::app::popup::{ExitConfirmPopup, MessagePopup};
use crate::event_handler::Focusable;

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
    fn focused_child_index(&self) -> Option<usize> {
        None
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
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}
