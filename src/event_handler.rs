use std::{io, path::PathBuf};

use crate::{app::{App, ListType, Popup, PopupConfirmType, View}, utils::{create_glyph, get_dir_names}};
use crossterm::event::{KeyCode, KeyEventKind, KeyEvent};
use color_eyre::eyre::{Ok, Result};

pub fn handle_key_events(key: &KeyEvent, app: &mut App) -> Result<()> {
    if let Some(popup) = app.peek_popup() {
        match popup {
            Popup::Confirm(popup_t) => {
                return handle_comfirm_popup(key, &popup_t.clone(), app);
            }
            _ => return Ok(()),
        }
    }
    if let Some(view) = app.peek_view() {
        match view {
            View::Entrance => {
                return handle_key_events_entrance_view(key, app);
            }
            View::CreateGlyph => {
                return handle_key_events_create_glyph_view(key, app);
            }
            _ => return Ok(()),
        }
    }
    Ok(())
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
                    _ => return Ok(())
                }
            }
            if let KeyCode::Backspace = key.code {
                let new_path: PathBuf = app.get_current_path().parent().unwrap().to_path_buf();
                app.set_current_path(&new_path);
                return Ok(());
            }
            if let KeyCode::Enter = key.code {
                let list = get_dir_names(app.get_current_path())?;
                let state = app.focused_list_state_mut().unwrap();
                if let Some(state) = state.selected() {
                    let selected_dir_name: &String = &(list[state]);
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