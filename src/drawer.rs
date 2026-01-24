mod page;
mod popup;
mod widget;
mod dialog;

use crate::app::Application;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::Frame;
use std::any::Any;
pub enum DrawFlag {
    DEFAULT = 0b0000_0000,
    HIGHLIGHTING = 0b0000_0001,
    FOCUSED = 0b0000_0010,
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
