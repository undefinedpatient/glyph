use std::any::Any;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use color_eyre::eyre::Result;
use ratatui::style::Stylize;
use ratatui::widgets::Widget;
use crate::app::{Command, Component};
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Interactable;
use crate::theme::Theme;

pub struct OptionMenuState {
    pub current_index: u8,
    pub options: Vec<(String, u8)>
}
pub struct OptionMenu {
    pub state: OptionMenuState,
    pub on_update: Option<Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}

impl OptionMenu {
    pub fn new(options: Vec<(String, u8)>, default: u8) -> Self {
        Self {
            state: OptionMenuState {
                current_index: default,
                options,
            },
            on_update: None
        }
    }
    pub fn on_interact(
        mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        self.on_update = Some(f);
        self
    }

    pub fn replace(&mut self, new_selection: u8) -> () {
        self.state.current_index = new_selection;
    }
}

impl From<OptionMenu> for Box<dyn Component> {
    fn from(container: OptionMenu) -> Self {
        Box::new(container)
    }
}
impl Drawable for OptionMenu {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let current_index: usize = self.state.current_index as usize;
        let current_text: String = self.state.options.get(current_index).unwrap().0.clone();
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from(["[< ", current_text.as_str(), " >]"].concat())
                    .bold()
                    .centered()
                    .render(area, frame.buffer_mut());
            }
            _ => {
                Line::from([" < ", current_text.as_str(), " > "].concat())
                    .centered()
                    .render(area, frame.buffer_mut());
            }
        }
    }
}

impl Interactable for OptionMenu {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        let len: u8 = self.state.options.len() as u8;
        self.state.current_index = (self.state.current_index + 1) % len;


        let Some(mut f) = self.on_update.take() else {
            return Ok(Vec::new());
        };
        let result = f(parent_state, Some(&mut self.state));
        self.on_update = Some(f);
        result
    }
}
