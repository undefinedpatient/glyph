use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Widget;
use ratatui::widgets::{Block, Clear};
use crate::app::App;

pub struct CreateGlyphInfoDialogLayout<'a> {
    ref_app: &'a App
}
impl<'a> CreateGlyphInfoDialogLayout<'a>{
    pub fn new(app: &'a mut App) -> Self {
        CreateGlyphInfoDialogLayout {
            ref_app: app
        }
    }
}
impl<'a> Widget for CreateGlyphInfoDialogLayout<'a>{
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        let dialog_frame: Block = Block::bordered().title("Create Info");
        let dialog_area: Rect = area.centered(Constraint::Min(16), Constraint::Length(7));
        let dialog_inner_area: Rect = dialog_frame.inner(dialog_area);
        Clear.render(area, buf);
        dialog_frame.render(area, buf);
    }
}

// Text Field Widget

