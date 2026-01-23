use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, ToSpan};
use ratatui::widgets::{Block, Clear, Padding, Paragraph, Widget, Wrap};
use crate::app::popup::{ExitConfirmPopup, MessagePopup};
use crate::drawer::Drawable;

impl Drawable for MessagePopup {
    fn draw(&self, area: Rect, buf: &mut Buffer) {

    }
}

impl Drawable for ExitConfirmPopup{
    fn draw(&self, area: Rect, buf: &mut Buffer) {
        let area: Rect = area.centered(Constraint::Length(42), Constraint::Length(6));
        let paragraph_message: Paragraph = Paragraph::new("Confirm Exit?")
            .wrap(Wrap {trim:true})
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .title_top(Line::from("Confirmation").centered())
                    .title_bottom(Line::from(
                        if self.focus_index == 0{
                            vec![
                                Span::from("[Cancel]").bold(),
                                Span::from(" Confirm "),
                            ]
                        }else{
                            vec![
                                Span::from(" Cancel "),
                                Span::from("[Confirm]").bold(),
                            ]
                        }
                    ).centered())
            );

        Clear.render(area, buf);
        paragraph_message.render(area, buf);
    }
}