use crate::app::dialog::TextInputDialog;
use crate::app::popup::{ExitConfirmPopup, MessagePopup};
use crate::app::widget::{DirectoryList, SimpleButton};
use crate::app::{Command, Component, Container};

pub struct EntrancePage {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hover_index: Option<usize>,
    pub components: Vec<Box<dyn Component>>,
}
impl EntrancePage {
    pub fn new() -> Self {
        Self {
            is_focused: true,
            is_hovered: false,
            hover_index: None,
            components: vec![
                Box::new(SimpleButton::new("Create").on_interact(Box::new(|| {
                    color_eyre::eyre::Ok(Command::PushPage(Box::new(CreateGlyphPage::new())))
                }))),
                Box::new(SimpleButton::new("Open").on_interact(Box::new(|| {
                    color_eyre::eyre::Ok(Command::PushPopup(Box::new(MessagePopup::new(
                        "Not Implemented",
                    ))))
                }))),
                Box::new(SimpleButton::new("Quit").on_interact(Box::new(|| {
                    color_eyre::eyre::Ok(Command::PushPopup(Box::new(ExitConfirmPopup::new(true))))
                }))),
            ],
        }
    }
}
pub struct CreateGlyphPage {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hover_index: Option<usize>,
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
}
impl CreateGlyphPage {
    pub fn new() -> Self {
        Self {
            is_focused: true,
            is_hovered: false,
            hover_index: None,
            containers: vec![Box::new(DirectoryList::new("Directory"))],
            components: vec![
                Box::new(SimpleButton::new("Back").on_interact(Box::new(|| Ok(Command::PopPage)))),
                Box::new(SimpleButton::new("Confirm").on_interact(Box::new(|| {
                    color_eyre::eyre::Ok(Command::PushDialog(Box::new(TextInputDialog::new())))}))),
            ],
        }
    }
}
