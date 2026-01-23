use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crate::app::page::Entrance;
use crate::app::{Command, Stateful};
use crate::app::widget::SimpleButton;
use crate::event_handler::{Focusable, Interactable};

// impl Focusable for SimpleButton {
//     fn is_focused(&self) -> bool {
//         self.is_focused
//     }
//     fn set_focus(&mut self, value: bool) -> () {
//         self.is_focused = value;
//     }
//     fn focused_child_ref(&self) -> Option<&dyn Stateful> {
//         None
//     }
//     fn focused_child_mut(&mut self) -> Option<&mut dyn Stateful> {
//         None
//     }
//     fn focus_index(&self) -> Option<usize> {
//         None
//     }
// }
// impl Interactable for SimpleButton {
//     fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command> {
//         match key.kind {
//             KeyEventKind::Press=> {
//                 if let KeyCode::Char(code) = key.code {
//                     return match code {
//                         'q' => {
//                             Ok(Command::Quit)
//                         },
//                         _ => Ok(Command::None)
//                     }
//                 }
//                 if let KeyCode::Esc = key.code {
//                     self.is_focused = false;
//                 }
//             },
//             _ => {}
//         }
//         Ok(Command::None)
//     }
// }