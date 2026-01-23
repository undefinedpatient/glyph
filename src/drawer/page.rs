use crate::app::page::{CreateGlyphPage, EntrancePage};
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Focusable;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Widget};
use std::rc::Rc;
use tui_big_text::{BigText, PixelSize};

impl Drawable for EntrancePage {
    fn render(&self, area: Rect, buf: &mut Buffer, draw_flag: DrawFlag) {
        /*
           Container Frame
        */
        let block: Block = match draw_flag {
            DrawFlag::DEFAULT => Block::bordered(),
            DrawFlag::HIGHLIGHTING => Block::bordered()
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
            DrawFlag::FOCUSED => Block::bordered()
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        };
        /*
          Title
        */
        let title = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec!["Glyph".magenta().into()])
            .alignment(HorizontalAlignment::Center)
            .build();
        /*

        */
        let area_inner: Rect = block.inner(area);
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let rects: Rc<[Rect]> = Layout::vertical([Constraint::Length(8), Constraint::Length(3)])
            .flex(Flex::Center)
            .split(rect);
        let button_rects = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(rects[1]);
        // Render Section
        block.render(area, buf);
        title.render(rects[0], buf);
        for (i, button_interactable) in (&self.elements).iter().enumerate() {
            if let Some(ci) = self.hover_index {
                if i == ci {
                    button_interactable.render(button_rects[i], buf, DrawFlag::HIGHLIGHTING);
                } else {
                    button_interactable.render(button_rects[i], buf, DrawFlag::DEFAULT);
                }
            }
            button_interactable.render(button_rects[i], buf, DrawFlag::DEFAULT);
        }
    }
}
impl Drawable for CreateGlyphPage {
    fn render(&self, area: Rect, buf: &mut Buffer, draw_flag: DrawFlag) {
        /*
           Outer Frame
        */
        let frame: Block = match draw_flag {
            DrawFlag::DEFAULT => Block::bordered()
                .title("Create glyph page"),
            DrawFlag::HIGHLIGHTING => Block::bordered()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title("Create glyph page"),
            DrawFlag::FOCUSED => Block::bordered()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title("Create glyph page"),
        };
        /*
           Chucks
        */
        let inner_area: Rect = frame.inner(area);
        frame.render(area, buf);
        let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Max(3)])
            .flex(Flex::Center)
            .spacing(3)
            .split(inner_area);

        let file_explorer_area = chunks[0].centered(Constraint::Max(42), Constraint::Min(42));
        let button_areas = Layout::horizontal([Constraint::Max(20), Constraint::Max(20)])
            .flex(Flex::Center)
            .split(chunks[1]);
        self.containers[0].render(
            file_explorer_area,
            buf,
            if let Some(index) = self.hover_index {
                if index == 0 {
                    if self.containers[0].is_focused() {
                        DrawFlag::FOCUSED
                    } else {
                        DrawFlag::HIGHLIGHTING
                    }
                } else {
                    DrawFlag::DEFAULT
                }
            } else {
                DrawFlag::DEFAULT
            },
        );
        self.elements[0].render(
            button_areas[0],
            buf,
            if let Some(index) = self.hover_index {
                if index == 1 {
                    DrawFlag::HIGHLIGHTING
                } else {
                    DrawFlag::DEFAULT
                }
            } else {
                DrawFlag::DEFAULT
            },
        );
        self.elements[1].render(
            button_areas[1],
            buf,
            if let Some(index) = self.hover_index {
                if index == 2 {
                    DrawFlag::HIGHLIGHTING
                } else {
                    DrawFlag::DEFAULT
                }
            } else {
                DrawFlag::DEFAULT
            },
        );
    }
}
