use crate::theme::Theme;
use crate::utils::number_to_roman;
use bitflags::bitflags;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Widget};
use tui_big_text::{BigText, PixelSize};

/// This is a customized Markdown Drawer powered by pulldown-cmark.
// List State
struct ListState {
    level: u32,
    is_ordered: bool,
    item_index: u32,
}
impl ListState {
    fn new(level: u32, is_ordered: bool, item_index:u32) -> Self {
        Self {
            level,
            is_ordered,
            item_index
        }
    }
    fn get_prefix(&self) -> String{
        if self.is_ordered {
            match self.level {
                2 => {
                    format!("{:<3}",number_to_roman(self.item_index as u16)+".")
                }
                _ => {
                    format!("{:<3}",self.item_index.to_string()+".")
                }
            }
        } else {
            match self.level {
                2 => {
                    "◦".to_string()
                }
                _ => {
                    "•".to_string()
                }
            }
        }
    }
}
struct QuoteState {
    level: u32,
}
impl QuoteState {
    fn new(level: u32) -> Self {
        Self {
            level
        }
    }
}
pub struct MarkdownRenderer<'a> {
    current_spans: Vec<Span<'a>>,
    rows_area: Vec<Rect>,
    render_row_index: usize,
    text_style: TextStyleBuilder,
    list_state_stack: Vec<ListState>,
    quote_state: QuoteState,
    is_in_code_block: bool,
    code_lines: Vec<Line<'a>>,
    area: Rect,
    theme: &'a dyn Theme
}

impl<'a> MarkdownRenderer<'a>{
    pub fn create(area: Rect, theme: &'a dyn Theme) -> Self {
        Self {
            current_spans: Vec::new(),
            rows_area: area.rows().into_iter().collect(),
            render_row_index: 0,
            text_style: TextStyleBuilder::new(),
            list_state_stack: Vec::new(),
            quote_state: QuoteState::new(0),
            is_in_code_block: false,
            code_lines: Vec::new(),
            area,
            theme,
        }
    }
    /// Render a line to the buffer and increment a row number by 1.
    fn render_line(&mut self, buffer: &mut Buffer) -> (){
        if self.quote_state.level != 0 {
            self.current_spans.insert(0, Span::from("░ ".repeat(self.quote_state.level as usize)));
        }

        if let Some(line_area) = self.rows_area.get(self.render_row_index) {
            Line::from(self.current_spans.clone()).render(*line_area, buffer);
            self.current_spans = Vec::new();
            self.render_row_index +=1;
        }
    }
    fn render_header(&mut self, buffer: &mut Buffer, level: HeadingLevel) -> () {
        if let Some(line_area) = self.rows_area.get(self.render_row_index) {
            match level {
                HeadingLevel::H1 => {
                    let text = BigText::builder().lines([Line::from(self.current_spans.clone())]).pixel_size(PixelSize::Full).build();
                    text.render(line_area.resize(Size::new(line_area.width, 8)).intersection(self.area), buffer);
                    self.render_row_index += 8;
                },
                HeadingLevel::H2 => {
                    let text = BigText::builder().lines([Line::from(self.current_spans.clone())]).pixel_size(PixelSize::HalfWidth).build();
                    text.render(line_area.resize(Size::new(line_area.width, 8)).intersection(self.area), buffer);
                    self.render_row_index += 8;
                },
                HeadingLevel::H3 => {
                    let text = BigText::builder().lines([Line::from(self.current_spans.clone())]).pixel_size(PixelSize::HalfHeight).build();
                    text.render(line_area.resize(Size::new(line_area.width, 4)).intersection(self.area), buffer);
                    self.render_row_index += 4;
                },
                HeadingLevel::H4 => {
                    let text = BigText::builder().lines([Line::from(self.current_spans.clone())]).pixel_size(PixelSize::Quadrant).build();
                    text.render(line_area.resize(Size::new(line_area.width, 4)).intersection(self.area), buffer);
                    self.render_row_index += 4;
                },
                HeadingLevel::H5 => {
                    let text = BigText::builder().lines([Line::from(self.current_spans.clone())]).pixel_size(PixelSize::Sextant).build();
                    text.render(line_area.resize(Size::new(line_area.width, 3)).intersection(self.area), buffer);
                    self.render_row_index += 3;
                },
                HeadingLevel::H6 => {
                    let text = BigText::builder().lines([Line::from(self.current_spans.clone())]).pixel_size(PixelSize::Octant).build();
                    text.render(line_area.resize(Size::new(line_area.width, 3)).intersection(self.area), buffer);
                    self.render_row_index += 3;
                }
            }
        }
        self.current_spans = Vec::new();
    }
    /// Render a markdown page in area, consume self.
    pub fn render(mut self, str: &'a str, buffer: &mut Buffer) -> () {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_FOOTNOTES);
        let parser = Parser::new_ext(&str, options);


        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Strong => {
                            self.text_style.set_flag(TextStyleFlag::STRONG);
                        }
                        Tag::Emphasis => {
                            self.text_style.set_flag(TextStyleFlag::EMPHASIS);
                        }
                        Tag::Strikethrough => {
                            self.text_style.set_flag(TextStyleFlag::STRIKETHROUGH);
                        }
                        Tag::List(is_ordered) => {
                            let new_level = self.list_state_stack.len() as u32 + 1;
                            self.list_state_stack.push(ListState {
                                level: new_level,
                                is_ordered: is_ordered.is_some(),
                                item_index: is_ordered.unwrap_or(1) as u32,
                            });
                            if !self.current_spans.is_empty() {
                                self.render_line(buffer);
                            }
                        }
                        // Simply add prefix to the line.
                        Tag::Item => {
                            // self.current_spans.push(Span::raw("<>"));
                            if let Some(list_state) = self.list_state_stack.last_mut() {
                                // Add indentation
                                let indent = "  ".repeat(list_state.level as usize);

                                // Add marker
                                self.current_spans.push(Span::raw(format!("{:<0}{1} ",indent, list_state.get_prefix())));
                                // Add for in case the list is ordered, if the list is not ordered, it has no effect.
                                list_state.item_index += 1;
                            }
                        }
                        Tag::BlockQuote(_quote_type) => {
                            self.quote_state.level += 1;
                        }
                        Tag::CodeBlock(_kind) => {
                            self.is_in_code_block = true;
                        }
                        _ => {
                        }
                    }
                }
                Event::Rule => {
                    self.current_spans = vec![
                        Span::from(format!("{:—<width$}", "", width = self.area.width as usize)).dark_gray()
                    ];
                    self.render_line(buffer);
                }
                Event::Code(text) => {
                    self.current_spans.push(Span::raw(text).bg(self.theme.surface_low_highlight()));
                }
                Event::Text(text) => {
                    if self.is_in_code_block {
                        for line in text.lines() {
                            self.code_lines.push(Line::from(line.to_string()));
                        }
                    } else {
                        self.current_spans.push(Span::styled(text, self.text_style.build(self.theme)));
                    }
                }
                Event::SoftBreak | Event::HardBreak => {
                    self.render_line(buffer);
                }
                Event::End(tag) => {
                    match tag {
                        TagEnd::Heading(level) => {
                            self.render_header(buffer, level);
                        }
                        TagEnd::Paragraph => {
                            self.render_line(buffer);
                        }
                        TagEnd::Strong => {
                            self.text_style.remove_flag(TextStyleFlag::STRONG);
                        }
                        TagEnd::Emphasis => {
                            self.text_style.remove_flag(TextStyleFlag::EMPHASIS);
                        }
                        TagEnd::Strikethrough => {
                            self.text_style.remove_flag(TextStyleFlag::STRIKETHROUGH);
                        }
                        TagEnd::List(_) => {

                            if self.list_state_stack.len() > 1 {
                                self.render_row_index -= 1;
                            }
                            if !self.list_state_stack.is_empty() {
                                self.list_state_stack.pop();

                            } else {
                                self.render_row_index += 1;
                            }
                        }
                        // Render the item in the list
                        TagEnd::Item => {
                            // self.current_spans.push(Span::raw("</>"));
                            self.render_line(buffer);
                        }
                        TagEnd::BlockQuote(_quote_type) => {
                            self.quote_state.level = self.quote_state.level.saturating_sub(1);
                            if self.quote_state.level == 0 {
                                self.render_row_index += 1;
                            }
                        }
                        TagEnd::CodeBlock => {
                            self.is_in_code_block = false;
                            let text: Text= Text::from(self.code_lines).bg(self.theme.surface_low_highlight());
                            let text_height: usize = text.height();
                            let text_width: usize = text.width();
                            if let Some(line_area) = self.rows_area.get(self.render_row_index) {
                                let code_block_frame: Block = Block::bordered().border_style(Style::default().dim());
                                let code_block_area: Rect = line_area.resize(Size::new(text_width as u16 + 2, text_height as u16 + 2));
                                let code_inner_area: Rect = code_block_frame.inner(code_block_area);
                                text.render(code_inner_area, buffer);
                                code_block_frame.render(code_block_area, buffer);
                                self.render_row_index += text_height + 2;
                            }
                            self.code_lines = Vec::new();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
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
}