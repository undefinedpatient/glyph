use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::widgets::Paragraph;

use crate::app::TextFieldState;

// pub struct TextField<'a> {
//     state: &'a mut TextFieldState,
// }

// impl<'a> TextField<'a> {
//     pub fn new(state: &'a mut TextFieldState) -> Self {
//         Self { state }
//     }
// }
// impl<'a> Widget for TextField<'a> {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         Paragraph::new(self.state.text_as_string()).render(area, buf);
//     }
// }
pub struct TextField{

}
impl TextField {
    pub fn new() -> Self {
        Self {}
    }
}
impl StatefulWidget for TextField {
    type State = TextFieldState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Paragraph::new(state.text_as_string()).render(area, buf);
    }
}