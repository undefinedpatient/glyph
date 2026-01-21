use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::widgets::Paragraph;

use crate::app::TextFieldState;
use crate::ui::popup_layouts::PopupLayout;

pub struct TextField<'a> {
    state: &'a mut TextFieldState,
}

impl<'a> TextField<'a> {
    pub fn new(state: &'a mut TextFieldState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for TextField<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.state.text_as_string()).render(area, buf);
    }
}