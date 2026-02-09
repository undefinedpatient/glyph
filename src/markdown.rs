use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use crate::theme::Theme;

pub struct Markdown;

impl Markdown {
    pub fn from_str<'a>(str: &'a str, area: &Rect, theme: &'a dyn Theme) -> Text<'a> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES| Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&str, options);
        let mut lines: Vec<Line> = Vec::new();
        let mut current_line: Vec<Span> = Vec::new();
        let mut style: Style = Style::default();
        let mut indent: u8 = 0u8;
        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Paragraph => {
                            lines.push(Line::default());
                        }
                        Tag::Strong => {
                            style = theme.bold();
                        }
                        Tag::Emphasis => {
                            style = theme.italic();
                        }
                        Tag::Strikethrough => {
                            style = theme.strikethrough();
                        }
                        _ => {}
                    }
                },
                Event::HardBreak => {
                    lines.push(Line::from(current_line));
                    current_line = Vec::new();
                    lines.push(Line::default());
                }
                Event::SoftBreak => {
                    lines.push(Line::from(current_line));
                    current_line = Vec::new();
                }
                Event::Rule => {
                    lines.push(format!("{:-<width$}", "", width=area.width as usize).into())
                }
                Event::Text(text) => current_line.push(Span::styled(text, style)),
                Event::End(tag) => {
                    match tag {
                        TagEnd::Paragraph => {
                            lines.push(Line::from(current_line));
                            current_line = Vec::new();
                            lines.push(Line::default());
                        }
                        TagEnd::Strong | TagEnd::Emphasis | TagEnd::Strikethrough => {
                            style = Style::default();
                        }
                        _ => {

                        }
                    }
                }
                _ => {}
            }
        }
        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }
        Text::from(lines)
    }
}
