use crate::app::{Command, Container};
use color_eyre::eyre::Result;
use ratatui::style::Color;
use std::any::Any;


/*

    All popup on_xxx() function only takes app's state.

 */

pub struct MessagePopup {
    pub is_focused: bool,
    pub color: Color,
    pub message: String,
    
    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl MessagePopup {
    pub fn new(message: &str, color: Color) -> Self {
        Self {
            is_focused: true,
            color,
            message: String::from(message),
            on_exit: None,
        }
    }
    pub fn on_exit(mut self, on_exit: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>) -> Self {
        self.on_exit = Some(on_exit);
        self
    }
}
impl From<MessagePopup> for Box<dyn Container>{
    fn from(container: MessagePopup) -> Self {
        Box::new(container)
    }
}
pub struct ConfirmPopup {
    pub is_focused: bool,
    pub focus_index: usize,
    pub message: String,
    
    
    pub on_confirm: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl ConfirmPopup {
    pub fn new(message: &str) -> Self {
        Self {
            is_focused: true,
            focus_index: 0,
            message: String::from(message),
            
            on_confirm: None,
        }
    }
    pub fn on_confirm(mut self, on_confirm: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>) -> Self {
        self.on_confirm = Some(on_confirm);
        self
    }
}
impl From<ConfirmPopup> for Box<dyn Container>{
    fn from(container: ConfirmPopup) -> Self {
        Box::new(container)
    }
}
