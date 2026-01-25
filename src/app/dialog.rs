use std::any::Any;
use crate::app::widget::{LineButton, TextField};
use crate::app::{Command, Component, Container};
use crate::state::dialog::CreateGlyphDialogState;
use std::path::PathBuf;
use crate::app::popup::MessagePopup;

pub struct CreateGlyphDialog {
    pub is_focused: bool,
    pub hover_index: Option<usize>,
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: CreateGlyphDialogState,
    pub path_buf: PathBuf
}
impl CreateGlyphDialog {
    pub fn new(path_buf: PathBuf) -> Self {
        Self {
            is_focused: false,
            hover_index: None,
            containers: vec![
                Box::new(
                    TextField::new("Glpyh Name", "Untitled".to_string())
                )
            ],
            components: vec![
                Box::new(
                    LineButton::new("Back").on_interact(
                        Box::new(|_| {
                            Ok(vec![Command::PopDialog])
                        })
                    )
                ),
                Box::new(
                    LineButton::new("Confirm")
                )
            ],
            state: CreateGlyphDialogState{
                new_glyph_name: String::from("default"),
            },
            path_buf: path_buf,
        }
    }
}