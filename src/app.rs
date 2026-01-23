use std::any::Any;

pub mod widget;
pub mod page;
pub mod popup;

use crate::drawer::Drawable;
use crate::event_handler::{Focusable, Interactable};
use page::Entrance;

pub enum Command {
    Quit,
    PushView(Box<dyn Stateful>),
    PopView,
    PushPopup(Box<dyn Stateful>),
    PopPopup,
    None
}
pub trait Stateful: Interactable +  Focusable+ Drawable {
    fn as_interactable_ref(&self) -> &dyn Interactable;
    fn as_interactable_mut(&mut self) -> &mut dyn Interactable;
    fn as_focusable_ref(& self) -> &dyn Focusable;
    fn as_focusable_mut(&mut self) -> &mut dyn Focusable;
    fn as_drawable_ref(& self) -> &dyn Drawable;
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable;
    fn as_stateful_ref(&self) -> &dyn Stateful;
    fn as_stateful_mut(&mut self) -> &mut dyn Stateful;
}
impl<T: Interactable + Focusable + Drawable> Stateful for T{
    fn as_interactable_ref(&self) -> &dyn Interactable{ self }
    fn as_interactable_mut(&mut self) -> &mut dyn Interactable{ self }
    fn as_focusable_ref(&self) -> &dyn Focusable{ self }
    fn as_focusable_mut(&mut self) -> &mut dyn Focusable { self }
    fn as_drawable_ref(&self) -> &dyn Drawable{ self }
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable{
        self
    }
    fn as_stateful_ref(&self) -> &dyn Stateful{ self }
    fn as_stateful_mut(&mut self) -> &mut dyn Stateful{ self }
}

// Global State of the Application
pub struct GlobalState {
    pub should_quit: bool,
}
impl Focusable for Application {
    fn is_focused(&self) -> bool {
       true
    }
    fn set_focus(&mut self, value: bool) -> () {}
    fn focused_child_ref(&self) -> Option<&dyn Stateful> {
        if self.popup_states.len() != 0 {
            return Some(self.popup_states.last().unwrap().as_stateful_ref());
        }
        if self.page_states.len() != 0 {
            return Some(self.page_states.last().unwrap().as_stateful_ref());
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Stateful> {
        if self.popup_states.len() != 0 {
            return Some(self.popup_states.last_mut().unwrap().as_stateful_mut());
        }
        if self.page_states.len() != 0 {
            return Some(self.page_states.last_mut().unwrap().as_stateful_mut());
        }
        None
    }
    
}
pub struct Application {
    pub page_states: Vec<Box<dyn Stateful>>,
    pub popup_states: Vec<Box<dyn Stateful>>,
    pub q_commands: Vec<Command>,
    pub state: GlobalState,
}

impl Application {
    pub fn new() -> Application {
        Application {
            page_states: vec![Box::new(Entrance::new())],
            popup_states: Vec::new(),
            state: GlobalState {
                should_quit: false
            },
            q_commands: Vec::new(),
        }
    }
}