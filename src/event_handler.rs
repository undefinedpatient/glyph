mod dialog;
mod page;
mod popup;
mod widget;

use crate::app::popup::MessagePopup;
use crate::app::{Application, Command, Container, Convertible};
use crate::drawer::Drawable;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::Color;
use std::any::Any;

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
    if (*app).view_to_focus_mut().is_none() {
        return;
    }
    
    // Retrieve the Command from Page/Popup
    let mut commands: Vec<Command> = Vec::new();
    if let Some(popup_index) = (*app).focused_popup_index() {
        commands = (*app).popup_states[popup_index].handle(key, Some(&mut app.state)).unwrap_or_else(
            |report|{
                return vec![Command::PushPopup(
                    MessagePopup::new( report.to_string().as_str(), Color::Red).into()
                )]}
        );
    } else if let Some(page_index) = (*app).focused_page_index() {
        commands = (*app).page_states[page_index].handle(key, Some(&mut app.state)).unwrap_or_else(
            |report|{
                return vec![Command::PushPopup(
                    MessagePopup::new( report.to_string().as_str(), Color::Red).into()
                )]}

        );
    }
    app.q_commands.append(&mut commands);
    
    // Process the Command
    while app.q_commands.len() > 0 {
        let command: Command = app.q_commands.pop().unwrap();
        match command {
            Command::PushPage(view) => {
                app.page_states.push(view);
            }
            Command::PopPage => {
                app.page_states.pop();
            }
            Command::PushPopup(popup) => {
                app.popup_states.push(popup);
            }
            Command::PopPopup => {
                app.popup_states.pop();
            }
            Command::Data(data) => {
                // Do nothing to data
            }
            Command::Quit => {
                app.state.should_quit = true;
            }
            _ => {
                app.page_states.push(
                    MessagePopup::new( "Unexpected Command!", Color::Red).into()
                );

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
                    2 => {}
                    3 => {}
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
