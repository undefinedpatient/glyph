use ratatui::widgets::BorderType;
use std::any::Any;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Offset, Position, Rect, Rows};
use ratatui::prelude::{Line, Span, Widget};
use ratatui::widgets::Block;
use color_eyre::eyre::Result;
use ratatui::style::Stylize;
use crate::app::{Command, Container};
use crate::block;
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Interactable;
use crate::focus_handler::Focusable;
use crate::theme::Theme;

pub enum EditMode {
    Normal,
    Insert,
    Visual,
    VisualLine
}
pub struct TextEditorState {
    pub is_focused: bool,
    pub label: String,

    pub mode: EditMode,
    pub lines: Vec<Vec<char>>,
    pub scroll_offset: (usize,usize),
    pub cursor_index: usize,
    pub cursor_line_index: usize,

    pub anchor: (usize, usize),

    pub copy_buffer: Vec<Vec<char>>, // First line insert char, the rest directly insert line.
}
pub struct TextEditor {
    pub state: TextEditorState,

    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl TextEditor { pub fn new(label: &str, default: &str) -> Self {
    Self {
        state: TextEditorState {
            is_focused: false,
            label: label.to_string(),

            mode: EditMode::Normal,
            lines: Vec::new(),
            scroll_offset: (0, 0),
            cursor_index: 0,
            cursor_line_index: 0,

            anchor: (0,0),

            copy_buffer: Vec::new(), // First line insert char, the rest directly insert line.

        },
        on_exit: None,
    }
}
    pub fn on_exit(mut self, on_exit: Box<dyn FnMut(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>) -> Self {
        self.on_exit = Some(on_exit);
        self
    }
    pub fn to_raw_string(&self) -> String {
        let mut lines = self.state.lines.clone();
        for line in &mut lines[0..self.state.lines.len()-1] {
            line.push('\n');
        }
        self.state.lines.concat().iter().collect::<String>()
    }

    pub fn replace(&mut self, content: String) -> () {
        let parsed_content_0: Vec<&str> = content.split('\n').collect::<Vec<&str>>();
        let parsed_content_1: Vec<Vec<char>> = parsed_content_0.iter().map(
            |line| line.chars().collect::<Vec<char>>(),
        ).collect::<Vec<Vec<char>>>();
        self.state.lines = parsed_content_1;
        self.state.cursor_index = 0;
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.state.cursor_index, self.state.cursor_line_index)
    }


    pub fn get_line_len_at(&self, row: usize) -> usize {
        if self.state.lines.get(row).is_some() {
            return self.state.lines.get(row).unwrap().len();
        }
        0
    }

    pub fn switch_mode(&mut self, mode: EditMode) {
        self.state.mode = mode;
    }

    pub fn scroll_vertical_offset(&mut self, offset: i16) -> () {
        if offset.is_positive() {
            self.state.scroll_offset =
                (
                    self.state.scroll_offset.0,
                    self.state.scroll_offset.1.saturating_add(offset.abs() as usize)
                );
        } else {
            self.state.scroll_offset =
                (
                    self.state.scroll_offset.0,
                    self.state.scroll_offset.1.saturating_sub(offset.abs() as usize)
                );
        }
    }
    pub fn scroll_horizontal_offset(&mut self, offset: i16) -> () {
        if offset.is_positive() {
            self.state.scroll_offset =
                (
                    self.state.scroll_offset.0.saturating_add(offset.abs() as usize),
                    self.state.scroll_offset.1
                );
        } else {
            self.state.scroll_offset =
                (
                    self.state.scroll_offset.0.saturating_sub(offset.abs() as usize),
                    self.state.scroll_offset.1
                );
        }
    }

    pub fn move_to_next_line(&mut self) {
        self.state.cursor_line_index = self.state.cursor_line_index.saturating_add(1)
            .clamp(0, self.state.lines.len().saturating_sub(1));
    }
    pub fn move_to_previous_line(&mut self) {
        self.state.cursor_line_index = self.state.cursor_line_index.saturating_sub(1)
            .clamp(0, self.state.lines.len().saturating_sub(1));
    }
    pub fn move_to_next_char(&mut self) {
        if let Some(current_line) = self.state.lines.get(self.state.cursor_line_index) {
            self.state.cursor_index = self.state.cursor_index.saturating_add(1).clamp(0, current_line.len());
        }
    }
    pub fn move_to_previous_char(&mut self) {
        if let Some(current_line) = self.state.lines.get(self.state.cursor_line_index) {
            self.state.cursor_index = self.state.cursor_index.clamp(0, current_line.len()).saturating_sub(1);
        }
    }
    pub fn move_to_next_word(&mut self) -> color_eyre::Result<()> {
        self.move_to_next_char();
        let _y: usize = self.state.cursor_line_index;
        let _x: usize = self.state.cursor_index.clamp(0, self.state.lines[_y].len().saturating_sub(1));
        if let Some((x, y)) = self.find_next(_x, _y,' ') {
            self.state.cursor_index = x;
        } else {
            self.move_to_end_of_line();
        }
        Ok(())
    }
    pub fn move_to_previous_word(&mut self) -> color_eyre::Result<()> {
        self.move_to_previous_char();
        let _y: usize = self.state.cursor_line_index;
        let _x: usize = self.state.cursor_index.clamp(0, self.state.lines[_y].len()-1);
        if let Some((x, y)) = self.find_previous(_x.saturating_sub(1), _y,' ') {
            self.state.cursor_index = x;
            self.move_to_next_char();
        } else {
            self.move_to_start_of_line();
        }
        Ok(())
    }

    pub fn move_to_end_of_line(&mut self) {
        if let Some(current_line) = self.state.lines.get(self.state.cursor_line_index) {
            self.state.cursor_index = current_line.len();
        }
    }
    pub fn move_to_start_of_line(&mut self) {
        if let Some(current_line) = self.state.lines.get(self.state.cursor_line_index) {
            self.state.cursor_index = 0;
        }
    }

    pub fn insert_char(&mut self, char: char){
        if let Some(current_line) = self.state.lines.get_mut(self.state.cursor_line_index) {
            self.state.cursor_index = self.state.cursor_index.clamp(0, current_line.len());
            current_line.insert(self.state.cursor_index, char);
        }
        self.move_to_next_char();
    }
    pub fn delete_char(&mut self) {
        if let Some(current_line) = self.state.lines.get_mut(self.state.cursor_line_index) {
            if current_line.is_empty() {
                return;
            }
            if self.state.cursor_index >= current_line.len() {
                return
            }
            self.state.cursor_index = self.state.cursor_index.clamp(0, current_line.len());
            current_line.remove(self.state.cursor_index);
        }
    }
    pub fn insert_new_line_below(&mut self) {
        self.state.lines.insert(self.state.cursor_line_index+1, Vec::new());
    }
    pub fn insert_new_line_above(&mut self){
        self.state.lines.insert(self.state.cursor_line_index, Vec::new());
    }
    pub fn merge_with_next_line(&mut self){
        if self.state.lines.get(self.state.cursor_line_index).is_none() {
            return;
        }
        if self.state.lines.get(self.state.cursor_line_index+1).is_none() {
            return;
        }
        self.merge_line(self.state.cursor_line_index+1, self.state.cursor_line_index);
    }
    pub fn cut_into_next_newline(&mut self) {
        let line_index = self.state.cursor_line_index;
        if let Some(current_line) = self.state.lines.get_mut(line_index) {
            let from = self.state.cursor_index;
            let to = current_line.len().saturating_sub(1);
            let mut portion = self.remove_line_portion(from, to);
            self.insert_new_line_below();
            if let Some(next_line) = self.state.lines.get_mut(line_index+1) {
                next_line.append(&mut portion);
            }
            self.move_to_next_line();
            self.move_to_start_of_line();

        }

    }
    pub fn auto_horizontal_offset(&mut self) -> () {
        let cursor_screen_location: (usize, usize) =
            (
                self.state.cursor_index.saturating_sub(self.state.scroll_offset.0),
                self.state.cursor_line_index.saturating_sub(self.state.scroll_offset.1)
            );

        // Scroll the Vertical offset (1)
        if cursor_screen_location.1 < 7 {
            self.state.scroll_offset = (self.state.scroll_offset.0, self.state.cursor_line_index.saturating_sub(7));
        }
        if 42 - cursor_screen_location.1 < 7 {
            self.state.scroll_offset = (self.state.scroll_offset.0, self.state.cursor_line_index.saturating_add(7));
        }
    }
    // pub fn auto

    fn remove_line_portion(&mut self, from:usize, to:usize) -> Vec<char> {
        if from == to {
            return vec![];
        }
        if let Some(current_line) = self.state.lines.get_mut(self.state.cursor_line_index) {
            let captured: Vec<char> = current_line[from..=to].to_vec();
            for i in from..=to{
                current_line.remove(from);
            }
            return captured;
        }
        Vec::new()
    }
    fn merge_line(&mut self, from: usize, to: usize) {
        let mut from_line = self.state.lines.get_mut(from).unwrap().to_vec();
        self.delete_line(from);
        let to_line = self.state.lines.get_mut(to).unwrap();
        to_line.append(&mut from_line);
    }
    fn delete_line(&mut self, at: usize) {
        if let Some(current_line) = self.state.lines.get(at) {
            self.state.lines.remove(at);
        }
    }

    fn find_next(&mut self, x: usize, y: usize, character: char) -> Option<(usize, usize)> {
        if let Some(current_line) = self.state.lines.get(y) {
            // If x exceed the len of the line, or the line is empty.
            let len = current_line.len();
            if current_line.is_empty() || current_line.get(x).is_none() {
                return None;
            }
            for (i, c) in (*current_line)[x..len].iter().enumerate() {
                if (*c) == character {
                    return Some((i+x, y));
                }
            }
            None
        } else {
            None // Such line does not exist.
        }
    }
    fn find_previous(&mut self, x: usize, y: usize, character: char) -> Option<(usize, usize)> {
        if let Some(current_line) = self.state.lines.get(y) {
            // If x exceed the len of the line, or the line is empty.
            let len = current_line.len();
            if current_line.is_empty() || current_line.get(x).is_none() {
                return None;
            }
            for (i, c) in (*current_line)[0..=x].iter().enumerate().rev() {
                if *c == character {
                    return Some((i, y));
                }
            }
            None
        } else {
            None // Such line does not exist.
        }
    }
}
impl From<TextEditor> for Box<dyn Container> {
    fn from(container: TextEditor) -> Self {
        Box::new(container)
    }
}
impl Drawable for TextEditor {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let mut border: Block = block!(self.state.label.as_str(),draw_flag,theme);
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
        let horizontal_offset = if self.state.cursor_index > inner_area.width.saturating_sub(7) as usize {
            self.state.cursor_index - inner_area.width.saturating_sub(7) as usize
        } else {
            0
        };

        border.render(area, frame.buffer_mut());
        let lines: Vec<Line> = self.state.lines.iter().enumerate().skip_while(
            |(line_number, line)| {
                *line_number < self.state.scroll_offset.1
            }
        ).map(
            |(line_number, line)| {

                let mut line = Line::from(
                    vec![
                        Span::from(format!("{:<4}", line_number.to_string())).dim(),
                        Span::from(line.iter().skip(horizontal_offset).collect::<String>())
                    ]);
                if line_number == self.state.cursor_line_index {
                    line = line.bg(theme.surface_low());
                } else {
                    line = line.bg(theme.background());
                }
                line
            }
        ).collect();
        for (line_number, line_row) in line_rows.into_iter().enumerate() {
            lines.get(line_number).render(line_row, frame.buffer_mut());
        }
        if self.is_focused() {
            let (_x, _y) = self.get_cursor_position();
            let x = if _x > self.get_line_len_at(_y) {
                self.get_line_len_at(_y).saturating_sub(horizontal_offset)
            } else {
                _x.saturating_sub(horizontal_offset)
            };

            let y = _y.saturating_sub(self.state.scroll_offset.1);
            let cursor_position: Position = inner_area.as_position().offset(Offset {
                x: 4 + x as i32,
                y: y as i32,
            });
            if x < horizontal_offset {
                return;
            }
            // if self.state.cursor_line_index >
            frame.set_cursor_position(cursor_position);
        }
    }
}

impl Interactable for TextEditor {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        match self.state.mode {
            EditMode::Normal => {
                handle_normal_mode(self, key, parent_state)
            }
            EditMode::Insert => {
                handle_insert_mode(self, key)
            }
            EditMode::Visual => {
                handle_visual_mode(self, key)
            }
            EditMode::VisualLine => {
                handle_visual_line_mode(self, key)
            }
        }
    }
}
fn handle_normal_mode(me: &mut TextEditor, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::eyre::Result<Vec<Command>> {
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Esc = key.code {
                me.set_focus(false);
                if let Some(mut on_exit) = me.on_exit.take() {
                    let result = (*on_exit)(parent_state, Some(&mut me.state));
                    me.on_exit = Some(on_exit);
                    return result;
                };
                return Ok(Vec::new());
            }
            if let KeyCode::Char(c) = key.code {
                match c {
                    'h' => {
                        me.move_to_previous_char();
                    }
                    'j' => {
                        me.move_to_next_line();
                    }
                    'e' => {
                        me.move_to_next_char();
                        me.move_to_next_word()?;
                        me.move_to_previous_char();
                    }
                    'k' => {
                        me.move_to_previous_line();
                    }
                    'l' => {
                        me.move_to_next_char();
                    }
                    'a' => {
                        me.move_to_next_char();
                        me.switch_mode(EditMode::Insert);
                    }
                    'A' => {
                        me.move_to_end_of_line();
                        me.switch_mode(EditMode::Insert);
                    }
                    '0' => {
                        me.move_to_start_of_line();
                    }
                    '$' => {
                        me.move_to_end_of_line();
                    }
                    'I' => {
                        me.move_to_start_of_line();
                        me.switch_mode(EditMode::Insert);
                    }
                    'J' => {
                        me.merge_with_next_line();
                    }
                    'w' => {
                        me.move_to_next_word()?;
                    }
                    'b' => {
                        me.move_to_previous_word()?;
                    }
                    'x' => {
                        me.delete_char();
                    }
                    'd' => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            me.scroll_vertical_offset(16)
                        }
                    }
                    'u' => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            me.scroll_vertical_offset(-16)
                        }
                    }
                    'o' => {
                        me.insert_new_line_below();
                        me.move_to_next_line();
                        me.switch_mode(EditMode::Insert);
                    }
                    'O' => {
                        me.insert_new_line_above();
                        me.switch_mode(EditMode::Insert);
                    }
                    'i' => {
                        me.switch_mode(EditMode::Insert);
                    }
                    // 'v' => {
                    //     me.switch_mode(EditMode::Visual);
                    // }
                    // 'V' => {
                    //     me.switch_mode(EditMode::VisualLine);
                    // }
                    _ => {

                    }
                }
            }
            if let KeyCode::Left = key.code {
                me.move_to_previous_char();
            }
            if let KeyCode::Right = key.code {
                me.move_to_next_char();
            }
            if let KeyCode::Up = key.code {
                me.move_to_previous_line();
            }
            if let KeyCode::Down = key.code {
                me.move_to_next_line();
            }
            if let KeyCode::Backspace = key.code {
                if me.get_cursor_position().0 == 0 {
                    me.move_to_previous_line();
                    me.move_to_end_of_line();
                } else {
                    me.move_to_previous_char();
                }
            }
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }
}
fn handle_insert_mode(me: &mut TextEditor, key: &KeyEvent) -> color_eyre::eyre::Result<Vec<Command>> {
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Esc = key.code {
                me.switch_mode(EditMode::Normal);
            }
            if let KeyCode::Char(c) = key.code {
                me.insert_char(c);
            }
            if let KeyCode::Left = key.code {
                me.move_to_previous_char();
            }
            if let KeyCode::Right = key.code {
                me.move_to_next_char();
            }
            if let KeyCode::Up = key.code {
                me.move_to_previous_line();
            }
            if let KeyCode::Down = key.code {
                me.move_to_next_line();
            }
            if let KeyCode::Tab = key.code {
                me.insert_char(' ');
                me.insert_char(' ');
                me.insert_char(' ');
                me.insert_char(' ');
            }
            if let KeyCode::Backspace = key.code {
                if me.get_cursor_position().0 == 0 {
                    me.move_to_previous_line();
                    me.move_to_end_of_line();
                    me.merge_with_next_line();
                }
                else {
                    me.move_to_previous_char();
                    me.delete_char();
                }
            }
            if let KeyCode::Enter = key.code {
                me.cut_into_next_newline();
            }
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }

}
fn handle_visual_mode(me: &mut TextEditor, key: &KeyEvent) -> color_eyre::eyre::Result<Vec<Command>> {
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Esc = key.code {
                me.switch_mode(EditMode::Normal);
            }
            if let KeyCode::Char(c) = key.code {
                match c {
                    'h' => {
                        me.move_to_previous_char();
                    }
                    'j' => {
                        me.move_to_next_line();
                    }
                    'k' => {
                        me.move_to_previous_line();
                    }
                    'l' => {
                        me.move_to_next_char();
                    }
                    'i' => {
                        me.switch_mode(EditMode::Insert);
                    }
                    _ => {

                    }
                }
            }
            if let KeyCode::Left = key.code {
                me.move_to_next_char();
            }
            if let KeyCode::Right = key.code {
                me.move_to_previous_char();
            }
            if let KeyCode::Up = key.code {
                me.move_to_next_line();
            }
            if let KeyCode::Down = key.code {
                me.move_to_previous_line();
            }
            if let KeyCode::Backspace = key.code {
                me.move_to_previous_char();
            }
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }

}
fn handle_visual_line_mode(me: &mut TextEditor, key: &KeyEvent) -> color_eyre::eyre::Result<Vec<Command>> {
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Esc = key.code {
                me.switch_mode(EditMode::Normal);
            }
            if let KeyCode::Char(c) = key.code {
                match c {
                    'h' => {
                        me.move_to_previous_char();
                    }
                    'j' => {
                        me.move_to_next_line();
                    }
                    'k' => {
                        me.move_to_previous_line();
                    }
                    'l' => {
                        me.move_to_next_char();
                    }
                    'i' => {
                        me.switch_mode(EditMode::Insert);
                    }
                    _ => {

                    }
                }
            }
            if let KeyCode::Left = key.code {
                me.move_to_next_char();
            }
            if let KeyCode::Right = key.code {
                me.move_to_previous_char();
            }
            if let KeyCode::Up = key.code {
                me.move_to_next_line();
            }
            if let KeyCode::Down = key.code {
                me.move_to_previous_line();
            }
            if let KeyCode::Backspace = key.code {
                me.move_to_previous_char();
            }
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }

}
impl Focusable for TextEditor {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}
