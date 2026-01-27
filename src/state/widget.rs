use std::path::PathBuf;
use crate::app::widget::GlyphNavigationBar;

struct GlyphNavigationBarState {
    pub is_focused: bool,
    pub hover_index: Option<usize>,
    pub path_buf: PathBuf,
    pub entries: Vec<GlyphNavigationBar>,
}