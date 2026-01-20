use std::{io, path::PathBuf};

use crate::{app::{App, ListType, Popup, PopupConfirmType, View}, utils::{create_glyph, get_dir_names}};
use crossterm::event::{KeyCode, KeyEventKind, KeyEvent};
use color_eyre::eyre::{Error, Ok, Result, Report};
use ratatui::widgets::ListState;
pub fn set_error_to_app(app: &mut App, report: Report) {
    app.set_error_message(report.to_string().as_str());
}
pub fn handle_key_events(key: &KeyEvent, app: &mut App) -> () {
    // Universal Key that should work across the app
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::F(num) = key.code {
                match num {
                    1 => {
                        app.set_info_message("Demo Info Message");
                    }
                    2 => {
                        app.set_warning_message("Demo Warning Message");
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
                return handle_comfirm_popup(key, &popup_t.clone(), app).unwrap_or_else(
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
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                match code {
                    'y' => {
                        app.set_should_quit(true);
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
                        app.focused_list_state_mut().unwrap().select_previous();
                    }
                    'j' => {
                        app.focused_list_state_mut().unwrap().select_next();
                    }
                    ' ' => {
                        let list: Vec<String> = get_dir_names(app.get_current_path())?;
                        if let Some(state) = app.focused_list_state_mut().unwrap().selected() {
                            let selected_dir_name: &String = &(list[state]);
                            let new_path: PathBuf = app.get_current_path().join(selected_dir_name);
                            create_glyph(&new_path)?;
                        }
                    }
                    _ => return Ok(())
                }
            }
            if let KeyCode::Enter = key.code {
                let list: Vec<String> = get_dir_names(app.get_current_path())?;
                let state: &mut ListState = app.focused_list_state_mut().unwrap();
                if let Some(index) = state.selected() {
                    if index == 0 {
                        let parent_path: PathBuf = app.get_current_path().parent().unwrap_or(app.get_current_path()).to_path_buf();
                        app.set_current_path(&parent_path);
                        return Ok(());
                    }
                    let selected_dir_name: &String = &(list[index]);
                    let new_path: PathBuf = app.get_current_path().join(selected_dir_name);
                    app.set_current_path(&new_path);
                }
                return Ok(());
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
                        app.set_focusd_list(ListType::CreateGlyph);
                        app.focused_list_state_mut().unwrap().select_first();
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