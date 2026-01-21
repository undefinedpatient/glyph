use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use color_eyre::eyre::Result;

use crate::app::App;
pub fn handle_create_glyph_info(key: &KeyEvent, app: &mut App) -> Result<()> {
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