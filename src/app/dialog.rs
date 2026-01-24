use crate::app::{Component, Container};

pub struct TextInputDialog {
    pub is_focused: bool,
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>
}
impl TextInputDialog {
    pub fn new() -> Self {
        Self {
            is_focused: false,
            containers: Vec::new(),
            components: vec![

            ],
        }
    }
}