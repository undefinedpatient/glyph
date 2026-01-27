use std::path::PathBuf;
use crate::app::widget::GlyphNavigationBar;

pub struct GlyphNavigationBarState {
    pub is_focused: bool,
    pub hovered_index: Option<usize>,
    pub entries: Vec<GlyphNavigationBar>,
}