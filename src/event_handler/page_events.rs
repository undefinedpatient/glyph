use crate::app::{App, DialogState, DialogView, PageState, PageView, PopupConfirmView, PopupView, TextFieldState};
use crate::app::widgets::ButtonState;
use crate::utils::get_dir_names;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::ListState;
use std::path::PathBuf;
use color_eyre::eyre::Result;

pub fn handle_create_glyph_page(key: &KeyEvent, app: &mut App) -> Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                match code {
                    'q' => {
                        app.pop_page();
                        return Ok(())
                    },
                    'k' => {
                        if let Some(state) = (&mut *app).h_page_states.get_mut(&PageView::CreateGlyph) {
                            match state {
                                PageState::CreateGlyph{ list_state: list_directory } => {
                                    list_directory.select_previous();
                                },
                                _ => {

                                }

                            }
                        };
                    }
                    'j' => {
                        if let Some(state) = (&mut *app).h_page_states.get_mut(&PageView::CreateGlyph) {
                            match state {
                                PageState::CreateGlyph{ list_state: list_directory } => {
                                    list_directory.select_next();
                                },
                                _ => {

                                }

                            }
                        };
                    }
                    'c' => {
                        if let Some(states) = app.h_dialog_states.get_mut(&DialogView::CreateGlyphInfo) {
                        } else {
                            app.h_dialog_states.insert(
                                DialogView::CreateGlyphInfo,
                                DialogState::CreateGlyphInfo {
                                    text_field_state: TextFieldState::new(),
                                    button_state: ButtonState::new(),
                                }
                            );
                        }
                        app.push_dialog(DialogView::CreateGlyphInfo);
                    }
                    ' ' => {
                        let app_path: PathBuf = app.state.get_current_path().clone();
                        if let Some(state) = (&mut *app).h_page_states.get_mut(&PageView::CreateGlyph) {
                            match state {
                                PageState::CreateGlyph{ list_state: list_directory } => {
                                    let list: Vec<String> = get_dir_names(&app_path)?;
                                    if let Some(index) = list_directory.selected() {
                                        if index == 0 {
                                            let parent_path: PathBuf = app.state.get_current_path().parent().unwrap_or(app.state.get_current_path()).to_path_buf();
                                            app.state.set_current_path(&parent_path);
                                            return Ok(());
                                        }
                                        let selected_dir_name: &String = &(list[index]);
                                        let new_path: PathBuf = app.state.get_current_path().join(selected_dir_name);
                                        app.state.set_current_path(&new_path);
                                    }
                                    return Ok(());
                                },
                                _ => {

                                }

                            }
                        };


                    }
                    _ => return Ok(())
                }
            }
        },
        KeyEventKind::Release=> return Ok(()),
        KeyEventKind::Repeat=> return Ok(()),
    }
    color_eyre::eyre::Ok(())

}

pub fn handle_entrance(key: &KeyEvent, app: &mut App) -> Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                return match code {
                    'q' => {
                        app.push_popup(PopupView::Confirm(PopupConfirmView::Exit));
                        color_eyre::eyre::Ok(())
                    },
                    'a' => {
                        app.push_page(PageView::CreateGlyph);
                        // Check if the state exist
                        if let Some(states) = app.h_page_states.get_mut(&PageView::CreateGlyph) {
                            color_eyre::eyre::Ok(())
                        } else {
                            app.h_page_states.insert(PageView::CreateGlyph, PageState::CreateGlyph { list_state: ListState::default() });
                            color_eyre::eyre::Ok(())
                        }
                    },
                    _ => color_eyre::eyre::Ok(())
                }
            }
        },
        KeyEventKind::Release=> return color_eyre::eyre::Ok(()),
        KeyEventKind::Repeat=> return color_eyre::eyre::Ok(()),
    }
    color_eyre::eyre::Ok(())

}
