use crate::app::widget_states::ButtonState;
use crate::app::{App, view_type::{DialogView, PageView, PopupConfirmView}, TextFieldState, CreateGlyphPageState, EntrancePageState, FocusHandler, GListState};
use crate::utils::get_dir_names;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::ListState;
use std::path::PathBuf;
use crate::app::view_type::PopupView;
use crate::event_handler::EventHandler;

impl EventHandler for CreateGlyphPageState {
    fn handle(&mut self, key: &KeyEvent, app: &mut App) -> color_eyre::Result<()> {
        // Global KeyFrame, does not return
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    self.reset_focus();
                }
            }
            _ => {}
        }
        if let Some(focused_widget) = self.focused_mut() {
            if let Some(list)= focused_widget.as_any_mut().downcast_mut::<GListState>() {

            }
        }
        color_eyre::eyre::Ok(())

    }
}
impl EventHandler for EntrancePageState {
    fn handle(&mut self, key: &KeyEvent, app: &mut App) -> color_eyre::Result<()> {
        match key.kind {
            KeyEventKind::Press=> {
                if let KeyCode::Char(code) = key.code {
                    return match code {
                        'q' => {
                            app.push_popup(PopupView::Confirm(PopupConfirmView::Exit));
                            color_eyre::eyre::Ok(())
                        },
                        'a' => {
                            app.push_page(PageView::CreateGlyph);
                            // Check if the state exist
                            if let Some(states) = app.h_page_states.get_mut(&PageView::CreateGlyph) {
                                color_eyre::eyre::Ok(())
                            } else {
                                app.h_page_states.insert(PageView::CreateGlyph, PageState::CreateGlyph { list_state: ListState::default() });
                                color_eyre::eyre::Ok(())
                            }
                        },
                        _ => color_eyre::eyre::Ok(())
                    }
                }
            },
            KeyEventKind::Release=> return color_eyre::eyre::Ok(()),
            KeyEventKind::Repeat=> return color_eyre::eyre::Ok(()),
        }
        color_eyre::eyre::Ok(())
    }
}
pub fn handle_entrance(key: &KeyEvent, app: &mut App) -> Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                return match code {
                    'q' => {
                        app.push_popup(PopupView::Confirm(PopupConfirmView::Exit));
                        color_eyre::eyre::Ok(())
                    },
                    'a' => {
                        app.push_page(PageView::CreateGlyph);
                        // Check if the state exist
                        if let Some(states) = app.h_page_states.get_mut(&PageView::CreateGlyph) {
                            color_eyre::eyre::Ok(())
                        } else {
                            app.h_page_states.insert(PageView::CreateGlyph, PageState::CreateGlyph { list_state: ListState::default() });
                            color_eyre::eyre::Ok(())
                        }
                    },
                    _ => color_eyre::eyre::Ok(())
                }
            }
        },
        KeyEventKind::Release=> return color_eyre::eyre::Ok(()),
        KeyEventKind::Repeat=> return color_eyre::eyre::Ok(()),
    }
    color_eyre::eyre::Ok(())

}
