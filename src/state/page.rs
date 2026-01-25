use std::path::PathBuf;

pub struct EntrancePageState{
    pub is_focused: bool,
    pub is_hovered: bool,
}
pub struct CreateGlyphPageState{
    pub is_focused: bool,
    pub is_hovered: bool,
    pub path_to_create: PathBuf,
}