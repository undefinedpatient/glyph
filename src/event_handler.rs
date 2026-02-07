mod dialog;
mod page;
mod popup;
mod widget;

use crate::app::popup::MessagePopup;
use crate::app::AppCommand::*;
use crate::app::Command::*;
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
        parent_state: Option<&mut dyn Any>,
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
                return vec![AppCommand(PushPopup(
                    MessagePopup::new( report.to_string().as_str(), Color::Red).into()
                ))]}
        );
    } else if let Some(page_index) = (*app).focused_page_index() {
        commands = (*app).page_states[page_index].handle(key, Some(&mut app.state)).unwrap_or_else(
            |report|{
                return vec![AppCommand(PushPopup(
                    MessagePopup::new( report.to_string().as_str(), Color::Red).into()
                ))]}
        );
    }
    app.q_commands.append(&mut commands);
    
    // Process the Command
    while app.q_commands.len() > 0 {
        let command: Command = app.q_commands.pop().unwrap();
        match command {
            AppCommand(app_command)=> {
                match app_command {
                    PushPage(view) => {
                        app.page_states.push(view);
                    }
                    PopPage => {
                        app.page_states.pop();
                    }
                    PushPopup(popup) => {
                        app.popup_states.push(popup);
                    }
                    PopPopup => {
                        app.popup_states.pop();
                    }
                    Quit => {
                        app.state.should_quit = true;
                    }
                }
            }
            _ => {
                app.popup_states.push(
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

/*
    Helper Function
 */
pub fn is_cycle_forward_hover_key(key_event: &KeyEvent) -> bool {
    if let KeyCode::Char(c) = key_event.code {
        return match c {
            'j' => true,
            _ => false
        }
    }
    if let KeyCode::Down = key_event.code {
        return true;
    }
    if let KeyCode::Right = key_event.code {
        return true;
    }
    if let KeyCode::Tab = key_event.code {
        return true;
    }
    false
}
pub fn is_cycle_backward_hover_key(key_event: &KeyEvent) -> bool {
    if let KeyCode::Char(c) = key_event.code {
        return match c {
            'k' => true,
            _ => false
        }
    }
    if let KeyCode::Up = key_event.code {
        return true;
    }
    if let KeyCode::Left = key_event.code {
        return true;
    }
    if let KeyCode::BackTab = key_event.code {
        return true;
    }
    false
}
