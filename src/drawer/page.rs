use crate::app::page::{CreateGlyphPage, EntrancePage};
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Focusable;
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Widget};
use ratatui::Frame;
use std::rc::Rc;
use tui_big_text::{BigText, PixelSize};

impl Drawable for EntrancePage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
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
        block.render(area, frame.buffer_mut());
        title.render(rects[0], frame.buffer_mut());
        for (i, button_interactable) in (&self.components).iter().enumerate() {
            if let Some(ci) = self.hover_index {
                if i == ci {
                    button_interactable.render(frame,button_rects[i], DrawFlag::HIGHLIGHTING);
                } else {
                    button_interactable.render(frame, button_rects[i], DrawFlag::DEFAULT);
                }
            }
            button_interactable.render(frame, button_rects[i], DrawFlag::DEFAULT);
        }
    }
}
impl Drawable for CreateGlyphPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Outer Frame
        */
        let page_frame: Block = match draw_flag {
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
        let inner_area: Rect = page_frame.inner(area);
        page_frame.render(area, frame.buffer_mut());
        let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Max(3)])
            .flex(Flex::Center)
            .spacing(3)
            .split(inner_area);

        let file_explorer_area = chunks[0].centered(Constraint::Max(42), Constraint::Min(42));
        let button_areas = Layout::horizontal([Constraint::Max(20), Constraint::Max(20)])
            .flex(Flex::Center)
            .split(chunks[1]);
        self.containers[0].render(
            frame,
            file_explorer_area,
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
        self.components[0].render(
            frame,
            button_areas[0],
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
        self.components[1].render(
            frame,
            button_areas[1],
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
