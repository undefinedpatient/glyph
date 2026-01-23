use crate::app::page::Entrance;
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, Widget};
use ratatui::Frame;
use std::rc::Rc;
use ratatui::buffer::Buffer;
use tui_big_text::{BigText, PixelSize};
use crate::app::widget::SimpleButton;
use crate::drawer::Drawable;
use crate::event_handler::Focusable;

impl Drawable for Entrance {
    fn draw(&self, area: Rect, buf: &mut Buffer)
    {
        let title = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec![
                "Glyph".magenta().into(),
            ])
            .alignment(HorizontalAlignment::Center)
            .build();
        let block: Block;
        if self.is_focused() {
            block = Block::default().borders(Borders::ALL).border_type(BorderType::Double);
        }else{
            block = Block::default().borders(Borders::ALL);
        }
        let area_inner: Rect = block.inner(area);
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let rects: Rc<[Rect]> = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(3)
        ])
            .flex(Flex::Center)
            .split(rect);
        let button_rects = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1)
        ]).split(rects[1]);
        // Render Section
        block.render(area, buf);
        title.render(rects[0], buf);
        self.buttons[0].draw(button_rects[0], buf);
        self.buttons[1].draw(button_rects[1], buf);
        self.buttons[2].draw(button_rects[2], buf);
        // text_actions.render(rects[1], buf);
    }
}
