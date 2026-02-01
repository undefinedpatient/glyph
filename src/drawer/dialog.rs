use crate::app::dialog::{ConfirmDialog, EditGlobalLayoutSizeDialog, NumberInputDialog, TextInputDialog};
use crate::app::widget::LineButton;
use crate::app::Convertible;
use crate::drawer::{get_draw_flag, DrawFlag, Drawable};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph, Widget};
use ratatui::Frame;
impl Drawable for TextInputDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .title("Text Input Dialog")
            .title_bottom(
                (*self.components[0])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 1, None))
                    .right_aligned(),
            )
            .title_bottom(
                (*self.components[1])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 2, None))
                    .right_aligned(),
            );
        let inner_dialog_area = dialog_frame.inner(dialog_area);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());
        self.containers[0].render(
            frame,
            inner_dialog_area,
            get_draw_flag(
                self.state.hovered_index,
                0,
                Some(self.containers[0].is_focused()),
            ),
        );
    }
}
impl Drawable for ConfirmDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .title("Confirm Dialog")
            .title_bottom(
                (*self.components[0])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 0, None))
                    .right_aligned(),
            )
            .title_bottom(
                (*self.components[1])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 1, None))
                    .right_aligned(),
            );
        let inner_dialog_area = dialog_frame.inner(dialog_area);
        let paragraph = Paragraph::new(self.message.clone()).alignment(Alignment::Center);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());
        paragraph.render(inner_dialog_area, frame.buffer_mut());
    }
}
impl Drawable for NumberInputDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .title("Number Input Dialog")
            .title_bottom(
                (*self.components[0])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 1, None))
                    .right_aligned(),
            )
            .title_bottom(
                (*self.components[1])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 2, None))
                    .right_aligned(),
            );
        let inner_dialog_area = dialog_frame.inner(dialog_area);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());
        self.containers[0].render(
            frame,
            inner_dialog_area,
            get_draw_flag(
                self.state.hovered_index,
                0,
                Some(self.containers[0].is_focused()),
            ),
        );
    }
}
impl Drawable for EditGlobalLayoutSizeDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(8));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .title("Number Input Dialog")
            .title_bottom(
                (*self.components[0])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 1, None))
                    .right_aligned(),
            )
            .title_bottom(
                (*self.components[1])
                    .as_any()
                    .downcast_ref::<LineButton>()
                    .unwrap()
                    .as_line(get_draw_flag(self.state.hovered_index, 2, None))
                    .right_aligned(),
            );
        let inner_dialog_area = dialog_frame.inner(dialog_area);
        let input_field_areas = Layout::vertical(
            [
                Constraint::Length(3),
                Constraint::Length(3),
            ]
        ).split(inner_dialog_area);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());
        self.containers[0].render(
            frame,
            input_field_areas[0],
            get_draw_flag(
                self.state.hovered_index,
                0,
                Some(self.containers[0].is_focused()),
            ),
        );
        self.containers[1].render(
            frame,
            input_field_areas[1],
            get_draw_flag(
                self.state.hovered_index,
                1,
                Some(self.containers[1].is_focused()),
            ),
        );
    }
}
