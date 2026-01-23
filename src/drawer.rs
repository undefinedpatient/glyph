mod page;
mod popup;
mod widget;

use crate::app::Application;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::Frame;
use std::any::Any;

pub trait Drawable{
    fn render(&self, area: Rect, buf: &mut Buffer);

}
pub fn draw(frame: &mut Frame, app: &mut Application) {
    for page in (*app.page_states).iter_mut() {
        page.as_drawable_mut().render(frame.area(), frame.buffer_mut());
        break;

    }
    for popup in (*app.popup_states).iter_mut() {
        popup.as_drawable_mut().render(frame.area(), frame.buffer_mut());

    }

}

