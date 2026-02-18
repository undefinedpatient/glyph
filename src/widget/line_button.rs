use std::any::Any;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use color_eyre::eyre::Result;
use ratatui::prelude::{Line, Stylize, Widget};
use crate::app::{Command, Component};
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Interactable;
use crate::theme::Theme;

pub struct LineButton {
    pub label: String,

    pub on_interact: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl LineButton {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_interact: None,
        }
    }
    pub fn on_interact(
        mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        self.on_interact = Some(f);
        self
    }

    pub fn as_line(&self, draw_flag: DrawFlag) -> Line<'_> {
        let text = self.label.clone().to_string();
        match draw_flag {
            DrawFlag::HIGHLIGHTING => Line::from(["[", text.as_str(), "]"].concat()).bold(),
            _ => Line::from([" ", text.as_str(), " "].concat()),
        }
    }
}
impl From<LineButton> for Box<dyn Component> {
    fn from(component: LineButton) -> Self {
        Box::new(component)
    }
}

impl Drawable for LineButton {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let text = self.label.clone().to_string();
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from(["> ", text.as_str(), "  "].concat()).render(area, frame.buffer_mut());
            }
            _ => {
                Line::from(["  ", text.as_str(), "  "].concat())
                    .bold()
                    .render(area, frame.buffer_mut());
            }
        }
    }
}
impl Interactable for LineButton {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        let Some(mut f) = self.on_interact.take() else {
            return Ok(Vec::new());
        };
        let result = f(parent_state);
        self.on_interact = Some(f);
        result
    }
}
