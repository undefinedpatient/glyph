use crate::app::widget::{DirectoryList, SimpleButton, TextField};
use crate::drawer::{DrawFlag, Drawable};
use crate::utils::get_dir_names;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Offset, Position, Rect};
use ratatui::prelude::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Widget};

impl Drawable for SimpleButton {
    fn render(&self, area: Rect, buf: &mut Buffer, draw_flag: DrawFlag) {
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from(["[", self.label.as_str(), "]"].concat())
                    .bold()
                    .centered()
                    .render(area, buf);
            }
            _ => {
                Line::from(self.label.as_str()).centered().render(area, buf);
            }
        }
    }
}
impl Drawable for DirectoryList {
    fn render(&self, area: Rect, buf: &mut Buffer, draw_flag: DrawFlag) {
        /*
           Container Frame
        */
        let current_path: String = (&self.current_path)
            .clone()
            .to_str()
            .unwrap_or("Invalid Path")
            .to_string();
        let frame: Block = match draw_flag {
            DrawFlag::DEFAULT => Block::default()
                .borders(Borders::ALL)
                .title(self.label.as_str())
                .title_bottom(current_path.as_str()),
            DrawFlag::HIGHLIGHTING => Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(self.label.as_str())
                .title_bottom(current_path.as_str()),
            DrawFlag::FOCUSED => Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(self.label.as_str())
                .title_bottom(current_path.as_str()),
        };
        /*
           Directory Widget
        */
        let inner_area: Rect = frame.inner(area);
        frame.render(area, buf);
        let list_items: Vec<Line> = get_dir_names(&self.current_path)
            .unwrap_or(Vec::new())
            .iter()
            .enumerate()
            .map(|(i, item)| {
                if let Some(index) = self.hover_index {
                    if index == i {
                        return Line::from(String::from("> ") + &*item.clone()).bold();
                    }
                }
                return Line::from(String::from("  ") + &*item.clone());
            })
            .collect();

        for (i, line) in list_items[self.offset..].iter().enumerate() {
            if i*self.line_height >= inner_area.height as usize {
                break;
            }
            line.render(
                inner_area.offset(Offset {
                    x: 0,
                    y: (i * &self.line_height) as i32,
                }),
                buf,
            );
        }
    }
}
/*
    Text Field
 */

impl Drawable for TextField {
    fn render(&self, area: Rect, buf: &mut Buffer, draw_flag: DrawFlag) {
        let text_field_area = area.centered(
            Constraint::Min(18),
            Constraint::Min(3),
        );
        let text_line: Line = Line::from(self.chars.iter().collect::<String>().as_str());
        let cursor_position: Position = text_field_area.as_position().offset(Offset{
            x: self.cursor_index as i32,
            y: 1
        });
    }
}
