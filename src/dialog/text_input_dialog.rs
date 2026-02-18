use std::any::Any;
use color_eyre::eyre::Result;
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Clear, Widget};
use crate::app::{Command, Component, Container};
use crate::app::Command::PageCommand;
use crate::app::PageCommand::PopDialog;
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::{is_cycle_backward_hover_key, is_cycle_forward_hover_key, Interactable};
use crate::focus_handler::Focusable;
use crate::theme::Theme;
use crate::utils::cycle_offset;
use crate::widget::line_button::LineButton;
use crate::widget::text_field::{TextField, TextFieldState};

pub struct TextInputDialogState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub text_input: String,
}
pub struct TextInputDialog {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: TextInputDialogState,

    pub on_submit: Option<Box<dyn FnOnce(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl TextInputDialog {
    pub fn new(field_title: &str, default: &str, validate: Box<dyn Fn(&str)->bool>) -> Self {
        Self {
            containers: vec![
                TextField::new(
                    field_title,
                    default,
                    validate,
                )
                    .on_exit(
                        Box::new(
                            |parent_state, state| {
                                let _parent_state = parent_state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                let _state = state.unwrap().downcast_mut::<TextFieldState>().unwrap();
                                _parent_state.text_input = _state.chars.iter().collect::<String>();
                                Ok(Vec::new())
                            }
                        )
                    )
                    .into()
            ],
            components: vec![
                LineButton::new("Back").on_interact(Box::new(|_| Ok(vec![PageCommand(PopDialog)]))).into(),
                LineButton::new("Confirm").into(),
            ],
            state: TextInputDialogState {
                is_focused: false,
                hovered_index: None,
                text_input: String::from(default),
            },
            on_submit: None,
        }
    }

    pub fn on_submit(mut self, on_submit:Box<dyn FnOnce(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>) ->Self {
        self.on_submit = Some(on_submit);
        self
    }

    pub fn is_valid_input(&self) -> bool {
        (*self.containers[0]).as_any().downcast_ref::<TextField>().unwrap().state.is_valid
    }

    pub(crate) fn cycle_hover(&mut self, offset: i16) -> () {
        let max: u16 = (self.containers.len() + self.components.len()) as u16;
        if let Some(hover_index) = self.state.hovered_index {
            self.state.hovered_index = Some(cycle_offset(hover_index as u16, offset, max) as usize);
        } else {
            self.state.hovered_index = Some(0);
        }
    }
}
impl From<TextInputDialog> for Box<dyn Container> {
    fn from(container: TextInputDialog) -> Self {
        Box::new(container)
    }
}

impl Drawable for TextInputDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let back_button =
            (*self.components[0])
                .as_any()
                .downcast_ref::<LineButton>()
                .unwrap()
                .as_line(crate::drawer::get_draw_flag(self.state.hovered_index, 1, None))
                .right_aligned();
        let mut submit_button =
            (*self.components[1])
                .as_any()
                .downcast_ref::<LineButton>()
                .unwrap()
                .as_line(crate::drawer::get_draw_flag(self.state.hovered_index, 2, None))
                .right_aligned();

        if !self.is_valid_input() {
            submit_button = submit_button.dim();
        }



        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .border_style(theme.on_surface())
            .style(theme.on_surface())
            .bg(theme.surface_low())
            .title("Text Input Dialog")
            .title_bottom(
                back_button
            )
            .title_bottom(
                submit_button
            );
        let inner_dialog_area = dialog_frame.inner(dialog_area);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());
        self.containers[0].render(
            frame,
            inner_dialog_area,
            crate::drawer::get_draw_flag(
                self.state.hovered_index,
                0,
                Some(self.containers[0].is_focused()),
            ),
            theme,
        );
    }
}
impl Interactable for TextInputDialog {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if self.focused_child_mut().is_none() {
            if is_cycle_forward_hover_key(key) {
                self.cycle_hover(1)
            }
            if is_cycle_backward_hover_key(key) {
                self.cycle_hover(-1);
            }
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![Command::PageCommand(crate::app::PageCommand::PopDialog)]);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            return match index {
                                0 => {
                                    // Text Field
                                    self.containers[0].set_focus(true);
                                    Ok(Vec::new())
                                }
                                1 => {
                                    // Back Button
                                    self.components[0].handle(key, None)
                                }
                                2 => {
                                    // Confirm Button
                                    if ! self.is_valid_input() {
                                        return Ok(Vec::new());
                                    }
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
        } else {
            let index: usize = self.focused_child_index().unwrap();
            let mut result =
                self.containers[index].handle(key, Some(&mut self.state));
            result
        }
    }

}
impl Focusable for TextInputDialog {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }

    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(&**container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        for (index, container) in self.containers.iter().enumerate() {
            if container.is_focused() {
                return Some(index);
            }
        }
        None
    }
}
