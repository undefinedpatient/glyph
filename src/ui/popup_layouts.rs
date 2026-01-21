use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::{Line, Style, Widget};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Clear, Padding, Paragraph, Wrap};
use crate::app::{PopupConfirmView, PopupView};

pub struct PopupLayout<'a> {
    ref_popup: &'a PopupView
}
impl<'a> PopupLayout<'a> {
    pub fn new(popup: &'a PopupView) -> Self {
        PopupLayout {
            ref_popup: popup
        }
    }
}
impl<'a> Widget for PopupLayout<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        match self.ref_popup {
            PopupView::Confirm(popup_t) => {
                match popup_t {
                    PopupConfirmView::Exit => {
                        let area: Rect = area.centered(Constraint::Length(42), Constraint::Length(6));
                        let paragraph_message: Paragraph = Paragraph::new("Confirm Exit?")
                            .wrap(Wrap {trim:true})
                            .alignment(Alignment::Center)
                            .block(
                                Block::bordered()
                                    .padding(Padding::uniform(1))
                                    .title_top(Line::from("Confirmation").right_aligned())
                                    .title_bottom(Line::from("Confirm (Y) Cancel (n)").right_aligned())
                            );

                        Clear.render(area, buf);
                        paragraph_message.render(area, buf);
                    }
                    _ => {}
                }
            }
            PopupView::Info(message) => {
                let area: Rect = area.centered(Constraint::Length(42), Constraint::Length(6));
                let paragraph_message: Paragraph = Paragraph::new(message.as_str())
                    .wrap(Wrap {trim:true})
                    .alignment(Alignment::Center)
                    .block(
                        Block::bordered()
                            .padding(Padding::uniform(1))
                            .title_top(Line::from("Info").centered())
                            .title_bottom(Line::from("Understood").centered())
                    );

                Clear.render(area, buf);
                paragraph_message.render(area, buf);
            }
            PopupView::Warning(message) => {
                let area: Rect = area.centered(Constraint::Length(48), Constraint::Length(7));
                let paragraph_message: Paragraph = Paragraph::new(message.as_str())
                    .wrap(Wrap {trim:true})
                    .alignment(Alignment::Center)
                    .block(
                        Block::bordered()
                            .style(Style::new().yellow())
                            .padding(Padding::uniform(1))
                            .title_top(Line::from("Warning").yellow().centered())
                            .title_bottom(Line::from("Understood").centered())
                    );

                Clear.render(area, buf);
                paragraph_message.render(area, buf);
            }
            PopupView::Error(message) => {
                let area: Rect = area.centered(Constraint::Length(64), Constraint::Length(8));
                let paragraph_message: Paragraph = Paragraph::new(message.as_str())
                    .wrap(Wrap {trim:true})
                    .alignment(Alignment::Center)
                    .block(
                        Block::bordered()
                            .style(Style::new().red())
                            .padding(Padding::uniform(1))
                            .title_top(Line::from("Error").red().centered())
                            .title_bottom(Line::from("Understood").centered())
                    );

                Clear.render(area, buf);
                paragraph_message.render(area, buf);
            }
            _ => {}
        }
    }
}
