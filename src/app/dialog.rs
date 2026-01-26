use crate::app::widget::{LineButton, TextField};
use crate::app::{Command, Component, Container};
use crate::state::dialog::CreateGlyphDialogState;
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
                Box::new(
                    LineButton::new("Back").on_interact(Box::new(|_| Ok(vec![Command::PopDialog]))),
                ),
                Box::new(
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
                            state.new_glyph_name.clone(),
                        ));
                        Ok(commands)
                    })),
                ),
            ],
            state: CreateGlyphDialogState {
                is_focused: false,
                hover_index: None,
                new_glyph_name: String::from("default"),
                path_buf: path_buf,
            },
        }
    }
}
