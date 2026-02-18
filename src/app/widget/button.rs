use crate::app::{Command, Component, DrawFlag, Drawable, Interactable};
use crate::theme::Theme;
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use ratatui::style::Stylize;
use ratatui::widgets::Widget;
use ratatui::Frame;
use std::any::Any;

pub struct Button {
    pub label: String,

    pub on_interact: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl Button {
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
}
impl From<Button> for Box<dyn Component> {
    fn from(component: Button) -> Self {
        Box::new(component)
    }
}

impl Drawable for Button {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from(["> ", self.label.as_str(), "  "].concat())
                    .bold()
                    .centered()
                    .render(area, frame.buffer_mut());
            }
            _ => {
                Line::from(self.label.as_str())
                    .centered()
                    .render(area, frame.buffer_mut());
            }
        }
    }
}
impl Interactable for Button {
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
