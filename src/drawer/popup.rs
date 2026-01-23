use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::app::popup::MessagePopup;
use crate::drawer::Drawable;

impl Drawable for MessagePopup {
    fn draw(&self, area: Rect, buf: &mut Buffer) {
        
    }
}