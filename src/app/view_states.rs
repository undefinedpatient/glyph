use ratatui::widgets::ListState;

use crate::app::{ButtonState, Focusable, TextFieldState};

pub trait FocusHandler {
    fn previous(&mut self) -> ();
    fn next(&mut self) -> ();
    fn focused_ref(&self) -> Option<&dyn Focusable>;
    fn focused_mut(&mut self) -> Option<&mut dyn Focusable>;
}
pub enum PageState {
    Entrance {

    },
    CreateGlyph {
        list_state: ListState,
    },
}
pub enum DialogState {
    CreateGlyphInfo {
        focuses: Vec<Box<dyn Focusable>>,
        text_field_state: TextFieldState,
        button_state: ButtonState,
    }
}
impl PageState {

}
impl DialogState {

}