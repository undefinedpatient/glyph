use std::path::PathBuf;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::ListState;
use crate::app::{App, DialogView, PageState, PageView, PopupConfirmView, PopupView};
use crate::utils::get_dir_names;

pub fn handle_key_events_create_glyph_page_view(key: &KeyEvent, app: &mut App) -> color_eyre::Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                match code {
                    'q' => {
                        app.pop_page();
                        return color_eyre::eyre::Ok(())
                    },
                    'k' => {
                        if let Some(state) = (&mut *app).h_page_states.get_mut(&PageView::CreateGlyph) {
                            match state {
                                PageState::CreateGlyph{list_directory} => {
                                    list_directory.select_previous();
                                },
                                _ => {

                                }

                            }
                        };
                    }
                    'j' => {
                        if let Some(state) = (&mut *app).h_page_states.get_mut(&PageView::CreateGlyph) {
                            match state {
                                PageState::CreateGlyph{list_directory} => {
                                    list_directory.select_next();
                                },
                                _ => {

                                }

                            }
                        };
                    }
                    'c' => {
                        app.push_dialog(DialogView::CreateGlyphInfo);
                    }
                    ' ' => {
                        let app_path: PathBuf = app.state.get_current_path().clone();
                        if let Some(state) = (&mut *app).h_page_states.get_mut(&PageView::CreateGlyph) {
                            match state {
                                PageState::CreateGlyph{list_directory} => {
                                    let list: Vec<String> = get_dir_names(&app_path)?;
                                    if let Some(index) = list_directory.selected() {
                                        if index == 0 {
                                            let parent_path: PathBuf = app.state.get_current_path().parent().unwrap_or(app.state.get_current_path()).to_path_buf();
                                            app.state.set_current_path(&parent_path);
                                            return color_eyre::eyre::Ok(());
                                        }
                                        let selected_dir_name: &String = &(list[index]);
                                        let new_path: PathBuf = app.state.get_current_path().join(selected_dir_name);
                                        app.state.set_current_path(&new_path);
                                    }
                                    return color_eyre::eyre::Ok(());
                                },
                                _ => {

                                }

                            }
                        };


                    }
                    _ => return color_eyre::eyre::Ok(())
                }
            }
        },
        KeyEventKind::Release=> return color_eyre::eyre::Ok(()),
        KeyEventKind::Repeat=> return color_eyre::eyre::Ok(()),
    }
    color_eyre::eyre::Ok(())

}

pub fn handle_key_events_entrance_view(key: &KeyEvent, app: &mut App) -> color_eyre::Result<()> {
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
                            app.h_page_states.insert(PageView::CreateGlyph, PageState::CreateGlyph { list_directory: ListState::default() });
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
