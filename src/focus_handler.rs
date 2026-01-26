use crate::app::{Component, Container};

pub mod dialog;
pub mod page;
pub mod popup;
pub mod widget;

// pub struct FocusHandler<'a, 'b> {
//     hover_index: Option<usize>,
//     focus_index: Option<usize>,
//     ref_containers: Option<&'a Vec<Box<dyn Container>>>,
//     ref_components: Option<&'b Vec<Box<dyn Component>>>,
//     size: usize,
// }
// impl<'a, 'b> FocusHandler<'a, 'b> {
//     pub fn new(containers: Option<&'a Vec<Box<dyn Container>>>, components: Option<&'b Vec<Box<dyn Component>>>) -> Self {
//             Self{
//                 hover_index: None,
//                 focus_index: None,
//                 ref_containers: containers,
//                 ref_components: components,
//                 size: containers.unwrap_or(&Vec::new()).len()+components.unwrap_or(&Vec::new()).len(),
//
//             }
//         }
//     pub fn focus_next(&mut self) {
//         if self.hover_index.is_none() {
//             self.hover_index = Some(0);
//         }
//         self.focus_index = Some(self.focus_index.unwrap() + 1);
//     }
//     pub fn focus_prev(&mut self) {
//         if self.hover_index.is_none() {
//             self.hover_index = Some(0);
//         }
//         self.focus_index = Some(self.focus_index.unwrap() - 1);
//     }
//
// }
