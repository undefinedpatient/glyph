use color_eyre::owo_colors::OwoColorize;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};
use crate::app::widget::SimpleButton;
use crate::drawer::Drawable;
use crate::event_handler::Focusable;

impl Drawable for SimpleButton {
    fn draw(&self, area: Rect, buf: &mut Buffer) {
  
        if self.is_highlighted {
            Line::from(
                [
                    "[",
                    self.label.as_str(),
                    "]"
                ].concat()
            ).bold().centered().render(area, buf);
        } else {
            Line::from(self.label.as_str()).centered().render(area, buf);
        }
    }
}