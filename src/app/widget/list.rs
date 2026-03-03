// use ratatui::prelude::Stylize;
// use ratatui::text::Line;
// use ratatui::widgets::BorderType;
// use ratatui::widgets::Block;
// use std::any::Any;
//
// use crate::app::{Command, Container, DrawFlag, Drawable, Focusable, Interactable};
// use crate::theme::Theme;
// use color_eyre::Result;
// use crossterm::event::KeyEvent;
// use ratatui::layout::Rect;
// use ratatui::Frame;
// use crate::block;
//
// /// Data non-owning list
// pub struct ListViewState {
//     hovered_index: Option<u16>,
//     focused_index: Option<u16>,
//     scroll_offset: u16,
//     /// (string representation of data, data)
//     entries: Vec<(String, Box<dyn Any>)>,
//     is_focused: bool
// }
//
// pub struct List{
//     state: ListViewState,
//     on_select: Box<dyn Fn()->Result<Vec<Command>>>,
//     on_exit: Box<dyn Fn()->Result<Vec<Command>>>
// }
// impl List {
//     pub fn new() -> Self {
//         Self {
//             state: ListViewState {
//                 hovered_index: None,
//                 focused_index: None,
//                 scroll_offset: 0,
//                 is_focused: false,
//                 entries: Vec::new(),
//             },
//             on_select: Box::new(||{Ok(vec![])}),
//             on_exit: Box::new(||{Ok(vec![])})
//         }
//     }
//     pub fn from(entries: Vec<(String, Box<dyn Any>)>) -> Self {
//         Self {
//             state: ListViewState {
//                 hovered_index: None,
//                 focused_index: None,
//                 scroll_offset: 0,
//                 is_focused: false,
//                 entries
//             },
//             on_select: Box::new(||{Ok(vec![])}),
//             on_exit: Box::new(||{Ok(vec![])})
//         }
//     }
//     pub fn on_select(mut self, on_select: Box<dyn Fn()->Result<Vec<Command>>>) -> Self {
//         self.on_select = on_select;
//         self
//     }
//     pub fn on_exit(mut self, on_exit:Box<dyn Fn()->Result<Vec<Command>>>) -> Self {
//         self.on_exit = on_exit;
//         self
//     }
//
//     /// Overwrite the existing data.
//     pub fn update_entries(&mut self, new_entries: Vec<(String, Box<dyn Any>)>) -> () {
//         self.state.entries = new_entries;
//     }
// }
//
// impl Drawable for List {
//     fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
//         let border: Block = block!("",draw_flag,theme);
//         let inner_area: Rect = border.inner(area);
//
//     }
// }
// impl Interactable for List {
//     fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
//         Ok(vec![])
//     }
// }
//
// impl Focusable for List {
//     fn is_focused(&self) -> bool {
//         self.state.is_focused
//     }
//
//     fn set_focus(&mut self, value: bool) -> () {
//         self.state.is_focused = value;
//     }
//
//     /// List does not have nest childrens
//     fn focused_child_ref(&self) -> Option<&dyn Container> {
//         None
//     }
//     /// List does not have nested childrens
//     fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
//         None
//     }
//     /// List does not have nested childrens
//     fn focused_child_index(&self) -> Option<usize> {
//         None
//     }
// }
//
// impl From<List> for Box<dyn Container> {
//     fn from(value: List) -> Self {
//         Box::new(value)
//     }
// }