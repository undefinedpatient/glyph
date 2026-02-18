use std::any::Any;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::{Color, Line, Span};
use color_eyre::eyre::Result;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Clear, Padding, Paragraph, Widget, Wrap};
use crate::app::{Command, Container};
use crate::app::AppCommand::PopPopup;
use crate::app::Command::AppCommand;
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Interactable;
use crate::focus_handler::Focusable;
use crate::theme::Theme;

pub struct MessagePopup {
    pub is_focused: bool,
    pub color: Color,
    pub message: String,

    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl MessagePopup {
    pub fn new(message: &str, color: Color) -> Self {
        Self {
            is_focused: true,
            color,
            message: String::from(message),
            on_exit: None,
        }
    }
    pub fn on_exit(mut self, on_exit: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>) -> Self {
        self.on_exit = Some(on_exit);
        self
    }
}
impl From<MessagePopup> for Box<dyn Container>{
    fn from(container: MessagePopup) -> Self {
        Box::new(container)
    }
}
impl Drawable for MessagePopup {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let ratio: f64 = 1.0/2.0;
        let width: u16 =  (((self.message.len().isqrt() as f64 + 1f64)  / ratio) as u16).clamp(42, area.width)+6;
        let height: u16 =  (((self.message.len().isqrt() as f64 + 1f64) * ratio) as u16).clamp(6, area.height)+6;
        let popup_area: Rect = area.centered(Constraint::Length(width), Constraint::Length(height));
        let paragraph_message: Paragraph = Paragraph::new(self.message.clone())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(if self.is_focused() {
                Block::bordered()
                    .border_type(BorderType::Double)
                    .padding(Padding::uniform(1))
                    .title_top(Line::from(" Message ").centered())
                    .title_bottom(Line::from(Span::from(" [Understood] ").bold()).centered())
                    .border_style(theme.on_surface())
                    .style(theme.on_surface())
                    .bg(theme.surface_low())
            } else {
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .title_top(Line::from(" Message ").centered())
                    .title_bottom(Line::from(Span::from(" [Understood] ").bold()).centered())
                    .border_style(theme.on_surface())
                    .style(theme.on_surface())
                    .bg(theme.surface_low())
            });
        Clear.render(popup_area, frame.buffer_mut());
        paragraph_message.render(popup_area, frame.buffer_mut());
    }
}

impl Interactable for MessagePopup {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        return match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Enter = key.code {
                    return Ok(vec![AppCommand(PopPopup)]);
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![AppCommand(PopPopup)]);
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        };
    }
}
impl Focusable for MessagePopup {
    fn is_focused(&self) -> bool {
        self.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}
