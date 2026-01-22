use crate::app::Focusable;

pub struct Entrance {
    is_focused: bool,
}
impl Entrance {
    pub fn new() -> Self {
        Self{
            is_focused: true,
        }
    }
}
impl Focusable for Entrance {
    fn is_focused(&self) -> bool {
        self.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.is_focused = value;
    }
}
