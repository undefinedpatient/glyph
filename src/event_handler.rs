mod popups_events;
mod page_events;

use std::{io, path::PathBuf};

use crossterm::event::{KeyCode, KeyEventKind, KeyEvent};
use color_eyre::eyre::{Error, Ok, Result, Report};
use ratatui::widgets::ListState;


use crate::{app::{App, PopupView, PopupConfirmView, PageView, states::ListStateType}, utils::{create_glyph, get_dir_names}};
use crate::app::{DialogView, PageState};

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
        return match popup {
            PopupView::Confirm(popup_t) => {
                let popup_type: PopupConfirmView = popup_t.clone();
                popups_events::handle_confirm_popup(key, &popup_type, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            PopupView::Info(_) | PopupView::Warning(_) | PopupView::Error(_) => {
                popups_events::hande_simple_message_popup(key, app).unwrap();
            }
            _ => (),
        }
    }
    if let Some(view) = app.peek_page_ref() {
        return match view {
            PageView::Entrance => {
                page_events::handle_key_events_entrance_view(key, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            PageView::CreateGlyph => {
                page_events::handle_key_events_create_glyph_page_view(key, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            _ => (),
        }
    }
}

