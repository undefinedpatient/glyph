use crate::app::dialog::{ConfirmDialog, TextInputDialog};
use crate::app::Container;
use crate::event_handler::Focusable;

impl Focusable for TextInputDialog {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }

    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(&**container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        for (index, container) in self.containers.iter().enumerate() {
            if container.is_focused() {
                return Some(index);
            }
        }
        None
    }
}
impl Focusable for ConfirmDialog {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
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
