mod page;
mod popup;
mod widget;

use crate::app::popup::MessagePopup;
use crate::app::{Application, Command, Convertible, Container};
use crate::drawer::Drawable;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::any::Any;

pub trait Interactable: Convertible {
    fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command>;
}
pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focus(&mut self, value: bool) -> ();
    fn focused_child_ref(&self) -> Option<&dyn Container>;
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container>;
}
pub fn handle_key_events(key: &KeyEvent, app: &mut Application) -> () {
    handle_global_events(key, app);
    let mut command: Option<Command> = None;
    if let Some(stateful) = (*app).view_to_focus_mut(){
        let result: color_eyre::Result<Command> = (*stateful).as_interactable_mut().handle(key);
        if (&result).is_ok() {
            command = Some(result.unwrap());
        } else {
            command = Some(
                Command::PushPopup(
                    Box::new(
                        MessagePopup::new(
                            result.err().unwrap().to_string().as_str()
                        )
                    )
                )
            )
        }
    }
    if let Some(c) = command {
        app.q_commands.push(c);
    }
    if app.q_commands.len() > 0 {
        let command: Command = app.q_commands.pop().unwrap();
        match command {
            Command::PushView(view) => {
                app.page_states.push(view);
            }
            Command::PopView => {
                app.page_states.pop();
            }
            Command::PushPopup(popup) => {
                app.popup_states.push(popup);
            }
            Command::PopPopup => {
                app.popup_states.pop();
            }
            Command::Quit => {
                app.state.should_quit = true;
            }
            Command::None => {

            }
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
                    2 => {
                    }
                    3 => {
                    }
                    _ => {}
                }

            }
        }
        _ => {}
    }
}