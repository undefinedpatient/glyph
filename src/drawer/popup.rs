use crate::app::popup::{ExitConfirmPopup, MessagePopup};
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Focusable;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Clear, Padding, Paragraph, Widget, Wrap};
use ratatui::Frame;
impl Drawable for MessagePopup {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let ratio: f64 = 1.0/2.0;
        let width: u16 =  (((self.message.len().isqrt() as f64 + 1f64)  / ratio) as u16).clamp(42, area.width)+6;
        let height: u16 =  (((self.message.len().isqrt() as f64 + 1f64) * ratio) as u16).clamp(6, area.height)+6;
        let popup_area: Rect = area.centered(Constraint::Length(width), Constraint::Length(height));
        let paragraph_message: Paragraph = Paragraph::new(self.message.clone())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(if self.is_focused() {
                Block::bordered()
                    .border_type(BorderType::Double)
                    .padding(Padding::uniform(1))
                    .title_top(Line::from(" Message ").centered())
                    .title_bottom(Line::from(Span::from(" [Understood] ").bold()).centered())
                    .style(self.color)
            } else {
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .title_top(Line::from(" Message ").centered())
                    .title_bottom(Line::from(Span::from(" [Understood] ").bold()).centered())
                    .style(self.color)
            });
        Clear.render(popup_area, frame.buffer_mut());
        paragraph_message.render(popup_area, frame.buffer_mut());
    }
}

impl Drawable for ExitConfirmPopup {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let area: Rect = area.centered(Constraint::Length(42), Constraint::Length(6));
        let paragraph_message: Paragraph = Paragraph::new("Confirm Exit?")
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(if self.is_focused() {
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .title_top(Line::from("Confirmation").centered())
                    .title_bottom(
                        Line::from(if self.focus_index == 0 {
                            vec![Span::from("[Cancel]").bold(), Span::from(" Confirm ")]
                        } else {
                            vec![Span::from(" Cancel "), Span::from("[Confirm]").bold()]
                        })
                        .centered(),
                    )
                    .border_type(BorderType::Double)
            } else {
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .title_top(Line::from("Confirmation").centered())
                    .title_bottom(
                        Line::from(if self.focus_index == 0 {
                            vec![Span::from("[Cancel]").bold(), Span::from(" Confirm ")]
                        } else {
                            vec![Span::from(" Cancel "), Span::from("[Confirm]").bold()]
                        })
                        .centered(),
                    )
            });

        Clear.render(area, frame.buffer_mut());
        paragraph_message.render(area, frame.buffer_mut());
    }
}
