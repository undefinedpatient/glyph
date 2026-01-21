use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crate::app::{App, PopupConfirmView};

pub fn handle_confirm_popup(key: &KeyEvent, confirm_type: &PopupConfirmView, app: &mut App) -> color_eyre::Result<()> {
    match confirm_type {
        PopupConfirmView::Exit => {
            match key.kind {
                KeyEventKind::Press=> {
                    if let KeyCode::Char(code) = key.code {
                        return match code {
                            'y' => {
                                app.state.set_should_quit(true);
                                color_eyre::eyre::Ok(())
                            },
                            'n' => {
                                app.pop_popup();
                                color_eyre::eyre::Ok(())
                            },
                            _ => color_eyre::eyre::Ok(())
                        }
                    }
                },
                KeyEventKind::Release=> return color_eyre::eyre::Ok(()),
                KeyEventKind::Repeat=> return color_eyre::eyre::Ok(()),
            }
        }
    }
    color_eyre::eyre::Ok(())
}

pub fn hande_simple_message_popup(key: &KeyEvent, app: &mut App) -> color_eyre::Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                return match code {
                    'y' | ' ' => {
                        (&mut *app).pop_popup();
                        color_eyre::eyre::Ok(())
                    },
                    _ => color_eyre::eyre::Ok(())
                }
            }
            if let KeyCode::Enter = key.code {
                (&mut *app).pop_popup();
                return color_eyre::eyre::Ok(())
            }
        },
        KeyEventKind::Release=> return color_eyre::eyre::Ok(()),
        KeyEventKind::Repeat=> return color_eyre::eyre::Ok(()),

    }
    color_eyre::eyre::Ok(())

}
