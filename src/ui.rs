mod page_layouts;
mod dialog_layouts;
mod popup_layouts;

use ratatui::style::Stylize;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::Frame;

use crate::app::{App, DialogView, PageView};


pub fn ui(frame: &mut Frame, app: &mut App) {
    if let Some(view) = app.peek_page_ref() {
        match view {
            PageView::Entrance => {
                frame.render_widget(
                    page_layouts::EntrancePageLayout::new(app),
                    frame.area()
                );
            },
            PageView::CreateGlyph => {
                frame.render_widget(
                    page_layouts::CreateGlyphView::new(app),
                    frame.area()
                );
            }
            _ => {}
        }

    }
    if app.state.error_message().is_some() {
        app.push_error_message();
    } else {
        if app.state.warning_message().is_some() {
            app.push_warning_message();
        } else {
            if app.state.info_message().is_some() {
                app.push_info_message();
            }
        }
    }
    if let Some(dialog) = app.peek_dialog_ref() {
        match dialog {
            DialogView::CreateGlyphInfo => {
                frame.render_widget(
                    dialog_layouts::CreateGlyphInfoDialogLayout::new(app), frame.area()
                );
            }
            _ => {}
        }
    }

    if let Some(popup) = app.peek_popup_ref() {
        frame.render_widget(popup_layouts::PopupLayout::new(popup), frame.area());
    }
}

