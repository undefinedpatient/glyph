use crate::app::Stateful;
use crate::app::widget::SimpleButton;

pub struct Entrance {
    pub is_focused: bool,
    pub is_highlighted: bool,
    pub hover_index: Option<usize>,
    pub buttons: Vec<SimpleButton>,
    
}
impl Entrance {
    pub fn new() -> Self {
        Self{
            is_focused: true,
            is_highlighted: false,
            hover_index: None,
            buttons: vec![
                SimpleButton::new("Create"),
                SimpleButton::new("Open"),
                SimpleButton::new("Quit"),
            ],
        }
    }
}
