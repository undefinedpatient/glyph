use crate::app::widget::{LineButton, TextField};
use crate::app::Command::{self, *};
use crate::app::PageCommand::*;
use crate::app::{Component, Container};
use crate::state::dialog::{ConfirmDialogState, NumberInputDialogState, TextInputDialogState};
use crate::state::widget::TextFieldState;
use crate::utils::cycle_offset;
use color_eyre::eyre::Result;
use std::any::Any;


/*
    Dialog is simply a overlay container, all dialog on_xxx() take two state, parent_state and the state dialog possesses.
 */


pub struct TextInputDialog {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: TextInputDialogState,

    pub on_submit: Option<Box<dyn FnOnce(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl TextInputDialog {
    pub fn new(field_title: &str, default: &str) -> Self {
        Self {
            containers: vec![
                TextField::new(
                    field_title,
                    String::from(default),
                )
                    .on_exit(
                        Box::new(
                            |parent_state, state| {
                                let _parent_state = parent_state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                let _state = state.unwrap().downcast_mut::<TextFieldState>().unwrap();
                                _parent_state.text_input = _state.chars.iter().collect::<String>();
                                Ok(Vec::new())
                            }
                        )
                    )
                    .into()
            ],
            components: vec![
                LineButton::new("Back").on_interact(Box::new(|_| Ok(vec![PageCommand(PopDialog)]))).into(),
                LineButton::new("Confirm").into(),
            ],
            state: TextInputDialogState {
                is_focused: false,
                hovered_index: None,
                text_input: String::from(default),
            },
            on_submit: None,
        }
    }

    pub fn on_submit(mut self, on_submit:Box<dyn FnOnce(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>) ->Self {
        self.on_submit = Some(on_submit);
        self
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


pub struct ConfirmDialog {
    pub components: Vec<Box<dyn Component>>,
    pub state: ConfirmDialogState,
    pub message: String,

    pub on_submit: Option<Box<dyn FnOnce(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl ConfirmDialog {
    pub fn new(message: &str) -> Self {
        Self {
            components: vec![
                LineButton::new("Back").on_interact(Box::new(|_| Ok(vec![PageCommand(PopDialog)]))).into(),
                LineButton::new("Confirm").into(),
            ],
            state: ConfirmDialogState {
                is_focused: false,
                hovered_index: None,
            },
            message: String::from(message),
            on_submit: None,
        }
    }

    pub fn on_submit(mut self, on_submit:Box<dyn FnOnce(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>) ->Self {
        self.on_submit = Some(on_submit);
        self
    }

    pub(crate) fn cycle_hover(&mut self, offset: i16) -> () {
        let max: u16 = (self.components.len()) as u16;
        if let Some(hover_index) = self.state.hovered_index {
            self.state.hovered_index = Some(cycle_offset(hover_index as u16, offset, max) as usize);
        } else {
            self.state.hovered_index = Some(0);
        }
    }
}
impl From<ConfirmDialog> for Box<dyn Container> {
    fn from(container: ConfirmDialog) -> Self {
        Box::new(container)
    }
}
/*
    Number Input Dialog
 */
pub struct NumberInputDialog {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: NumberInputDialogState,

    pub on_submit: Option<Box<dyn FnOnce(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl NumberInputDialog {
    pub fn new(field_title: &str, default: i16) -> Self {
        Self {
            containers: vec![
                TextField::new(
                    field_title,
                    default.to_string(),
                )
                    .on_exit(
                        Box::new(
                            |parent_state, state| {
                                let _parent_state = parent_state.unwrap().downcast_mut::<NumberInputDialog>().unwrap();
                                let _state = state.unwrap().downcast_mut::<TextFieldState>().unwrap();
                                _parent_state.state.number_input =  _state.chars.iter().collect::<String>().parse::<i16>().unwrap();
                                Ok(Vec::new())
                            }
                        )
                    )
                    .into()
            ],
            components: vec![
                LineButton::new("Back").on_interact(Box::new(|_| Ok(vec![PageCommand(PopDialog)]))).into(),
                LineButton::new("Confirm").into(),
            ],
            state: NumberInputDialogState {
                is_focused: false,
                hovered_index: None,
                number_input: default
            },
            on_submit: None,
        }
    }

    pub fn on_submit(mut self, on_submit:Box<dyn FnOnce(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>) ->Self {
        self.on_submit = Some(on_submit);
        self
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
impl From<NumberInputDialog> for Box<dyn Container> {
    fn from(container: NumberInputDialog) -> Self {
        Box::new(container)
    }
}
