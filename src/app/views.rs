#[derive(Clone)]
pub enum PageView {
    Entrance,
    CreateGlyph
}

#[derive(Clone)]
pub enum DialogView {
    CreateGlyphInfo,
}

#[derive(Clone)]
pub enum PopupConfirmType {
    Exit,
}
#[derive(Clone)]
pub enum PopupView {
    Info(String),
    Warning(String),
    Error(String),
    Confirm(PopupConfirmType),
}