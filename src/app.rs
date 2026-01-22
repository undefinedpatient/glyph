use std::any::Any;

pub mod entrance;
mod widget;

use entrance::Entrance;
use crate::drawer::Drawable;
use crate::event_handler::{Focusable, Interactable};

pub enum Command {
    Quit,
    PushView(Box<dyn View>),
    PopView,
    None
}
pub trait View: Interactable + Focusable + Drawable {
    fn as_interactable(&mut self) -> &mut dyn Interactable;
    fn as_focusable(&mut self) -> &mut dyn Focusable;
    fn as_drawable(&mut self) -> &mut dyn Drawable;
}
impl<T: Interactable+Focusable+Drawable> View for T{
    fn as_interactable(&mut self) -> &mut dyn Interactable{
        self
    }
    fn as_focusable(&mut self) -> &mut dyn Focusable{
        self
    }
    fn as_drawable(&mut self) -> &mut dyn Drawable{
        self
    }
}

// Global State of the Application
pub struct AppState{
    pub should_quit: bool,
}
pub struct Application {
    pub views: Vec<Box<dyn View>>,
    pub q_commands: Vec<Command>,
    pub state: AppState,
}

impl Application {
    pub fn new() -> Application {
        Application {
            views: vec![Box::new(Entrance::new())],
            state: AppState{
                should_quit: false
            },
            q_commands: Vec::new(),
        }
    }
}