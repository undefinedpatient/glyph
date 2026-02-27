use crate::theme::Theme;
use crate::utils::number_to_roman;
use bitflags::bitflags;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::{Report, Result};
use pulldown_cmark::{Alignment, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Widget};
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

/// A Row Major Table with max size (256, 256)
#[derive(Debug)]
struct MarkdownTable<'a> {
    data: Vec<Vec<Vec<Span<'a>>>>,
    /// Zero-indexed position
    position: (u8, u8),
    /// Alignment for each column, should have exact same size of num of column
    alignments: Vec<Alignment>,
    size: (u8, u8)
}
impl<'a> MarkdownTable<'a> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            position: (0,0),
            alignments: Vec::new(),
            size: (0, 0),
        }
    }
    /// Get the size (num of row, num of column)
    fn size(&self) -> (u8, u8) {
        self.size
    }

    /// Return true for all u8 x in any case among: (0, x), (x, 0), (0,0)
    fn is_zero_size(&self) -> bool {
        self.size.0 == 0 ||  self.size.1 == 0
    }

    fn set_alignments(&mut self, alignments: Vec<Alignment>) -> () {
        self.alignments = alignments;
    }

    /// Set the size of each column
    fn set_height(&mut self, height: u8) -> () {
        self.size.0 = height;
        let mut default_row: Vec<Vec<Span>> = Vec::new();
        default_row.resize(self.size.1 as usize, Vec::new());
        self.data.resize(height as usize, default_row);
    }
    /// Set the size of each row
    fn set_width(&mut self, width: u8) -> () {
        self.size.1 = width;
        for row in self.data.iter_mut() {
            row.resize(width as usize, Vec::new());
        }
    }
    /// Set the pointer to point next row.
    fn next_row(&mut self) -> () {
        self.position.0 = self.position.0.saturating_add(1);
    }
    /// Set the pointer to point next column.
    fn next_column(&mut self) -> () {
        self.position.1 = self.position.1.saturating_add(1).clamp(0, self.size().1.saturating_sub(1));
    }
    /// Set the pointer to point previous row.
    fn previous_row(&mut self) -> () {
        self.position.0 = self.position.0.saturating_sub(1);

    }
    /// Set the pointer to point previous column.
    fn previous_column(&mut self) -> () {
        self.position.1 = self.position.1.saturating_add(1);
    }
    /// Set the row position back to 0, meaning the table pointer will point to the first item in the current row.
    fn reset_row_position(&mut self) {
        self.position.1 = 0;
    }
    /// Set the column position back to 0, meaning the table pointer will point to the first item in the current column.
    fn reset_column_position(&mut self) {
        self.position.0 = 0;
    }
    fn set_position(&mut self, pos: (u8, u8)) -> Result<(u8, u8)> {
        let size: (u8, u8) = self.size();
        if pos.0 >= size.0 || pos.1 >= size.1 {
            return Err(Report::msg("Index out of bounds"));
        }
        self.position = pos;
        Ok(pos)
    }
    /// Add span at current table position.
    fn push_span(&mut self, span: Span<'a>) -> Result<()> {
        let size: (u8, u8) = self.size();
        if self.position.0 >= size.0 || self.position.1 >= size.1 {
            return Err(Report::msg(format!("Index out of bounds: pos: {:?}, size: {:?}", self.position, size)));
        }
        self.data[self.position.0 as usize][self.position.1 as usize].push(span);
        Ok(())
    }
    /// Get a cloned spans at current table position.
    fn get_spans(&self) -> Vec<Span<'a>> {
        self.data[self.position.0 as usize][self.position.1 as usize].clone()
    }
    fn get_spans_at(&self, at: (u8, u8)) -> Result<Vec<Span<'a>>> {
        let size: (u8, u8) = self.size();
        if at.0 >= size.0 || at.1 >= size.1 {
            Err(Report::msg("Index out of bounds"))
        } else {
            Ok(self.data[at.0 as usize][at.1 as usize].clone())
        }
    }
    /// Get the size of the table for each cell in area (width, height) without taking account of border.
    fn cell_sizes(&self) -> (Vec<usize>, Vec<usize>) {
        let size: (u8, u8) = self.size();
        let mut heights: Vec<usize> = Vec::with_capacity(size.0 as usize);
        heights.resize(size.0 as usize, 0);
        let mut widths: Vec<usize> = Vec::with_capacity(size.1 as usize);
        widths.resize(size.0 as usize, 0);
        for (r_idx, row) in self.data.iter().enumerate() {
            let mut max_width = 1u16;
            for (c_idx, col) in row.iter().enumerate() {
                let cell_size = self.cell_size((r_idx as u8, c_idx as u8));
                if cell_size.0 > widths[c_idx] as usize {
                    widths[c_idx] = cell_size.0;
                }
                if cell_size.1 > heights[r_idx] as usize {
                    heights[r_idx] = cell_size.1;
                }
            }
        }
        (widths, heights)
    }
    fn to_lines(&self) -> Vec<Line<'a>> {
        let cell_sizes= self.cell_sizes();
        let mut lines: Vec<Line<'a>> = Vec::new();
        for (r_idx, row) in self.data.iter().enumerate() {
            // Render: Starting borders
            if r_idx == 0 {
                let mut line = Line::default();
                line.push_span(Span::raw("┏"));
                for (c_idx, _) in row.iter().enumerate() {
                    // If the first Cell in the row
                    let mut cell_width = cell_sizes.0[c_idx];
                    line.push_span(Span::raw("━".repeat(cell_width)));
                    if c_idx == row.len() - 1 {
                        line.push_span(Span::raw("┓"));
                        break;
                    }
                    line.push_span(Span::raw("┳"));
                }
                lines.push(line);
            }

            // Render: In between borders
            if r_idx != 0 {
                let mut line = Line::default();
                line.push_span(Span::raw(if r_idx == 1 {"┡"} else {"├"}));
                for (c_idx, cell) in row.iter().enumerate() {
                    // If the first Cell in the row
                    let mut cell_width = cell_sizes.0[c_idx];
                    line.push_span(Span::raw({if r_idx == 1 {"━"} else {"─"}}.repeat(cell_width)));
                    if c_idx == row.len() - 1 {
                        line.push_span(Span::raw(if r_idx == 1 {"┩"} else {"┤"}));
                        break;
                    }
                    line.push_span(Span::raw(if r_idx == 1 {"╇"} else {"┼"}));
                }
                lines.push(line);
            }


            // Render: Content
            let mut line = Line::default();
            for (c_idx, cells) in row.iter().enumerate() {
                // If the first Cell in the row
                if c_idx == 0 {
                    line.push_span(Span::raw(if r_idx == 0 {"┃"} else {"│"}));
                }
                let mut individual_cell_width = 0;
                // Count the individual cell width first, "abc" == 3
                for span in cells.clone() {
                    individual_cell_width += span.width();
                }

                // Get the number of padding needed.
                let num_of_empty_padding: usize = if individual_cell_width < cell_sizes.0[c_idx] {
                    cell_sizes.0[c_idx] - individual_cell_width
                } else {0};

                let (left_padding, right_padding) = match self.alignments[c_idx] {
                    Alignment::None | Alignment::Left => {
                        ("".to_string()," ".repeat(num_of_empty_padding))
                    }
                    Alignment::Center => {
                        (" ".repeat(num_of_empty_padding/2)," ".repeat(num_of_empty_padding.div_ceil(2)))
                    }
                    Alignment::Right => {
                        (" ".repeat(num_of_empty_padding),"".to_string())
                    }
                };
                line.push_span(Span::raw(left_padding));

                for span in cells.clone() {
                    if r_idx == 0 {
                        line.push_span(span.bold());
                    } else {
                        line.push_span(span);
                    }
                }

                line.push_span(Span::raw(right_padding));

                line.push_span(Span::raw(if r_idx == 0 {"┃"} else {"│"}));
            }
            lines.push(line);


            // Render: Closing border
            if r_idx == self.data.len() - 1{
                let mut line = Line::default();
                line.push_span(Span::raw("└"));
                for (c_idx, cell) in row.iter().enumerate() {
                    // If the first Cell in the row
                    let mut cell_width = cell_sizes.0[c_idx];
                    line.push_span(Span::raw("─".repeat(cell_width)));
                    if c_idx == row.len() - 1 {
                        line.push_span(Span::raw("┘"));
                        break;
                    }
                    line.push_span(Span::raw("┴"));
                }
                lines.push(line);
            }
        }
        lines
    }


    // Return the size of the cell without border (width, height) at (row, col)
    fn cell_size(&self, at: (u8, u8)) -> (usize, usize) {
        if let Ok(cell_spans) = self.get_spans_at(at) {
            let mut height = 1usize;
            let mut width = 1usize;
            for span in &cell_spans {
                width += span.width();
            }
            (width, height)
        } else {
            (0, 0)
        }
    }
}







pub struct MarkdownRenderer<'a> {
    spans_buffer: Vec<Span<'a>>,
    rows_area: Vec<Rect>,
    render_row_index: usize,
    text_style: TextStyleBuilder,

    list_state_stack: Vec<ListState>,
    quote_state: QuoteState,

    ///
    is_in_code_block: bool,
    is_in_table: bool,
    code_lines: Vec<Line<'a>>,
    /// Containing table data in Row Major Alignment
    table: MarkdownTable<'a>,
    area: Rect,
    theme: &'a dyn Theme
}

impl<'a> MarkdownRenderer<'a>{
    pub fn create(area: Rect, theme: &'a dyn Theme) -> Self {
        Self {
            spans_buffer: Vec::new(),
            rows_area: area.rows().into_iter().collect(),
            render_row_index: 0,
            text_style: TextStyleBuilder::new(),
            list_state_stack: Vec::new(),
            quote_state: QuoteState::new(0),
            is_in_code_block: false,
            is_in_table: false,
            code_lines: Vec::new(),
            table: MarkdownTable::new(),
            area,
            theme,
        }
    }
    /// Render a line to the buffer and increment a row number by 1.
    fn render_buffer(&mut self, buffer: &mut Buffer) -> (){
        if self.quote_state.level != 0 {
            self.spans_buffer.insert(0, Span::from("░ ".repeat(self.quote_state.level as usize)));
        }

        if let Some(line_area) = self.rows_area.get(self.render_row_index) {
            Line::from(self.spans_buffer.clone()).render(*line_area, buffer);
            self.spans_buffer = Vec::new();
            self.render_row_index += 1;
        }
    }
    fn render_header(&mut self, buffer: &mut Buffer, level: HeadingLevel) -> () {
        if let Some(line_area) = self.rows_area.get(self.render_row_index) {
            match level {
                HeadingLevel::H1 => {
                    let text = BigText::builder().lines([Line::from(self.spans_buffer.clone())]).pixel_size(PixelSize::Full).build();
                    text.render(line_area.resize(Size::new(line_area.width, 8)).intersection(self.area), buffer);
                    self.render_row_index += 8;
                },
                HeadingLevel::H2 => {
                    let text = BigText::builder().lines([Line::from(self.spans_buffer.clone())]).pixel_size(PixelSize::HalfWidth).build();
                    text.render(line_area.resize(Size::new(line_area.width, 8)).intersection(self.area), buffer);
                    self.render_row_index += 8;
                },
                HeadingLevel::H3 => {
                    let text = BigText::builder().lines([Line::from(self.spans_buffer.clone())]).pixel_size(PixelSize::HalfHeight).build();
                    text.render(line_area.resize(Size::new(line_area.width, 4)).intersection(self.area), buffer);
                    self.render_row_index += 4;
                },
                HeadingLevel::H4 => {
                    let text = BigText::builder().lines([Line::from(self.spans_buffer.clone())]).pixel_size(PixelSize::Quadrant).build();
                    text.render(line_area.resize(Size::new(line_area.width, 4)).intersection(self.area), buffer);
                    self.render_row_index += 4;
                },
                HeadingLevel::H5 => {
                    let text = BigText::builder().lines([Line::from(self.spans_buffer.clone())]).pixel_size(PixelSize::Sextant).build();
                    text.render(line_area.resize(Size::new(line_area.width, 3)).intersection(self.area), buffer);
                    self.render_row_index += 3;
                },
                HeadingLevel::H6 => {
                    let text = BigText::builder().lines([Line::from(self.spans_buffer.clone())]).pixel_size(PixelSize::Octant).build();
                    text.render(line_area.resize(Size::new(line_area.width, 2)).intersection(self.area), buffer);
                    self.render_row_index += 2;
                }
            }
        }
        self.spans_buffer = Vec::new();
    }
    /// Render a markdown page in area, consume self.
    pub fn render(mut self, str: &'a str, buffer: &mut Buffer) -> () {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_FOOTNOTES | Options::ENABLE_TASKLISTS);
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
                            if !self.spans_buffer.is_empty() {
                                self.render_buffer(buffer);
                            }
                        }
                        // Simply add prefix to the line.
                        Tag::Item => {
                            // self.current_spans.push(Span::raw("<>"));
                            if let Some(list_state) = self.list_state_stack.last_mut() {
                                // Add indentation
                                let indent = "  ".repeat(list_state.level as usize);

                                // Add marker
                                self.spans_buffer.push(Span::raw(format!("{:<0}{1} ", indent, list_state.get_prefix())));
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
                        Tag::Table(alignments) => {
                            self.is_in_table = true;
                            self.table = MarkdownTable::new();
                            self.table.set_width(alignments.len() as u8);
                            self.table.set_alignments(alignments);
                        }
                        Tag::TableCell => {
                        }
                        Tag::TableHead => {
                            self.table.set_height(self.table.size().0.saturating_add(1));
                        }
                        Tag::TableRow => {
                            // self.spans_buffer.push(Span::raw("<R>"));
                            self.table.set_height(self.table.size().0.saturating_add(1));
                        }
                        _ => {
                        }
                    }
                }
                Event::TaskListMarker(checked) => {
                    if checked {
                        self.spans_buffer.push(Span::raw("☑  "));
                    } else {
                        self.spans_buffer.push(Span::raw("☐  "));
                    }
                }
                Event::Rule => {
                    self.spans_buffer = vec![
                        Span::from(format!("{:—<width$}", "", width = self.area.width.saturating_sub(1) as usize)).dark_gray()
                    ];
                    self.render_buffer(buffer);
                }
                Event::Code(text) => {
                    self.spans_buffer.push(Span::raw(text).bg(self.theme.surface_low_highlight()));
                }
                Event::Text(text) => {
                    if self.is_in_code_block {
                        for line in text.lines() {
                            self.code_lines.push(Line::from(line.to_string()));
                        }
                        continue
                    }
                    if self.is_in_table {
                        self.table.push_span(Span::styled(text, self.text_style.build(self.theme))).unwrap();
                        continue
                    }
                    self.spans_buffer.push(Span::styled(text, self.text_style.build(self.theme)));
                }
                Event::SoftBreak | Event::HardBreak => {
                    self.render_buffer(buffer);
                }
                Event::End(tag) => {
                    match tag {
                        TagEnd::Heading(level) => {
                            self.render_header(buffer, level);
                        }
                        TagEnd::Paragraph => {
                            self.render_buffer(buffer);
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
                            self.render_buffer(buffer);
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
                                let code_block_frame: Block = Block::bordered().border_style(Style::default().dim()).border_type(BorderType::LightDoubleDashed);
                                let code_block_area: Rect = line_area.resize(Size::new(text_width as u16 + 2, text_height as u16 + 2));
                                let code_inner_area: Rect = code_block_frame.inner(code_block_area);
                                text.render(code_inner_area, buffer);
                                code_block_frame.render(code_block_area, buffer);
                                self.render_row_index += text_height + 2;
                            }
                            self.code_lines = Vec::new();
                        }
                        TagEnd::Table => {
                            self.is_in_table = false;
                            for line in self.table.to_lines() {
                                self.spans_buffer = line.spans;
                                self.render_buffer(buffer);
                            }
                        }
                        TagEnd::TableCell => {
                            self.table.next_column();
                        }
                        TagEnd::TableHead => {
                            self.table.next_row();
                            self.table.reset_row_position();
                        }
                        TagEnd::TableRow => {
                            self.table.next_row();
                            self.table.reset_row_position();
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








#[cfg(test)]
mod test {
    use crate::utils::markdown_renderer::MarkdownTable;

    #[test]
    fn test_table() {
        let mut table = MarkdownTable::new();
        table.set_height(6);
        table.set_width(4);
        assert_eq!(table.size(), (6,4));
        table.set_height(245);
        table.set_width(1);
        assert_eq!(table.size(), (245, 1));
    }
}
