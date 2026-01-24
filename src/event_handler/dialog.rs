use crossterm::event::KeyEvent;
use crate::app::{Command, Container};
use crate::app::dialog::TextInputDialog;
use crate::event_handler::{Focusable, Interactable};

impl Interactable for TextInputDialog {
    fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command> {
        todo!()
    }
}
impl Focusable for TextInputDialog {
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