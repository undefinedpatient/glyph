pub struct MessagePopup {
    pub is_focused: bool,
    pub message: String,
}
impl MessagePopup {
    pub fn new(message: &str) -> Self {
        Self { 
            is_focused: false,
            message: String::from(message),
        }
    }
}