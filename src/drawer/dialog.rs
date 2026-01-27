use crate::app::dialog::CreateGlyphDialog;
use crate::app::widget::LineButton;
use crate::app::Convertible;
use crate::drawer::{get_draw_flag, DrawFlag, Drawable};
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Block, BorderType, Clear, Widget};
use ratatui::Frame;
impl Drawable for CreateGlyphDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .title_bottom(
                (*self.components[0])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hover_index, 1, None))
                    .right_aligned(),
            )
            .title_bottom(
                (*self.components[1])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hover_index, 2, None))
                    .right_aligned(),
            );
        let inner_dialog_area = dialog_frame.inner(dialog_area);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());
        self.containers[0].render(
            frame,
            inner_dialog_area,
            get_draw_flag(
                self.state.hover_index,
                0,
                Some(self.containers[0].is_focused()),
            ),
        );
    }
}
