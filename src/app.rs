use std::any::Any;

pub mod widget;
pub mod page;
pub mod popup;

use crate::drawer::Drawable;
use crate::event_handler::{Focusable, Interactable};
use page::EntrancePage;

pub enum Command {
    Quit,
    PushView(Box<dyn Stateful>),
    PopView,
    PushPopup(Box<dyn Stateful>),
    PopPopup,
    None
}
pub trait Convertible {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T: Any> Convertible for T {
    fn as_any(&self) -> &dyn Any where Self: Sized {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any where Self: Sized {
        self
    }
}
pub trait Stateful: Interactable + Drawable + Focusable {
    fn as_interactable_ref(&self) -> &dyn Interactable;
    fn as_interactable_mut(&mut self) -> &mut dyn Interactable;
    fn as_focusable_ref(& self) -> &dyn Focusable;
    fn as_focusable_mut(&mut self) -> &mut dyn Focusable;
    fn as_drawable_ref(& self) -> &dyn Drawable;
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable;
    fn as_stateful_ref(&self) -> &dyn Stateful;
    fn as_stateful_mut(&mut self) -> &mut dyn Stateful;
}
impl<T: Interactable + Drawable + Focusable> Stateful for T{
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
impl  Application {
    fn view_to_focus_ref(&self) -> Option<&dyn Stateful> {
        if self.popup_states.len() != 0 {
            // for (index, popup_state) in (&self.popup_states).iter().enumerate() {
            //     if popup_state.is_focused() || index == self.popup_states.len() - 1 {
            //         return Some(popup_state.as_stateful_ref());
            //     }
            // }
            return Some(self.popup_states.last().unwrap().as_stateful_ref());
        }
        if self.page_states.len() != 0 {
            // for (index, page_state) in (&self.page_states).iter().enumerate() {
            //     if page_state.is_focused() || index == self.page_states.len() - 1 {
            //         return Some(page_state.as_stateful_ref());
            //     }
            // }
            return Some(self.page_states.last().unwrap().as_stateful_ref());
        }
        None
    }
    pub(crate) fn view_to_focus_mut(&mut self) -> Option<&mut dyn Stateful> {
        if self.popup_states.len() != 0 {
            // let len: usize = self.popup_states.len();
            // for (index, popup_state) in (&mut self.popup_states).iter_mut().enumerate() {
            //     if popup_state.is_focused() || index == len-1 {
            //         return Some(popup_state.as_stateful_mut());
            //     }
            // }
            return Some(self.popup_states.last_mut().unwrap().as_stateful_mut());
        }
        if self.page_states.len() != 0 {
            // let len: usize = self.page_states.len();
            // for (index, page_state)in (&mut self.page_states).iter_mut().enumerate() {
            //     if page_state.is_focused() || index == len-1 {
            //         return Some(page_state.as_stateful_mut());
            //     }
            // }
            return Some((self.page_states).last_mut().unwrap().as_stateful_mut());
        }
        None
    }
    
}
pub struct Application {
    pub page_states: Vec<Box<dyn Stateful>>,
    pub dialog_states: Vec<Box<dyn Stateful>>,
    pub popup_states: Vec<Box<dyn Stateful>>,
    pub q_commands: Vec<Command>,
    pub state: GlobalState,
}

impl Application {
    pub fn new() -> Application {
        Application {
            page_states: vec![Box::new(EntrancePage::new())],
            dialog_states: Vec::new(),
            popup_states: Vec::new(),
            state: GlobalState {
                should_quit: false
            },
            q_commands: Vec::new(),
        }
    }
}