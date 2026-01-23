mod page;
mod popup;
mod widget;

use ratatui::buffer::Buffer;
use crate::app::Application;
use ratatui::style::Stylize;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::Frame;
use ratatui::layout::Rect;

pub trait Drawable {
    fn draw(&self, area: Rect, buf: &mut Buffer);
}
pub fn draw(frame: &mut Frame, app: &mut Application) {
    const MAX_FULLSCREEN: u8 = 1;
    let mut fullscreen_count: u8 = 0;
    for view in (*app.page_states).iter_mut() {
        view.as_drawable_mut().draw(frame.area(), frame.buffer_mut());
        break;
    }
    for popup in (*app.popup_states).iter_mut() {
        popup.as_drawable_mut().draw(frame.area(), frame.buffer_mut());

    }

}

