use std::rc::Rc;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders};
use tui_big_text::{BigText, PixelSize};
use crate::app::{entrance::Entrance};
use crate::drawer::{DrawType, Drawable};

impl Drawable for Entrance {
    fn draw_type(&self) -> DrawType{
        DrawType::Full
    }
    fn draw(&self, frame: &mut Frame) {
        let title = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec![
                "Glyph".magenta().into(),
            ])
            .alignment(HorizontalAlignment::Center)
            .build();
        let text_actions: Text = Text::from(vec![
            Line::from("Create (a)"),
            Line::from("Open   (o)"),
            Line::from("Quit   (q)"),
        ]).centered();
        let block: Block = Block::default().borders(Borders::ALL);
        let area_inner: Rect = block.inner(frame.area());
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let rects: Rc<[Rect]> = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(3)
        ])
            .flex(Flex::Center)
            .split(rect);
        // Render Section
        frame.render_widget(block, frame.area());
        frame.render_widget(title, rects[0]);
        frame.render_widget(text_actions, rects[1]);
    }
}
