use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::{App};
use crate::app::view_type::{DialogView, PageView};

pub fn handle_create_glyph_info(key: &KeyEvent, app: &mut App) -> Result<()> {
    if let Some(states) = app.h_dialog_states.get_mut(&DialogView::CreateGlyphInfo) {
        
    }
    return match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Char(code) = key.code {
                return match code {
                    'q' => {
                        app.pop_dialog();
                        Ok(())
                    }
                    _ => Ok(())
                }
            }
            Ok(())
        },
        _ => Ok(())
    }
}
