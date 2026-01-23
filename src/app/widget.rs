
pub struct SimpleButton {
    pub label: String,
    pub(crate) is_highlighted: bool,
}
impl SimpleButton {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            is_highlighted: false,
        }
    }
}