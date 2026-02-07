use crate::app::dialog::{ConfirmDialog, NumberInputDialog, TextInputDialog};
use crate::app::{Command, PageCommand};
use crate::event_handler::{is_cycle_backward_hover_key, is_cycle_forward_hover_key, Focusable, Interactable};
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use rusqlite::fallible_iterator::FallibleIterator;
use std::any::Any;

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
                        return Ok(vec![Command::PageCommand(PageCommand::PopDialog)]);
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
                                    if let Some(on_submit) = self.on_submit.take() {
                                        let callback_result = on_submit(parent_state, Some(&mut self.state));
                                        if callback_result.is_err() {
                                            callback_result
                                        } else {
                                            let mut commands = callback_result?;
                                            commands.push(Command::PageCommand(PageCommand::PopDialog));
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

impl Interactable for ConfirmDialog {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    return Ok(vec![Command::PageCommand(PageCommand::PopDialog)]);
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
                                        commands.push(Command::PageCommand(PageCommand::PopDialog));
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
impl Interactable for NumberInputDialog {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if self.focused_child_mut().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![Command::PageCommand(PageCommand::PopDialog)]);
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
                                    if let Some(on_submit) = self.on_submit.take() {
                                        let callback_result = on_submit(parent_state, Some(&mut self.state));
                                        if callback_result.is_err() {
                                            callback_result
                                        } else {
                                            let mut commands = callback_result?;
                                            commands.push(Command::PageCommand(PageCommand::PopDialog));
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

