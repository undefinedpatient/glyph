
use color_eyre::eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

mod popups_events;
mod page_events;
mod dialog_events;
use crate::app::{App, DialogView, MessageLevel, PageView, PopupConfirmView, PopupView};

pub fn set_error_to_app(app: &mut App, report: Report) {
    app.state.set_message(report.to_string().as_str(), MessageLevel::ERROR);
}

pub fn handle_key_events(key: &KeyEvent, app: &mut App) -> () {
    // Universal Key that should work across the app
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::F(num) = key.code {
                match num {
                    1 => {
                        app.state.set_message("Demo Info Message", MessageLevel::INFO);
                    }
                    2 => {
                        app.state.set_message("Demo Warning Message", MessageLevel::WARNING);
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
                popups_events::handle_confirm(key, &popup_type, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            PopupView::Info(_) | PopupView::Warning(_) | PopupView::Error(_) => {
                popups_events::hande_simple(key, app).unwrap();
            }
            _ => (),
        }
    }
    if let Some(dialog) = app.peek_dialog_ref() {
        return match dialog {
            DialogView::CreateGlyphInfo => {
                dialog_events::handle_create_glyph_info(key, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
        }
    }
    if let Some(view) = app.peek_page_ref() {
        return match view {
            PageView::Entrance => {
                page_events::handle_entrance(key, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            PageView::CreateGlyph => {
                page_events::handle_create_glyph_page(key, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            _ => (),
        }
    }
}

