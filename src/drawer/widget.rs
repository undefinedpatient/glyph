use crate::app::widget::{DirectoryList, SimpleButton};
use crate::drawer::{DrawFlag, Drawable};
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::{Offset, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, StatefulWidget, Widget};
use crate::event_handler::Focusable;
use crate::utils::get_dir_names;

impl Drawable for SimpleButton {
    fn render(&self, area: Rect, buf: &mut Buffer, draw_flag: DrawFlag) {
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from(
                    [
                        "[",
                        self.label.as_str(),
                        "]"
                    ].concat()
                ).bold().centered().render(area, buf);
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
        let frame: Block = match draw_flag {
            DrawFlag::DEFAULT => Block::default().borders(Borders::ALL).title(self.label.as_str()),
            DrawFlag::HIGHLIGHTING=> Block::default().borders(Borders::ALL).border_type(BorderType::Double).title(self.label.as_str()),
            DrawFlag::FOCUSED=> Block::default().borders(Borders::ALL).border_type(BorderType::Thick).title(self.label.as_str()),
            
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
            .map(
                |(i, item)| {
                    if let Some(index) = self.hover_index {
                        if index == i {
                            return Line::from(String::from("> ")+ &*item.clone());
                        }
                    }
                    return Line::from(String::from("  ")+ &*item.clone());
                }
            )
            .collect();
        let current_path: String = (&self.current_path).clone().to_str().unwrap_or("Invalid Path").to_string();
        for (i, line) in list_items.iter().enumerate() {
            line.render(inner_area.offset(Offset{x:0, y:(i*&self.line_height) as i32}), buf);
        }
    }
}