use crate::app::widget::{DirectoryList, SimpleButton};
use crate::app::Command;
use crate::event_handler::Interactable;
use crossterm::event::KeyEvent;
impl Interactable for SimpleButton {
    fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command> {
        if let Some(f) = &mut self.on_interact {
            f()
        } else {
            Ok(Command::None)
        }
    }
}
impl Interactable for DirectoryList {
    fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command> {
        Ok(Command::None)
    }
}