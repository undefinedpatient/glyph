mod entrance;

use color_eyre::eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crate::app::{Application, Command, View};

pub trait Interactable {
    fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command>;
}
pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focus(&mut self, value: bool) -> ();
}
pub fn handle_key_events(key: &KeyEvent, app: &mut Application) -> () {
    for view in (*app.views).iter_mut() {
        if view.is_focused() {
            app.q_commands.push(view.handle(key).unwrap());
        }
    }
    if app.q_commands.len() > 0 {
        let command: Command = app.q_commands.pop().unwrap();
        match command {
            Command::PushView(view) => {
                app.views.push(view);
            }
            Command::PopView => {

            }
            Command::Quit => {
                app.state.should_quit = true;
            }
            Command::None => {

            }
        }
    }
}
