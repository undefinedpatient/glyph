use crate::app::dialog::TextInputDialog;
use crate::app::Command;
use crate::event_handler::{Focusable, Interactable};
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use rusqlite::fallible_iterator::FallibleIterator;
use std::any::Any;


impl Interactable for TextInputDialog {
    fn handle(
        &mut self,
        key: &KeyEvent,
        data: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if self.focused_child_mut().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![Command::PopDialog]);
                    }
                    if let KeyCode::Tab = key.code {
                        self.cycle_hover(1)
                    }
                    if let KeyCode::BackTab = key.code {
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
                                    if let Some(callback) = self.on_submit.take() {
                                        // Now 'callback' is owned, so we can call it
                                        let input = self.state.text_input.clone();
                                        let callback_result = callback(input, data);
                                        if callback_result.is_err() {
                                            callback_result
                                        } else {
                                            let mut commands = callback_result?;
                                            commands.push(Command::PopDialog);
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
                self.containers[index].handle(key, Some(&mut self.state.text_input));
            result
        }
    }
    
}