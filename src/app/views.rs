#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum PageView {
    Entrance,
    CreateGlyph
}

#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum DialogView {
    CreateGlyphInfo,
}

#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum PopupConfirmView {
    Exit,
}
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum PopupView {
    Info(String),
    Warning(String),
    Error(String),
    Confirm(PopupConfirmView),
}

pub enum View {
    Page(PageView),
    Dialog(DialogView),
    Popup(PopupView),
}