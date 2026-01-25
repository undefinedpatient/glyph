use crate::app::widget::{DirectoryList, LineButton, SimpleButton, TextField, TextFieldInputMode};
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Focusable;
use crate::utils::get_dir_names;
use ratatui::layout::{Constraint, Offset, Position, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Widget};
use ratatui::Frame;

impl Drawable for SimpleButton {
    fn render(&self, frame: &mut Frame,area: Rect, draw_flag: DrawFlag) {
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from(["[", self.label.as_str(), "]"].concat())
                    .bold()
                    .centered()
                    .render(area, frame.buffer_mut());
            }
            _ => {
                Line::from(self.label.as_str()).centered().render(area, frame.buffer_mut());
            }
        }
    }
}

impl Drawable for LineButton {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let text = self.label.clone().to_string();
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from([" ",text.as_str()," "].concat()).render(area, frame.buffer_mut());
            },
            _ => {
                Line::from(["[",text.as_str(),"]"].concat()).bold().render(area, frame.buffer_mut());
            }
        }
    }
}
impl Drawable for DirectoryList {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Container Frame
        */
        let current_path: String = (&self.current_path)
            .clone()
            .to_str()
            .unwrap_or("Invalid Path")
            .to_string();
        let widget_frame: Block = match draw_flag {
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
        let inner_area: Rect = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());
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
                frame.buffer_mut(),
            );
        }
    }
}
/*
    Text Field
 */

impl Drawable for TextField {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let text_field_area = area.centered(
            Constraint::Min(18),
            Constraint::Min(3),
        );
        let text = self.chars.iter().collect::<String>();
        let text_line: Line = Line::from(text);
        let text_field_block: Block = Block::bordered().title(self.label.as_str())
            .border_type(
                match draw_flag {
                    DrawFlag::DEFAULT => BorderType::Plain,
                    DrawFlag::HIGHLIGHTING => BorderType::Double,
                    DrawFlag::FOCUSED => BorderType::Thick,
                    _ => BorderType::LightDoubleDashed,
                }
            )
            .border_style(
                match self.input_mode {
                    TextFieldInputMode::Normal => Style::default().fg(Color::White),
                    TextFieldInputMode::Edit => Style::default().fg(Color::Yellow),
                }
            );
        let text_line_area: Rect = text_field_block.inner(text_field_area);
        if self.is_focused() {
            let cursor_position: Position = text_field_area.as_position().offset(Offset{ x: 1 + self.cursor_index as i32,
                y: 1
            });
            frame.set_cursor_position(cursor_position);
        }
        Clear.render(text_field_area, frame.buffer_mut());
        text_field_block.render(text_field_area, frame.buffer_mut());
        text_line.render(text_line_area, frame.buffer_mut());
    }
}
