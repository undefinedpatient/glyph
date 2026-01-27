mod dialog;
mod page;
mod popup;
mod widget;

use crate::app::popup::MessagePopup;
use crate::app::{Application, Command, Container, Convertible};
use crate::drawer::Drawable;
use crate::utils::create_glyph;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::any::Any;
use color_eyre::owo_colors::OwoColorize;

pub trait Interactable: Convertible {
    fn handle(
        &mut self,
        key: &KeyEvent,
        data: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>>;
}
pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focus(&mut self, value: bool) -> ();
    fn focused_child_ref(&self) -> Option<&dyn Container>;
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container>;
    fn focused_child_index(&self) -> Option<usize>;
}
pub fn handle_key_events(key: &KeyEvent, app: &mut Application) -> () {
    handle_global_events(key, app);
    if let Some(stateful) = (*app).view_to_focus_mut() {
        let mut commands: Vec<Command> = (*stateful)
            .as_interactable_mut()
            .handle(key, None)
            .unwrap_or_else(|report| {
                return vec![Command::PushPopup(Box::new(MessagePopup::new(
                    report.to_string().as_str(),
                )))];
            });
        app.q_commands.append(&mut commands);
    }
    while app.q_commands.len() > 0 {
        let command: Command = app.q_commands.pop().unwrap();
        match command {
            Command::PushPage(view) => {
                app.page_states.push(view);
            }
            Command::PopPage => {
                app.page_states.pop();
            }
            Command::PushDialog(view) => {
                app.dialog_states.push(view);
            }
            Command::PopDialog => {
                app.dialog_states.pop();
            }
            Command::PushPopup(popup) => {
                app.popup_states.push(popup);
            }
            Command::PopPopup => {
                app.popup_states.pop();
            }
            Command::CreateGlyph(path_buf, name) => {
                create_glyph(&path_buf, &name);
            }
            Command::Quit => {
                app.state.should_quit = true;
            }
            Command::Data(data) => {}
            _ => {}
        }
    }
}
fn handle_global_events(key: &KeyEvent, app: &mut Application) -> () {
    match (*key).kind {
        KeyEventKind::Press => {
            if let KeyCode::F(num) = (*key).code {
                match num {
                    1 => {
                        app.state.should_quit = true;
                    }
                    2 => {}
                    3 => {}
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
