use crate::app::widget::line_button::LineButton;
use crate::app::Command::PageCommand;
use crate::app::PageCommand::PopDialog;
use crate::app::{get_draw_flag, is_cycle_backward_hover_key, is_cycle_forward_hover_key, Command, Component, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::theme::Theme;
use crate::utils::cycle_offset;
use color_eyre::eyre::Result;
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Clear, Paragraph, Widget};
use ratatui::Frame;
use std::any::Any;

pub struct ConfirmDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
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
impl Drawable for ConfirmDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .border_style(theme.on_surface())
            .style(theme.on_surface())
            .bg(theme.surface_low())
            .title("Confirmation Dialog")
            .title_bottom(
                (*self.components[0])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 0, None))
                    .right_aligned(),
            )
            .title_bottom(
                (*self.components[1])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 1, None))
                    .right_aligned(),
            );
        let inner_dialog_area = dialog_frame.inner(dialog_area);
        let paragraph = Paragraph::new(self.message.clone()).alignment(Alignment::Center);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());
        paragraph.render(inner_dialog_area, frame.buffer_mut());
    }
}
impl Interactable for ConfirmDialog {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    return Ok(vec![Command::PageCommand(crate::app::PageCommand::PopDialog)]);
                }
                if is_cycle_forward_hover_key(key) {
                    self.cycle_hover(1)
                }
                if is_cycle_backward_hover_key(key) {
                    self.cycle_hover(-1);
                }
                if let KeyCode::Enter = key.code {
                    if let Some(index) = self.state.hovered_index {
                        return match index {
                            0 => {
                                // Back Button
                                self.components[0].handle(key, None)
                            }
                            1 => {
                                // Confirm Button
                                if let Some(on_submit) = self.on_submit.take() {
                                    let callback_result = on_submit(parent_state, Some(&mut self.state));
                                    if callback_result.is_err() {
                                        callback_result
                                    } else {
                                        let mut commands = callback_result?;
                                        commands.push(Command::PageCommand(crate::app::PageCommand::PopDialog));
                                        Ok(commands)
                                    }
                                } else {
                                    Err(Report::msg("Submit has already been called!"))
                                }
                            }
                            _ => Ok(Vec::new()),
                        };
                    }
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        }
    }

}
impl Focusable for ConfirmDialog {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
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
