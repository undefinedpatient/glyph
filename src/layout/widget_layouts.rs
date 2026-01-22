use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};
use crate::app::{ButtonState, Focusable, TextFieldState};

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
pub struct Button{
    label: String,
}
impl Button{
    pub fn new(label: &str) -> Self {
        Button{
            label: label.to_string()
        }
    }
}

impl StatefulWidget for Button{
    type State = ButtonState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.is_focused() {
            Line::from(self.label.as_str()).on_light_magenta().render(area, buf);
        } else {
            Line::from(self.label.as_str()).render(area, buf);
        }
    }
}

pub struct TextField{
    label: String,
}
impl TextField {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
}
impl StatefulWidget for TextField {
    type State = TextFieldState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let frame: Block;
        if state.is_focused() {
            frame = Block::bordered().border_type(BorderType::Double).title(self.label);
        } else {
            frame = Block::bordered().title(self.label);
        }
        let text_area: Rect = frame.inner(area);
        Clear.render(area, buf);
        frame.render(area, buf);
        Paragraph::new(state.text_as_string()).render(text_area, buf);
    }
}