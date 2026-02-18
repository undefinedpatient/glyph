use crate::app::{is_cycle_backward_hover_key, is_cycle_forward_hover_key, Command, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::block;
use crate::theme::Theme;
use crate::utils::{cycle_offset, get_dir_names, get_file_names};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Offset, Rect};
use ratatui::prelude::Stylize;
use ratatui::prelude::{Line, Span};
use ratatui::widgets::Block;
use ratatui::widgets::{BorderType, Widget};
use ratatui::Frame;
use std::any::Any;
use std::path::PathBuf;

pub struct DirectoryListState {
    pub is_focused: bool,
    pub label: String,
    pub line_height: usize,
    pub current_path: PathBuf,
    pub selected_file_path: Option<PathBuf>,
    pub hovered_index: Option<usize>,
    pub selected_index: Option<usize>,
    pub offset: usize,
    pub show_files: bool,
    pub select_dir: bool,
}
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
        let widget_frame: Block = block!(self.state.label.as_str(),draw_flag,theme)
            .title_top(Span::from(current_path.as_str()).into_right_aligned_line());

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
                let is_selected = self.state.selected_index == Some(i);
                let is_hovered   = self.state.hovered_index == Some(i);

                let prefix = match (is_selected, is_hovered) {
                    (true, true)   => ">[",
                    (true, false)  => " [",
                    (false, true)  => "> ",
                    (false, false) => "  ",
                };

                let suffix = if is_selected { "] " } else { "  " };

                let content = format!("{prefix}{}{suffix}", item);

                let mut line = Line::from(content);
                if is_selected {
                    line = line.bold();
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

impl Interactable for DirectoryList {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            if is_cycle_forward_hover_key(key) {
                self.next_entry();
            }
            if is_cycle_backward_hover_key(key) {
                self.previous_entry();
            }
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Char(char) = key.code {
                        return match char {
                            'u' => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    self.page_up();
                                }
                                Ok(Vec::new())
                            }
                            'd' => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    self.page_down();
                                }
                                Ok(Vec::new())
                            }
                            ' ' => {
                                if let Some(hovered_index) = self.state.hovered_index {
                                    if let Some(selected_index) = self.state.selected_index {
                                        if selected_index == hovered_index {
                                            self.state.selected_index = None;
                                            return Ok(Vec::new());
                                        }
                                    }
                                    if self.state.select_dir || hovered_index >= self.get_num_dirs() {
                                        self.state.selected_index = self.state.hovered_index;
                                    }
                                }
                                Ok(Vec::new())
                            }
                            _ => Ok(Vec::new()),
                        };
                    }
                    if let KeyCode::Esc = key.code {
                        if let Some(selected_index) = self.state.selected_index {
                            let mut entries = get_dir_names(self.state.current_path.as_path()).unwrap_or(Vec::new());
                            entries.append(&mut get_file_names(self.state.current_path.as_path()).unwrap_or(Vec::new()));
                            self.state.selected_file_path = Some(self.state.current_path.join(entries[selected_index].clone()));
                        } else {
                            self.state.selected_file_path = Some(self.state.current_path.clone());
                        }
                        self.set_focus(false);
                        if let Some(mut on_exit) = self.on_exit.take() {
                            let result = on_exit(parent_state, Some(&mut self.state));
                            self.on_exit = Some(on_exit);
                            return result;
                        }
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            // "cd .."
                            if index == 0 {
                                if let Some(path_buf) = (&self.state.current_path).parent() {
                                    self.state.current_path = path_buf.to_path_buf().clone();
                                }
                                return Ok(Vec::new());
                            }
                            if index < get_dir_names(&self.state.current_path).unwrap_or(Vec::new()).len() {
                                self.state.current_path = self.state.current_path.join(PathBuf::from(
                                    get_dir_names(&self.state.current_path)?[index].to_string(),
                                ));
                                self.state.selected_index = None;
                                self.state.hovered_index = Some(0);
                            }
                            return Ok(Vec::new());
                        }
                    }
                    Ok(Vec::new())
                }
                _ => Ok(Vec::new()),
            }
        }
    }
}

impl Focusable for DirectoryList {
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
