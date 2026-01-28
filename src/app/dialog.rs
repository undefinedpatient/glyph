use crate::app::widget::{LineButton, TextField};
use crate::app::{Command, Component, Container};
use crate::state::dialog::{CreateGlyphDialogState, TextInputDialogState};
use color_eyre::eyre::Result;
use std::ops::Add;
use std::path::PathBuf;

pub struct CreateGlyphDialog {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: CreateGlyphDialogState,
}
impl CreateGlyphDialog {
    pub fn new(path_buf: PathBuf) -> Self {
        Self {
            containers: vec![Box::new(TextField::new(
                "Glpyh Name",
                "Untitled".to_string(),
            ))],
            components: vec![
                LineButton::new("Back").on_interact(Box::new(|_| Ok(vec![Command::PopDialog]))).into(),
                LineButton::new("Confirm").on_interact(Box::new(|state_data| {
                    let state = state_data
                        .unwrap()
                        .downcast_mut::<CreateGlyphDialogState>()
                        .unwrap();
                    let mut commands: Vec<Command> = Vec::new();
                    commands.push(Command::PopPage);
                    commands.push(Command::PopDialog);
                    commands.push(Command::CreateGlyph(
                        state.path_buf.clone(),
                        state.text_input.clone().add(".glyph"),
                    ));
                    Ok(commands)
                })).into(),
            ],
            state: CreateGlyphDialogState {
                is_focused: false,
                hovered_index: None,
                text_input: String::from("default"),
                path_buf: path_buf,
            },
        }
    }
}

pub struct TextInputDialog {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: TextInputDialogState,
    pub on_submit: Option<Box<dyn FnOnce(String) -> Result<Vec<Command>>>>,
}
impl TextInputDialog {
    pub fn new(field_title: &str, default: &str, on_submit: Box<dyn FnOnce(String) -> Result<Vec<Command>>>) -> Self {
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
}
impl From<TextInputDialog> for Box<dyn Container> {
    fn from(container: TextInputDialog) -> Self {
        Box::new(container)
    }
}