use color_eyre::Report;
use edtui::{EditorState, EditorView};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::widgets::{Block, Clear};

use crate::app::{App, DialogState, DialogView};
use crate::ui::widgets::*;

pub struct CreateGlyphInfoDialogLayout<'a> {
    app: &'a mut App,
}
impl<'a> CreateGlyphInfoDialogLayout<'a>{
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
    pub fn draw(&mut self, frame: &mut Frame) -> () {
        let dialog_frame: Block = Block::bordered().title("Create Info");
        let dialog_area: Rect = frame.area().centered(Constraint::Length(42), Constraint::Length(9));
        let dialog_inner_area: Rect = dialog_frame.inner(dialog_area);

        Clear.render(dialog_area,frame.buffer_mut());
        frame.render_widget(dialog_frame, dialog_area);
        // Text Field Widget
        if let Some(dialog_state) = self.app.h_dialog_states.get_mut(&DialogView::CreateGlyphInfo) {
            match dialog_state {
                DialogState::CreateGlyphInfo { text_field_state , button_state} => {
                    let text_field = TextField::new(text_field_state);
                    frame.render_widget(text_field, dialog_inner_area);
                }
            }
        }

    }
}
// Text Field Widget

