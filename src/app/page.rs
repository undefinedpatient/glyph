use crate::app::popup::MessagePopup;
use crate::app::widget::SimpleButton;
use crate::app::Command;
use crate::event_handler::Interactable;
use ratatui::widgets::ListState;

pub struct EntrancePage {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hover_index: Option<usize>,
    // pub buttons: Vec<SimpleButton>,
    pub interactables: Vec<Box<dyn Interactable>>,
    
}
impl EntrancePage {
    pub fn new() -> Self {
        Self{
            is_focused: true,
            is_hovered: false,
            hover_index: None,
            interactables: vec![
                Box::new(SimpleButton::new("Create").on_interact(
                    Box::new(
                        || {
                            color_eyre::eyre::Ok(Command::PushPopup(Box::new(MessagePopup::new("Not Implemented"))))
                        }
                    )
                )),
                Box::new(SimpleButton::new("Open").on_interact(
                    Box::new(
                        || {
                            color_eyre::eyre::Ok(Command::PushPopup(Box::new(MessagePopup::new("Not Implemented"))))
                        }
                    )
                )),
                Box::new(SimpleButton::new("Quit").on_interact(
                    Box::new(
                        || {
                            color_eyre::eyre::Ok(Command::PushPopup(Box::new(MessagePopup::new("Not Implemented"))))
                        }
                    )
                )),
            ],
            // interactables: Vec::new(),
        }
    }
}
pub struct CreateGlyphPage{
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hover_index: Option<usize>,
    pub list: ListState,
}
impl CreateGlyphPage {
    pub fn new() -> Self {
        Self {
            is_focused: true,
            is_hovered: false,
            hover_index: None,
            list: ListState::default(),
        }
    }
}