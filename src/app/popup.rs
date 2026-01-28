use crate::app::Container;
use ratatui::style::Color;

pub struct MessagePopup {
    pub is_focused: bool,
    pub color: Color,
    pub message: String,
}
impl MessagePopup {
    pub fn new(message: &str, color: Color) -> Self {
        Self {
            is_focused: false,
            color,
            message: String::from(message),
        }
    }
}
impl From<MessagePopup> for Box<dyn Container>{
    fn from(container: MessagePopup) -> Self {
        Box::new(container)
    }
}
pub struct ExitConfirmPopup {
    pub is_focused: bool,
    pub focus_index: usize,
}
impl ExitConfirmPopup {
    pub fn new(is_focused: bool) -> Self {
        Self {
            is_focused,
            focus_index: 0,
        }
    }
}
impl From<ExitConfirmPopup> for Box<dyn Container>{
    fn from(container: ExitConfirmPopup) -> Self {
        Box::new(container)
    }
}
