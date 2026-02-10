use color_eyre::owo_colors::OwoColorize;
use crate::app::dialog::{ConfirmDialog, NumberInputDialog, TextInputDialog};
use crate::app::widget::LineButton;
use crate::app::Convertible;
use crate::drawer::{get_draw_flag, DrawFlag, Drawable};
use crate::theme::Theme;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph, Widget};
use ratatui::Frame;
use ratatui::style::Stylize;

impl Drawable for TextInputDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .border_style(theme.on_surface())
            .style(theme.on_surface())
            .bg(theme.surface_low())
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
            theme,
        );
    }
}
impl Drawable for ConfirmDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .border_style(theme.on_surface())
            .style(theme.on_surface())
            .bg(theme.surface_low())
            .title("Confirmation Dialog")
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
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let dialog_area: Rect = area.centered(Constraint::Length(42), Constraint::Length(5));
        let dialog_frame = Block::bordered()
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
            })
            .border_style(theme.on_surface())
            .style(theme.on_surface())
            .bg(theme.surface_low())
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
            theme
        );
    }
}
