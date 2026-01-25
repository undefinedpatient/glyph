use crate::app::widget::{LineButton, TextField};
use crate::app::{Command, Component, Container};

pub struct TextInputDialog {
    pub is_focused: bool,
    pub hover_index: Option<usize>,
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>
}
impl TextInputDialog {
    pub fn new() -> Self {
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
                            Ok(Command::PopDialog)
                        })
                    )
                ),
                Box::new(
                    LineButton::new("Confirm").on_interact(
                        Box::new(|data| {
                            
                        })
                    )
                )
            ],
        }
    }
}