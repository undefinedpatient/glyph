use color_eyre::eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

mod popups_events;
mod page_events;
mod dialog_events;
use crate::app::{App, MessageLevel, views::*};
use crate::app::views::PopupConfirmView;

pub fn handle_key_events(key: &KeyEvent, app: &mut App) -> () {
    handle_universal_events(key, app);
    if handle_popup_events(key, app) {return};
    if handle_dialog_events(key, app) {return};
    if handle_page_events(key, app) {return};
}
pub fn set_error_to_app(app: &mut App, report: Report) {
    app.state.set_message(report.to_string().as_str(), MessageLevel::ERROR);
}

fn handle_universal_events(key: &KeyEvent, app: &mut App) -> () {
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
}

fn handle_popup_events(key: &KeyEvent, app: &mut App) -> bool {
    if let Some(popup) = app.peek_popup_ref() {
        match popup {
            PopupView::Confirm(popup_t) => {
                let popup_type: PopupConfirmView = popup_t.clone();
                popups_events::handle_confirm(key, &popup_type, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
            PopupView::Info(_) | PopupView::Warning(_) | PopupView::Error(_) => {
                popups_events::hande_simple(key, app).unwrap();
            }
            _ => {}
        }
        return true;
    }
    false
}

fn handle_dialog_events(key: &KeyEvent, app: &mut App) -> bool {
    if let Some(dialog) = app.peek_dialog_ref() {
        match dialog {
            DialogView::CreateGlyphInfo => {
                dialog_events::handle_create_glyph_info(key, app).unwrap_or_else(
                    |report| set_error_to_app(app, report)
                );
            }
        }
        return true
    }
    false
}

fn handle_page_events(key: &KeyEvent, app: &mut App) -> bool {
    if let Some(view) = app.peek_page_ref() {
        match view {
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
        return true;
    }
    false
}