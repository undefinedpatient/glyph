use ratatui::style::Stylize;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::Frame;

mod page_layouts;
mod dialog_layouts;
mod popup_layouts;
mod widget_layouts;
use crate::app::{App, view_type::*, MessageLevel};

pub trait LayoutHandler {
    fn get_layout(&self) -> View;
}




pub fn ui(frame: &mut Frame, app: &mut App) {
    if let Some(view) = app.peek_page_ref() {
        match view {
            PageView::Entrance => {
                page_layouts::EntrancePageLayout::new(app).draw(frame);
            },
            PageView::CreateGlyph => {
                page_layouts::CreateGlyphLayout::new(app).draw(frame);
            }
            _ => {}
        }

    }
    if app.state.message(MessageLevel::ERROR).is_some() {
        app.push_message(MessageLevel::ERROR);
    } else {
        if app.state.message(MessageLevel::WARNING).is_some() {
            app.push_message(MessageLevel::WARNING);
        } else {
            if app.state.message(MessageLevel::INFO).is_some() {
                app.push_message(MessageLevel::INFO);
        }
        }
    }
    if let Some(dialog) = app.peek_dialog_ref() {
        match dialog {
            DialogView::CreateGlyphInfo => {
                dialog_layouts::CreateGlyphInfoDialogLayout::new(app).draw(frame)
            }
            _ => {}
        }
    }

    if let Some(popup) = app.peek_popup_ref() {
        frame.render_widget(popup_layouts::PopupLayout::new(popup), frame.area());
    }
}

