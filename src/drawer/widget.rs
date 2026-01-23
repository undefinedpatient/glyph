use crate::app::widget::SimpleButton;
use crate::drawer::Drawable;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Widget;

impl Drawable for SimpleButton {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Line::from(self.label.as_str()).centered().render(area, buf);
        
    }
}
// impl Drawable for DirectoryList {
//     fn draw(&self, area: Rect, buf: &mut Buffer) {
//     }
// }