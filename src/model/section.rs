/*
    Section
 */
#[derive(Clone)]
pub struct Section {
    // pub entry_id: i64,
    pub position: i64,
    pub title: String,
    pub content: String,
}

impl Section {
    pub fn new(title: &str, default: &str, position: i64) -> Self {
        Self {
            position: position,
            title: title.to_string(),
            content: default.to_string(),
        }
    }
}
