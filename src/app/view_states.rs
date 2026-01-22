use std::path::PathBuf;
use ratatui::widgets::ListState;

use crate::app::{ButtonState, Focusable, GListState, TextFieldState};
use crate::app::view_type::DialogView;
use crate::event_handler::EventHandler;
use crate::layout::LayoutHandler;

pub trait FocusHandler {
    fn previous(&mut self) -> ();
    fn next(&mut self) -> ();
    fn focused_ref(&self) -> Option<&dyn Focusable>;
    fn focused_mut(&mut self) -> Option<&mut dyn Focusable>;
    fn has_focus(&self) -> bool;
    fn reset_focus(&mut self) -> () {}
}








// pub enum PageState {
//     Entrance {
// 
//     },
//     CreateGlyph {
//         list_state: ListState,
//     },
// }
// 




pub struct EntrancePageState {}
impl EntrancePageState {}
impl FocusHandler for EntrancePageState {
    fn previous(&mut self) -> () {}
    fn next(&mut self) -> () {}
    fn focused_ref(&self) -> Option<&dyn Focusable> {
        None
    }
    fn focused_mut(&mut self) -> Option<&mut dyn Focusable> {
        None
    }
    fn has_focus(&self) -> bool {
        false
    }
    fn reset_focus(&mut self) -> () {}
}





pub struct CreateGlyphPageState {
    index: Option<usize>,
    states: Vec<Box<dyn Focusable>>,
    current_path: PathBuf,
}
impl CreateGlyphPageState {
    fn new() -> Self {
        Self {
            index: None,
            states: vec![Box::new(GListState::new())],
            current_path: std::env::current_dir().unwrap(),
        }
    }
}
impl FocusHandler for CreateGlyphPageState {
    fn previous(&mut self) -> () {
        if let Some(index) = self.index {
            self.index = Some((index - 1) % self.states.len());
        } else {
            self.index = Some(self.states.len()-1);
        }
    }
    fn next(&mut self) -> () {
        if let Some(index) = self.index {
            self.index = Some((index + 1) % self.states.len());
        } else {
            self.index = Some(0);
        }
    }
    fn focused_ref(&self) -> Option<&dyn Focusable> {
        if let Some(index) = self.index {
            Some(&*self.states[index])
        } else {
            None
        }
    }
    fn focused_mut(&mut self) -> Option<&mut dyn Focusable> {
        if let Some(index) = self.index {
            Some(&mut *self.states[index])
        } else {
            None
        }
    }
    fn has_focus(&self) -> bool {
        self.index.is_some()
    }
    fn reset_focus(&mut self) -> () {
        self.index = None
    }
}





pub struct CreateGlyphDialogState {
    index: Option<usize>,
    states: Vec<Box<dyn Focusable>>,
}
impl CreateGlyphDialogState {
    fn new() -> Self {
        Self {
            index: None,
            states: vec![Box::new(TextFieldState::new()), Box::new(ButtonState::new())],
        }
    }
}
impl FocusHandler for CreateGlyphDialogState {
    fn previous(&mut self) -> () {
        if let Some(index) = self.index {
            self.index = Some((index - 1) % self.states.len());
        } else {
            self.index = Some(self.states.len()-1);
        }
    }
    fn next(&mut self) -> () {
        if let Some(index) = self.index {
            self.index = Some((index + 1) % self.states.len());
        } else {
            self.index = Some(0);
        }
    }
    fn focused_ref(&self) -> Option<&dyn Focusable> {
        if let Some(index) = self.index {
            Some(&*self.states[index])
        } else {
            None
        }
    }
    fn focused_mut(&mut self) -> Option<&mut dyn Focusable> {
        if let Some(index) = self.index {
            Some(&mut *self.states[index])
        } else {
            None
        }
    }
    fn has_focus(&self) -> bool {
        self.index.is_some()
    }
    fn reset_focus(&mut self) -> () {
        self.index = None
    }
}