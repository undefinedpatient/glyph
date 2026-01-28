use crate::app::widget::{LineButton, TextField};
use crate::app::{Command, Component, Container};
use crate::state::dialog::TextInputDialogState;
use color_eyre::eyre::Result;
use std::any::Any;
use crate::utils::{cycle_add, cycle_offset, cycle_sub};

pub struct TextInputDialog {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: TextInputDialogState,
    pub on_submit: Option<Box<dyn FnOnce(String, Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl TextInputDialog {
    pub fn new(field_title: &str, default: &str, on_submit: Box<dyn FnOnce(String, Option<&mut dyn Any>) -> Result<Vec<Command>>>) -> Self {
        Self {
            containers: vec![Box::new(TextField::new(
                field_title,
                String::from(default),
            ))],
            components: vec![
                LineButton::new("Back").on_interact(Box::new(|_| Ok(vec![Command::PopDialog]))).into(),
                LineButton::new("Confirm").into(),
            ],
            state: TextInputDialogState {
                is_focused: false,
                hovered_index: None,
                text_input: String::from(default),
            },
            on_submit: Some(on_submit),
        }
    }

    pub(crate) fn cycle_hover(&mut self, offset: i16) -> () {
        let max: u16 = (self.containers.len() + self.components.len()) as u16;
        if let Some(hover_index) = self.state.hovered_index {
            self.state.hovered_index = Some(cycle_offset(hover_index as u16, offset, max) as usize);
        } else {
            self.state.hovered_index = Some(0);
        }
    }
}
impl From<TextInputDialog> for Box<dyn Container> {
    fn from(container: TextInputDialog) -> Self {
        Box::new(container)
    }
}