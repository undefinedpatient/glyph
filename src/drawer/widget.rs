use color_eyre::owo_colors::OwoColorize;
use crate::app::widget::{Button, DirectoryList, LineButton, NumberField, TextEditor, TextField};
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Focusable;
use crate::theme::Theme;
use crate::utils::{get_dir_names, get_file_names};
use ratatui::layout::{Constraint, Offset, Position, Rect, Rows};
use ratatui::prelude::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap};
use ratatui::Frame;
use crate::state::widget::EditMode;

impl Drawable for Button {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from(["[", self.label.as_str(), "]"].concat())
                    .bold()
                    .centered()
                    .render(area, frame.buffer_mut());
            }
            _ => {
                Line::from(self.label.as_str())
                    .centered()
                    .render(area, frame.buffer_mut());
            }
        }
    }
}

impl Drawable for LineButton {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let text = self.label.clone().to_string();
        match draw_flag {
            DrawFlag::HIGHLIGHTING => {
                Line::from([" ", text.as_str(), " "].concat()).render(area, frame.buffer_mut());
            }
            _ => {
                Line::from(["[", text.as_str(), "]"].concat())
                    .bold()
                    .render(area, frame.buffer_mut());
            }
        }
    }
}
impl Drawable for DirectoryList {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let current_path: String = (&self.state.current_path)
            .clone()
            .to_str()
            .unwrap_or("Invalid Path")
            .to_string();
        let widget_frame: Block = match draw_flag {
            DrawFlag::DEFAULT => Block::default()
                .borders(Borders::ALL)
                .title(self.state.label.as_str())
                .title_top(Span::from(current_path.as_str()).into_right_aligned_line()),
            DrawFlag::HIGHLIGHTING => Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(self.state.label.as_str())
                .title_top(Span::from(current_path.as_str()).into_right_aligned_line()),
            DrawFlag::FOCUSED => Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(self.state.label.as_str())
                .title_top(Span::from(current_path.as_str()).into_right_aligned_line()),
        };

        /*
           Directory Widget
        */
        let inner_area: Rect = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());
        let mut list: Vec<String> = get_dir_names(&self.state.current_path).unwrap_or(Vec::new());
        let num_dir: usize = list.len();
        if self.state.show_files {
            list.append(&mut get_file_names(&self.state.current_path).unwrap_or(Vec::new()))
        }
        let list_items: Vec<Line> = list
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let mut line: Line;
                // If Selected => " >"
                if let Some(selected_index) = self.state.selected_index {
                    if selected_index == i {
                        line = Line::from(String::from(" > ") + &*item.clone());
                    } else {
                        line = Line::from(String::from("   ") + &*item.clone());
                    }
                } else {
                    line = Line::from(String::from("   ") + &*item.clone());
                }

                // If Hovered => Bold
                if let Some(hovered_index) = self.state.hovered_index {
                    if hovered_index == i {
                        line = line.bold();
                    }
                }
                if i != 0 && i < num_dir {
                    line.push_span(Span::raw("/"));
                }
                line
            })
            .collect();

        for (i, line) in list_items[self.state.offset..].iter().enumerate() {
            if i * self.state.line_height >= inner_area.height as usize {
                break;
            }
            line.render(
                inner_area.offset(Offset {
                    x: 0,
                    y: (i * &self.state.line_height) as i32,
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
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let text_field_area = area.centered(Constraint::Min(18), Constraint::Min(3));
        let content = self.state.chars.iter().collect::<String>();
        let content_paragraph: Paragraph = Paragraph::new(Line::from(content)).wrap(Wrap{trim: true});
        let text_field_block: Block = Block::bordered()
            .title(self.state.label.as_str())
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
                _ => BorderType::LightDoubleDashed,
            });
        let content_area: Rect = text_field_block.inner(text_field_area);
        if self.is_focused() {
            let cursor_position: Position = text_field_area.as_position().offset(Offset {
                x: 1 + (self.state.cursor_index % content_area.width as usize) as i32,
                y: 1 + (self.state.cursor_index /content_area.width as usize) as i32 ,
            });
            frame.set_cursor_position(cursor_position);
        }
        Clear.render(text_field_area, frame.buffer_mut());
        text_field_block.render(text_field_area, frame.buffer_mut());
        content_paragraph.render(content_area, frame.buffer_mut());
    }
}
/*
   Number Field
*/
impl Drawable for NumberField {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, p3: &dyn Theme) {
        let text_field_area = area.centered(Constraint::Min(18), Constraint::Min(3));
        let text = self.state.chars.iter().collect::<String>();
        let text_line: Line = Line::from(text);
        let text_field_block: Block = Block::bordered()
            .title(self.state.label.as_str())
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
                _ => BorderType::LightDoubleDashed,
            });
        let text_line_area: Rect = text_field_block.inner(text_field_area);
        if self.is_focused() {
            let cursor_position: Position = text_field_area.as_position().offset(Offset {
                x: 1 + self.state.cursor_index as i32,
                y: 1,
            });
            frame.set_cursor_position(cursor_position);
        }
        Clear.render(text_field_area, frame.buffer_mut());
        text_field_block.render(text_field_area, frame.buffer_mut());
        text_line.render(text_line_area, frame.buffer_mut());
    }
}

/*
    Editor Wrapper
 */
impl Drawable for TextEditor {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let mut border: Block = Block::bordered()
            .title(self.state.label.as_str())
            .border_type(match draw_flag {
                DrawFlag::DEFAULT => BorderType::Plain,
                DrawFlag::HIGHLIGHTING => BorderType::Double,
                DrawFlag::FOCUSED => BorderType::Thick,
                _ => BorderType::LightDoubleDashed,
            });
        match self.state.mode {
            EditMode::Normal => {
                border = border.title(Line::from("NORMAL").bold())
            },
            EditMode::Insert => {
                border = border.title(Line::from("INSERT").bold()).yellow()
            },
            EditMode::Visual => {
                border = border.title(Line::from("VISUAL").bold()).blue()
            }
            EditMode::VisualLine => {
                border = border.title(Line::from("VISUAL LINE").bold()).cyan()
            }

        }
        let inner_area = border.inner(area);
        let line_rows: Rows = inner_area.rows();
        border.render(area, frame.buffer_mut());
        let lines: Vec<Line> = self.state.lines.iter().enumerate().map(
            |(line_number, line)| {
                let mut line = Line::from(
                    vec![Span::from(line_number.to_string()+" ").dim(),
                         Span::from(line.iter().collect::<String>())
                    ]);

                if line_number == self.state.cursor_line_index {
                    line = line.bg(theme.surface_low_color());
                } else {
                    line = line.bg(theme.background_color());
                }



                line
            }
        ).collect();
        for (line_number, line_row) in line_rows.into_iter().enumerate() {
            lines.get(line_number).render(line_row, frame.buffer_mut());
        }
        if self.is_focused() {
            let cursor_position: Position = inner_area.as_position().offset(Offset {
                x: 2+ self.state.cursor_index as i32,
                y: self.state.cursor_line_index as i32,
            });
            frame.set_cursor_position(cursor_position);
        }
    }
}