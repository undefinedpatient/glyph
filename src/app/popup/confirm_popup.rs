use crate::app::AppCommand::PopPopup;
use crate::app::Command::AppCommand;
use crate::app::{Command, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::theme::Theme;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::{Line, Span};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Clear, Padding, Paragraph, Widget, Wrap};
use ratatui::Frame;
use std::any::Any;

pub struct ConfirmPopup {
    pub is_focused: bool,
    pub focus_index: usize,
    pub message: String,


    pub on_confirm: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl ConfirmPopup {
    pub fn new(message: &str) -> Self {
        Self {
            is_focused: true,
            focus_index: 0,
            message: String::from(message),

            on_confirm: None,
        }
    }
    pub fn on_confirm(mut self, on_confirm: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>) -> Self {
        self.on_confirm = Some(on_confirm);
        self
    }
}
impl From<ConfirmPopup> for Box<dyn Container>{
    fn from(container: ConfirmPopup) -> Self {
        Box::new(container)
    }
}
impl Drawable for ConfirmPopup {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let area: Rect = area.centered(Constraint::Length(42), Constraint::Length(6));
        let paragraph_message: Paragraph = Paragraph::new(self.message.clone())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(if self.is_focused() {
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .title_top(Line::from("Confirmation").centered())
                    .title_bottom(
                        Line::from(if self.focus_index == 0 {
                            vec![Span::from("[Cancel]").bold(), Span::from(" Confirm ")]
                        } else {
                            vec![Span::from(" Cancel "), Span::from("[Confirm]").bold()]
                        })
                            .centered(),
                    )
                    .border_type(BorderType::Double)
                    .border_style(theme.on_surface())
                    .style(theme.on_surface())
                    .bg(theme.surface_low())
            } else {
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .title_top(Line::from("Confirmation").centered())
                    .title_bottom(
                        Line::from(if self.focus_index == 0 {
                            vec![Span::from("[Cancel]").bold(), Span::from(" Confirm ")]
                        } else {
                            vec![Span::from(" Cancel "), Span::from("[Confirm]").bold()]
                        })
                            .centered(),
                    )
                    .border_style(theme.on_surface())
                    .style(theme.on_surface())
                    .bg(theme.surface_low())
            });

        Clear.render(area, frame.buffer_mut());
        paragraph_message.render(area, frame.buffer_mut());
    }
}
impl Interactable for ConfirmPopup {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        return match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Tab = key.code {
                    self.focus_index = (self.focus_index + 1) % 2;
                }
                if let KeyCode::BackTab = key.code {
                    if self.focus_index == 0 {
                        self.focus_index = 1;
                    } else {
                        self.focus_index -= 1;
                    }
                }
                if let KeyCode::Enter = key.code {
                    if self.focus_index == 0 {
                        return Ok(vec![AppCommand(PopPopup)]);
                    }
                    if self.focus_index == 1 {
                        if let Some(mut function) = self.on_confirm.take() {
                            return (*function)(parent_state);
                        }
                        return Ok(Vec::new());
                    }
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![AppCommand(PopPopup)]);
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        };
    }
    fn keymap(&self) -> Vec<(&str, &str)>{
        [
            ("j/k/up/down/tab/backtab","Navigate"),
            ("Enter","Interact"),
        ].into()
    }
}
impl Focusable for ConfirmPopup {
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
