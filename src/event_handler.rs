use std::io;

use crate::app::{App, View, Popup, PopupConfirmType};
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
                return handle_key_events_entrance(key, app);
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
pub fn handle_key_events_entrance(key: &KeyEvent, app: &mut App) -> Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                match code {
                    'q' => {
                        app.push_popup(Popup::Confirm(PopupConfirmType::Exit));
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