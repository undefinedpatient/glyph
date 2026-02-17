use crate::app::{Command, Component, Container};
use crate::drawer::DrawFlag;
use crate::state::widget::{DirectoryListState, EditMode, NumberFieldState, OptionMenuState, TextEditorState, TextFieldState};
use crate::utils::{cycle_offset, get_dir_names, get_file_names};
use color_eyre::eyre::Result;
use ratatui::style::Stylize;
use ratatui::text::Line;
use std::any::Any;
/*
    For component widget's on_xxx() function only takes its direct parent's state.
    For container widget's on_xxx() function takes its own state as well.

 */



/*
    Button
    - on_interact(parent_state)
 */
pub struct Button {
    pub label: String,

    pub on_interact: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_interact: None,
        }
    }
    pub fn on_interact(
        mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        self.on_interact = Some(f);
        self
    }
}
impl From<Button> for Box<dyn Component> {
    fn from(component: Button) -> Self {
        Box::new(component)
    }
}
pub struct LineButton {
    pub label: String,

    pub on_interact: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl LineButton {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_interact: None,
        }
    }
    pub fn on_interact(
        mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        self.on_interact = Some(f);
        self
    }

    pub fn as_line(&self, draw_flag: DrawFlag) -> Line<'_> {
        let text = self.label.clone().to_string();
        match draw_flag {
            DrawFlag::HIGHLIGHTING => Line::from(["[", text.as_str(), "]"].concat()).bold(),
            _ => Line::from([" ", text.as_str(), " "].concat()),
        }
    }
}
impl From<LineButton> for Box<dyn Component> {
    fn from(component: LineButton) -> Self {
        Box::new(component)
    }
}


/*
   Directory Lists
*/
pub struct DirectoryList {
    pub state: DirectoryListState,
    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl DirectoryList {
    pub(crate) fn new(label: &str, show_files: bool, select_dir: bool) -> Self {
        Self {
            state: DirectoryListState {
                is_focused: false,
                label: label.to_string(),
                line_height: 1,
                current_path: std::env::current_dir().unwrap(),
                selected_file_path: None,
                hovered_index: None,
                selected_index: None,
                offset: 0,
                show_files,
                select_dir,
            },
            on_exit: None,
        }
    }
    pub fn on_exit(mut self, on_exit: Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>)-> Result<Vec<Command>>>) -> Self {
        self.on_exit = Some(on_exit);
        self
    }
    pub fn get_num_files(&self) -> usize {
        get_file_names(&self.state.current_path).unwrap().len()
    }
    pub fn get_num_dirs(&self) -> usize {
        get_dir_names(&self.state.current_path).unwrap().len()
    }
    pub fn get_num_entries(&self) -> usize {
        if self.state.show_files {
            self.get_num_files() + self.get_num_dirs()
        } else {
            self.get_num_dirs()
        }
    }
    pub fn next_entry(&mut self) -> () {
        if let Some(index) = self.state.hovered_index {
            let num_entries = self.get_num_entries();
            self.state.hovered_index = Some(cycle_offset(index as u16, 1, num_entries as u16) as usize);
        } else {
            self.state.hovered_index = Some(0);
        }
    }
    pub fn previous_entry(&mut self) -> () {
        let num_entries = self.get_num_entries();
        if let Some(index) = self.state.hovered_index {
            self.state.hovered_index = Some(cycle_offset(index as u16, -1, num_entries as u16) as usize);
        } else {
            self.state.hovered_index = Some(num_entries - 1usize);
        }
    }
    pub fn page_up(&mut self) {
        self.state.offset = self.state.offset.saturating_sub(4);
    }
    pub fn page_down(&mut self) {
        self.state.offset += 4;
    }
}

impl From<DirectoryList> for Box<dyn Container> {
    fn from(component: DirectoryList) -> Self {
        Box::new(component)
    }
}


/*
   Text Field
   - on_exit(parent_state)
*/
pub struct TextField {
    pub state: TextFieldState,
    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
    pub validate: Box<dyn Fn(&str) -> bool>
}

impl TextField {
    pub fn new(label: &str, default: &str, validate: Box<dyn Fn(&str)->bool>) -> Self {
        Self {
            state: TextFieldState {
                is_focused: false,
                label: label.to_string(),
                chars: default.chars().collect(),
                cursor_index: default.len(),
                is_valid: true,
            },
            on_exit: None,
            validate
        }
    }
    pub fn replace(&mut self, content: String) -> () {
        self.state.chars = content.chars().collect();
        self.state.cursor_index = self.state.chars.len();
    }
    pub fn on_exit(mut self, on_exit: Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>)-> Result<Vec<Command>>>) -> Self {
        self.on_exit = Some(on_exit);
        self
    }
    pub fn move_to_next_char(&mut self) {
        self.state.cursor_index = self.state.cursor_index.saturating_add(1);
        if self.state.cursor_index >= self.state.chars.len() {
            self.state.cursor_index = self.state.chars.len();
        }
    }
    pub fn move_to_previous_char(&mut self) {
        self.state.cursor_index = self.state.cursor_index.saturating_sub(1);
    }
    pub fn insert_char(&mut self, char: char) {
        self.state.chars.insert(self.state.cursor_index, char);
    }
    pub fn delete_char(&mut self) {
        if self.state.cursor_index >= self.state.chars.len() {
            return;
        }
        self.state.chars.remove(self.state.cursor_index);
    }
}

impl From<TextField> for Box<dyn Container> {
    fn from(component: TextField) -> Self {
        Box::new(component)
    }
}


/*

    Number Field

 */
pub struct NumberField {
    pub state: NumberFieldState,
    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
    pub validate: Box<dyn Fn(&str) -> bool>
}

impl NumberField {
    pub fn new(label: &str, default: i16, validate: Box<dyn Fn(&str)->bool>) -> Self {
        Self {
            state: NumberFieldState {
                is_focused: false,
                label: label.to_string(),
                chars: default.to_string().chars().collect(),
                cursor_index: default.to_string().len(),
                is_valid: true,
            },
            on_exit: None,
            validate,
        }
    }
    pub fn replace(&mut self, content: i16) -> () {
        self.state.chars = content.to_string().chars().collect();
        self.state.cursor_index = self.state.chars.len();
    }
    pub fn on_exit(mut self, on_exit: Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>)-> Result<Vec<Command>>>) -> Self {
        self.on_exit = Some(on_exit);
        self
    }
    pub fn move_to_next_char(&mut self) {
        self.state.cursor_index = self.state.cursor_index.saturating_add(1);
        if self.state.cursor_index >= self.state.chars.len() {
            self.state.cursor_index = self.state.chars.len();
        }
    }
    pub fn move_to_previous_char(&mut self) {
        self.state.cursor_index = self.state.cursor_index.saturating_sub(1);
    }
    pub fn insert_char(&mut self, char: char) {
        self.state.chars.insert(self.state.cursor_index, char);
    }
    pub fn delete_char(&mut self) {
        if self.state.cursor_index >= self.state.chars.len() {
            return;
        }
        self.state.chars.remove(self.state.cursor_index);
    }
}

impl From<NumberField> for Box<dyn Container> {
    fn from(component: NumberField) -> Self {
        Box::new(component)
    }
}


/*
    EditorWrapper
 */

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
    pub fn move_to_next_word(&mut self) -> Result<()> {
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
    pub fn move_to_previous_word(&mut self) -> Result<()> {
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

pub struct OptionMenu {
    pub state: OptionMenuState,
    pub on_update: Option<Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}

impl OptionMenu {
    pub fn new(options: Vec<(String, u8)>, default: u8) -> Self {
        Self {
            state: OptionMenuState {
                current_index: default,
                options,
            },
            on_update: None
        }
    }
    pub fn on_interact(
        mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>, Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        self.on_update = Some(f);
        self
    }

    pub fn replace(&mut self, new_selection: u8) -> () {
        self.state.current_index = new_selection;
    }
}

impl From<OptionMenu> for Box<dyn Component> {
    fn from(container: OptionMenu) -> Self {
        Box::new(container)
    }
}