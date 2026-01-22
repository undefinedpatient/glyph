use ratatui::prelude::Widget;
use ratatui::widgets::{Block, Clear};
use ratatui::Frame;

use ratatui::layout::{Alignment, Constraint, Flex, HorizontalAlignment, Layout, Rect};
use crate::layout::widget_layouts::*;
use crate::app::{App, view_type::DialogView};

pub struct CreateGlyphInfoDialogLayout<'a> {
    ref_mut_app: &'a mut App,
}
impl<'a> CreateGlyphInfoDialogLayout<'a>{
    pub fn new(app: &'a mut App) -> Self {
        Self { ref_mut_app: app }
    }
    pub fn draw(&mut self, frame: &mut Frame) -> () {
        let dialog_frame: Block = Block::bordered().title("Create Info");

        let dialog_area: Rect = frame.area().centered(Constraint::Length(42), Constraint::Length(9));
        let dialog_inner_area: Rect = dialog_frame.inner(dialog_area);
        let info_areas = Layout::vertical(
            [
                Constraint::Length(3), // Text Field Area
                Constraint::Length(1), //
            ]
        ).split(dialog_inner_area);
        Clear.render(dialog_area,frame.buffer_mut());
        frame.render_widget(dialog_frame, dialog_area);
        // Text Field Widget
        if let Some(dialog_state) = self.ref_mut_app.h_dialog_states.get_mut(&DialogView::CreateGlyphInfo) {
            match dialog_state {
                DialogState::CreateGlyphInfo { states: focuses, text_field_state , button_state} => {
                    frame.render_stateful_widget(TextField::new("Glyph Name"), info_areas[0], text_field_state);
                    frame.render_stateful_widget(Button::new("Create"), info_areas[1], button_state);
                }
            }
        }

    }
}
// Text Field Widget

