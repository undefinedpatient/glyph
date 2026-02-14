use bitflags::bitflags;
use color_eyre::owo_colors::OwoColorize;
use crate::theme::Theme;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use ratatui::buffer::Buffer;
use ratatui::layout::{Offset, Rect, Size};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Widget};

/// This is a customized Markdown Drawer powered by pulldown-cmark.
pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn render_markdown<'a>(str: &'a str, area: &Rect, buffer: &mut Buffer, theme: &'a dyn Theme) -> () {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_FOOTNOTES);
        let parser = Parser::new_ext(&str, options);

        let mut lines: Vec<Line> = Vec::new();
        let mut current_line: Vec<Span> = Vec::new();
        let mut render_offset: usize = 0; // Next y-position the line to be inserted.

        let mut style: TextStyleBuilder = TextStyleBuilder::new();

        // List State
        struct ListState {
            level: u32,
            is_ordered: bool,
            item_index: u32,
            bullet: char
        }
        struct QuoteState {
            level: u32,
        }
        let mut list_stack: Vec<ListState> = Vec::new();
        let mut quote_state: QuoteState = QuoteState { level: 0 };

        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Paragraph => {
                            current_line.push(Span::from("<p>"));
                        }
                        Tag::Strong => {
                            style.set_flag(TextStyleFlag::STRONG);
                        }
                        Tag::Emphasis => {
                            style.set_flag(TextStyleFlag::EMPHASIS);
                        }
                        Tag::Strikethrough => {
                            style.set_flag(TextStyleFlag::STRIKETHROUGH);
                        }
                        Tag::List(is_ordered) => {
                            let level = list_stack.len() as u32 + 1;
                            let bullet = if is_ordered.is_some() {'?'} else {'•'};
                            list_stack.push(ListState {
                                level,
                                is_ordered: is_ordered.is_some(),
                                item_index: is_ordered.unwrap_or(1) as u32,
                                bullet
                            })
                        }
                        Tag::BlockQuote(quote_type) => {
                            quote_state.level += 1;
                        }
                        Tag::Item => {
                            if let Some(state) = list_stack.last_mut() {
                                // Add indentation
                                let indent = "  ".repeat(state.level as usize);
                                if !current_line.is_empty() {
                                    lines.push(Line::from(current_line));
                                    current_line = Vec::new();
                                }
                                current_line.push(Span::raw(indent));

                                // Add marker
                                if state.is_ordered {
                                    current_line.push(Span::raw(format!("{}. ", state.item_index)));
                                    state.item_index += 1;
                                } else {
                                    current_line.push(Span::raw(format!("{} ", state.bullet)));
                                }
                            }
                        }
                        _ => {}
                    }
                },
                Event::HardBreak => {
                    if quote_state.level != 0 {
                        current_line.insert(0, Span::from("░ ".repeat(quote_state.level as usize)));
                    }
                    lines.push(Line::from(current_line));
                    current_line = Vec::new();
                    lines.push(Line::default());
                    render_offset += 1;
                }
                Event::SoftBreak => {
                    if quote_state.level != 0{
                        current_line.insert(0, Span::from("░ ".repeat(quote_state.level as usize)));
                    }
                    lines.push(Line::from(current_line));
                    current_line = Vec::new();
                    render_offset += 1;
                }
                Event::Rule => {
                    lines.push(Line::from(format!("{:—<width$}", "", width=area.width as usize - 1)).dark_gray());
                    render_offset+=1;
                }
                Event::Text(text) => {
                    current_line.push(Span::styled(text, style.build(theme)));
                },
                Event::End(tag) => {
                    match tag {
                        TagEnd::List(_) => {
                            if !list_stack.is_empty() {
                                list_stack.pop();
                            } else {
                                lines.push(Line::default());
                                render_offset+=1;
                            }
                        }
                        TagEnd::Item => {
                            if !current_line.is_empty() {
                                lines.push(Line::from(current_line));
                                current_line = Vec::new();
                                render_offset+=1;
                            }
                        }
                        TagEnd::BlockQuote(quote_type) => {
                            quote_state.level = quote_state.level.saturating_sub(1);
                            if quote_state.level == 0 {
                                lines.push(Line::default());
                            }
                        }
                        TagEnd::Paragraph => {
                            current_line.push(Span::from("</p>"));
                            if quote_state.level != 0{
                                current_line.insert(0, Span::from("░ ".repeat(quote_state.level as usize)));
                                lines.push(Line::from(current_line));
                                current_line = Vec::new();
                            } else {
                                lines.push(Line::from(current_line));
                                current_line = Vec::new();
                                lines.push(Line::default());
                            }
                            render_offset+=1;
                        }
                        TagEnd::Strong => {
                            style.remove_flag(TextStyleFlag::STRONG);
                        }
                        TagEnd::Emphasis => {
                            style.remove_flag(TextStyleFlag::EMPHASIS);
                        }
                        TagEnd::Strikethrough => {
                            style.remove_flag(TextStyleFlag::STRIKETHROUGH);
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
        Text::from(lines).render(*area, buffer);
    }
}
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct TextStyleFlag: u32 {
        const STRONG = 0b0000_0001;
        const EMPHASIS = 0b0000_0010;
        const STRIKETHROUGH = 0b0000_0100;
    }
}
struct TextStyleBuilder{
    flags: TextStyleFlag,
}
impl TextStyleBuilder {
    pub fn new() -> TextStyleBuilder {
        Self {
            flags: TextStyleFlag::empty(),
        }
    }
    pub fn set_flag(&mut self, flag: TextStyleFlag) -> () {
        self.flags.insert(flag);
    }
    pub fn remove_flag(&mut self, flag: TextStyleFlag) {
        self.flags.remove(flag);
    }
    pub fn build(&self, theme: &dyn Theme) -> Style {
        let mut style: Style = Style::default().fg(theme.font());
        if self.flags.contains(TextStyleFlag::STRONG) {
            style = style.patch(theme.bold());
        }
        if self.flags.contains(TextStyleFlag::EMPHASIS) {
            style = style.patch(theme.italic());
        }
        if self.flags.contains(TextStyleFlag::STRIKETHROUGH) {
            style = style.patch(theme.strikethrough());
        }
        style
    }
    pub fn reset_all(&mut self) -> () {
        self.flags = TextStyleFlag::empty();
    }
}