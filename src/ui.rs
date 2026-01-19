use std::rc::Rc;

use ratatui::symbols::block;
use ratatui::{Frame, text};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget, Wrap};
use ratatui::layout::{self, Alignment, Constraint, Direction, Flex, HorizontalAlignment, Layout, Rect};
use tui_big_text::{BigText, PixelSize};

use crate::app::{App, Popup, PopupConfirmType, View};
use crate::{utils::get_file_names};

pub fn ui(frame: &mut Frame, app: &App) {
    if let Some(view) = app.peek_view() {
        match view {
            View::Entrance => {
                frame.render_widget(
                    EntranceView::new(app),
                    frame.area()
                );
            },
        }
    }
    if let Some(popup) = app.peek_popup() {
        frame.render_widget(    PopupWidget::new(popup), frame.area());
    }
}

// Widget itself must not own any recources, and never outlive the AppState it references to.
struct EntranceView<'a>{
    app: &'a App
}
impl<'a> EntranceView<'a>{
    fn new(app: &'a App) -> Self {
        EntranceView { 
            app: app
        }
    }
}
impl<'a> Widget for EntranceView<'a>{
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {

        // Widget/Data Section
        let title = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec![
                "Glyph".magenta().into(),
            ])
            .alignment(HorizontalAlignment::Center)
            .build();
        let text_actions: Text = Text::from(vec![
            Line::from("Create (A)"),
            Line::from("Open   (O)"),
            Line::from("Quit   (Q)"),
        ]).centered();
        let block: Block = Block::default().borders(Borders::ALL);
        let area_inner: Rect = block.inner(area);
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let rects: Rc<[Rect]> = Layout::vertical([
                Constraint::Length(8),
                Constraint::Length(3)
            ])
            .flex(Flex::Center)
            .split(rect);
        // Render Section
        block.render(area, buf);
        title.render(rects[0], buf);
        text_actions.render(rects[1], buf);

    }
}

struct PopupWidget<'a> {
    popup: &'a Popup
}
impl<'a> PopupWidget<'a> {
    fn new(popup: &'a Popup) -> Self {
        PopupWidget {
            popup: popup
        }
    }
}
impl<'a> Widget for PopupWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        match self.popup {
            Popup::Confirm(popup_t) => {
                match popup_t {
                    PopupConfirmType::Exit => {
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
                        // frame.render(area, buf);
                        paragraph_message.render(area, buf);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

