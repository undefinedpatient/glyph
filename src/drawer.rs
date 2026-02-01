mod dialog;
mod page;
mod popup;
mod widget;

use crate::app::Application;
use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::Frame;
use std::any::Any;

pub enum DrawFlag {
    DEFAULT = 0b0000_0000,
    HIGHLIGHTING = 0b0000_0001,
    FOCUSED = 0b0000_0010,
    // DISABLED = 0b0000_0100,
}
pub trait Drawable {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag);
}
pub fn draw(frame: &mut Frame, app: &mut Application) {
    for page in (*app.page_states).iter_mut().rev() {
        page.as_drawable_mut()
            .render(frame, frame.area(), DrawFlag::DEFAULT);
        break;
    }
    for popup in (*app.popup_states).iter_mut() {
        popup
            .as_drawable_mut()
            .render(frame, frame.area(), DrawFlag::DEFAULT);
    }
}

/*
   Helper Functions
*/

// Get draw flag for components/containers.
fn get_draw_flag(
    current_hover_index: Option<usize>,
    widget_index: usize,
    focused: Option<bool>,
) -> DrawFlag {
    if let Some(should_focus) = focused {
        if should_focus {
            return DrawFlag::FOCUSED;
        }
    }
    if let Some(index) = current_hover_index {
        if index == widget_index {
            DrawFlag::HIGHLIGHTING
        } else {
            DrawFlag::DEFAULT
        }
    } else {
        DrawFlag::DEFAULT
    }
}
