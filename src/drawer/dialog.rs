use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Block, BorderType, Clear, Widget};
use crate::app::dialog::TextInputDialog;
use crate::drawer::{DrawFlag, Drawable};

impl Drawable for TextInputDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let dialog_area: Rect = area.centered(Constraint::Length(64), Constraint::Length(13));
        let dialog_frame = Block::bordered().border_type(
            match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            }
        );
        let inner_dialog_area = dialog_frame.inner(dialog_area);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());
    }
}