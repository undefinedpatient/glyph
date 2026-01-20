use std::{io, path::PathBuf};

use crossterm::event::{KeyCode, KeyEventKind, KeyEvent};
use color_eyre::eyre::{Error, Ok, Result, Report};
use ratatui::widgets::ListState;


use crate::{app::{App, Popup, PopupConfirmType, View, states::ListStateType}, utils::{create_glyph, get_dir_names}};


pub fn set_error_to_app(app: &mut App, report: Report) {
    app.state.set_error_message(report.to_string().as_str());
}

pub fn handle_key_events(key: &KeyEvent, app: &mut App) -> () {
    // Universal Key that should work across the app
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::F(num) = key.code {
                match num {
                    1 => {
                        app.state.set_info_message("Demo Info Message");
                    }
                    2 => {
                        app.state.set_warning_message("Demo Warning Message");
                    }
                    3 => {
                        set_error_to_app(app,Report::msg("Demo Error Message"));
                    }
                    _ => {}
                }

            }
        }
        _ => {}
    }

    if let Some(popup) = app.peek_popup_ref() {
        match popup {
            Popup::Confirm(popup_t) => {
                let popup_type: PopupConfirmType = popup_t.clone();
                return handle_comfirm_popup(key, &popup_type, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            Popup::Info(_) | Popup::Warning(_) | Popup::Error(_) => {
                return hande_simple_message_popup(key, app).unwrap();
            }
            _ => return (),
        }
    }
    if let Some(view) = app.peek_view_ref() {
        match view {
            View::Entrance => {
                return handle_key_events_entrance_view(key, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            View::CreateGlyph => {
                return handle_key_events_create_glyph_view(key, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            _ => return (),
        }
    }
}

pub fn handle_comfirm_popup(key: &KeyEvent, confirm_type: &PopupConfirmType, app: &mut App) -> Result<()> {
    match confirm_type {
        PopupConfirmType::Exit => {
            match key.kind {
                KeyEventKind::Press=> {
                    if let KeyCode::Char(code) = key.code {
                        match code {
                            'y' => {
                                app.state.set_should_quit(true);
                                return Ok(())
                            },
                            'n' => {
                                app.pop_popup();
                                return Ok(())
                            },
                            _ => return Ok(())
                        }
                    }
                },
                KeyEventKind::Release=> return Ok(()),
                KeyEventKind::Repeat=> return Ok(()),
            }
        }
    }
    Ok(())
}

pub fn hande_simple_message_popup(key: &KeyEvent, app: &mut App) -> Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                match code {
                    'y' | ' ' => {
                        app.pop_popup();
                        return Ok(())
                    },
                    _ => return Ok(())
                }
            }
            if let KeyCode::Enter = key.code {
                app.pop_popup();
                return Ok(())
            }
        },
        KeyEventKind::Release=> return Ok(()),
        KeyEventKind::Repeat=> return Ok(()),

    }
    Ok(())

}

pub fn handle_key_events_create_glyph_view(key: &KeyEvent, app: &mut App) -> Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                match code {
                    'q' => {
                        app.pop_view();
                        return Ok(())
                    },
                    'k' => {
                        app.widget_states.active_list_state_mut().unwrap().select_previous();
                    }
                    'j' => {
                        app.widget_states.active_list_state_mut().unwrap().select_next();
                    }
                    'c' => {
                        let list: Vec<String> = get_dir_names(app.state.get_current_path())?;
                        if let Some(state) = app.widget_states.active_list_state_mut().unwrap().selected() {
                            let selected_dir_name: &String = &(list[state]);
                            let new_path: PathBuf = app.state.get_current_path().join(selected_dir_name);
                            create_glyph(&new_path, "default")?;
                        }
                    }
                    ' ' => {
                        let list: Vec<String> = get_dir_names(app.state.get_current_path())?;
                        let state: &mut ListState = app.widget_states.active_list_state_mut().unwrap();
                        if let Some(index) = state.selected() {
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
                    }
                    _ => return Ok(())
                }
            }
        },
        KeyEventKind::Release=> return Ok(()),
        KeyEventKind::Repeat=> return Ok(()),
    }
    Ok(())

}

pub fn handle_key_events_entrance_view(key: &KeyEvent, app: &mut App) -> Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                match code {
                    'q' => {
                        app.push_popup(Popup::Confirm(PopupConfirmType::Exit));
                        return Ok(());
                    },
                    'a' => {
                        app.push_view(View::CreateGlyph);
                        app.widget_states.set_active_list(ListStateType::CreateGlyph);
                        app.widget_states.active_list_state_mut().unwrap().select_first();
                        return Ok(());
                    },
                    _ => return Ok(())
                }
            }
        },
        KeyEventKind::Release=> return Ok(()),
        KeyEventKind::Repeat=> return Ok(()),
    }
    Ok(())

}