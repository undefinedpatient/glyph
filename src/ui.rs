use std::rc::Rc;

use ratatui::layout::{Alignment, Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, StatefulWidget, Widget, Wrap};
use ratatui::Frame;
use tui_big_text::{BigText, PixelSize};

use crate::app::{App, DialogView, PopupView, PopupConfirmType, PageView};
use crate::utils::get_dir_names;

mod page_layouts;
mod dialog_layouts;
mod popup_layouts;

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

