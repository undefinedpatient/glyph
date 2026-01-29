use crate::app::dialog::TextInputDialog;
use crate::app::page::{CreateGlyphPage, GlyphNavigationBar, GlyphPage, GlyphReader, OpenGlyphPage};
use crate::app::popup::ConfirmPopup;
use crate::app::page::EntrancePage;

use crate::app::AppCommand::*;
use crate::app::Command::{self, *};
use crate::app::GlyphCommand::*;
use crate::app::PageCommand::*;

use crate::event_handler::{Focusable, Interactable};
use crate::model::{EntryRepository, GlyphRepository};
use crate::state::dialog::TextInputDialogState;
use crate::state::page::{CreateGlyphPageState, GlyphPageState};
use color_eyre::eyre::Result;
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::any::Any;

impl Interactable for EntrancePage {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Tab = key.code {
                    self.cycle_hover(1);
                }
                if let KeyCode::BackTab = key.code {
                    self.cycle_hover(-1);
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![
                        AppCommand(PushPopup(
                            ConfirmPopup::new("").into()
                        ))
                    ]);
                }
                if let KeyCode::Enter = key.code {
                    if let Some(index) = self.state.hovered_index {
                        return self.components[index].as_mut().handle(key, None);
                    }
                }
            }
            _ => {}
        }
        Ok(Vec::new())
    }
}
impl Interactable for CreateGlyphPage {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        /*
            Page's Dialog
         */
        if !self.dialogs.is_empty() {
            let result = self.dialogs.last_mut().unwrap().handle(key, Some(&mut self.state));
            return if result.is_err() {
                result
            } else {
                let mut processed_commands: Vec<Command> = Vec::new();
                let mut commands = result?;
                while let Some(command) = commands.pop() {
                    match command {
                        PageCommand(page_command) => {
                            match page_command {
                                PopDialog => {
                                    self.dialogs.pop();
                                }
                                PushDialog(dialog) => {
                                    self.dialogs.push(dialog);
                                }
                            }
                        }
                        _ => {
                            processed_commands.insert(0, command);
                        }
                    }
                }
                Ok(processed_commands)
            }
        }
        /*
            Page
         */
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Tab = key.code {
                        self.cycle_hover(1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::BackTab = key.code {
                        self.cycle_hover(-1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![AppCommand(PopPage)]);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            match index {
                                0 => {
                                    // Directory List
                                    self.containers[index].set_focus(true);
                                }
                                1 => {
                                    // Back Button
                                    return self.components[0].handle(key, None);
                                }
                                2 => {
                                    // Create Button
                                    self.dialogs.push(
                                        TextInputDialog::new( "Glyph Name", "untitled_glyph", )
                                            .on_submit( Box::new(|parent_state, state| {
                                                let _parent_state = parent_state.unwrap().downcast_mut::<CreateGlyphPageState>().unwrap();
                                                let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                                let connection = GlyphRepository::init_glyph_db(&_parent_state.path_to_create.join(_state.text_input.clone()+".glyph"));
                                                Ok(
                                                    vec![
                                                        PageCommand(PopDialog),
                                                        AppCommand(PushPage(
                                                            GlyphPage::new(connection.unwrap()).into()
                                                        )),
                                                        AppCommand(PopPage)
                                                    ]
                                                )
                                            }
                                        )
                                    ).into());
                                    return Ok(Vec::new());
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(Vec::new())
        } else {
            let index: usize = self.focused_child_index().unwrap();
            let mut result =
                self.containers[index].handle(key, Some(&mut self.state));
            result
        }
    }
}
impl Interactable for OpenGlyphPage {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Tab = key.code {
                        self.cycle_hover(1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::BackTab = key.code {
                        self.cycle_hover(-1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![AppCommand(PopPage)]);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            match index {
                                0 => {
                                    // Directory List
                                    self.containers[index].set_focus(true);
                                }
                                1 => {
                                    // Back Button
                                    return self.components[0].handle(key, None);
                                }
                                2 => {
                                    // Open Button
                                    return self.components[1].handle(key, Some(&mut self.state));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(Vec::new())
        } else {
            let index: usize = self.focused_child_index().unwrap();
            let mut result =
                self.containers[index].handle(key, Some(&mut self.state));
            result
        }
    }
}

impl Interactable for GlyphPage {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        /*
            Process Dialog
         */
        if !self.dialogs.is_empty() {
            let result = self.dialogs.last_mut().unwrap().handle(key, Some(&mut self.state));
            return if result.is_err() {
                result
            } else {
                let mut processed_commands: Vec<Command> = Vec::new();
                let mut commands = result?;
                while let Some(command) = commands.pop() {
                    match command {
                        PageCommand(page_command) => {
                            match page_command {
                                PopDialog => {
                                    self.dialogs.pop();
                                }
                                PushDialog(dialog) => {
                                    self.dialogs.push(dialog);
                                }
                            }
                        }
                        _ => {
                            processed_commands.insert(0, command);
                        }
                    }
                }
                Ok(processed_commands)
            }
        }

        /*
            Process Page
         */
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Tab = key.code {
                        self.cycle_hover(1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::BackTab = key.code {
                        self.cycle_hover(-1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![AppCommand(PopPage)]);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            match index {
                                0 => self.containers[0].set_focus(true),
                                1 => self.containers[1].set_focus(true),
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(Vec::new())
        } else {
            /*
                Process Nested Components
             */
            let index: usize = self.focused_child_index().unwrap();
            let mut result =
                self.containers[index].handle(key, Some(&mut self.state));
            return if result.is_err() {
                result
            } else {
                let mut processed_commands: Vec<Command> = Vec::new();
                let mut commands = result?;
                while let Some(command) = commands.pop() {
                    match command {
                        PageCommand(page_command) => {
                            match page_command {
                                PopDialog => {
                                    self.dialogs.pop();
                                }
                                PushDialog(dialog) => {
                                    self.dialogs.push(dialog);
                                }
                            }
                        }
                        _ => {
                            processed_commands.insert(0, command);
                        }
                    }
                }
                Ok(processed_commands)
            }
        }
    }
}

/*
    Navigation Bar (Subpage)
    Operations:
    - Create Entry
    - Remove Entry (Todo)
    - Rename Entry (Todo)
 */

impl Interactable for GlyphNavigationBar {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            match key.kind {
                KeyEventKind::Press => {

                    if let KeyCode::Tab = key.code {
                        self.next_entry();
                        return Ok(Vec::new());
                    }
                    if let KeyCode::BackTab = key.code {
                        self.previous_entry();
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Esc = key.code {
                        self.set_focus(false);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Char(c) = key.code {
                        match c {
                            ' ' => {
                                let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                if let Some(index) = self.state.hovered_index {
                                    match _parent_state.entries.borrow().get(index) {
                                        Some(entry) => {
                                            self.state.selected_id = Some(entry.id);
                                        }
                                        None => {}
                                    }
                                }
                                return Ok(Vec::new());

                            }
                            'A' => {
                                return Ok(
                                    vec![
                                        PageCommand(
                                            PushDialog(
                                                TextInputDialog::new(
                                                    "Entry Name",
                                                    "untitled",
                                                ).on_submit(
                                                    // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                                    Box::new(|parent_state, state| {
                                                        let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                                        let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                                        let id: i64 = EntryRepository::create_entry(&_parent_state.connection, _state.text_input.as_str(), "")?;
                                                        let returned_entry = EntryRepository::read_by_id(&_parent_state.connection, id)?;
                                                        if let Some(entry) = returned_entry {
                                                            match _parent_state.entries.try_borrow_mut() {
                                                                Ok(mut entries) => {
                                                                    entries.push(entry);
                                                                }
                                                                Err(e) => {
                                                                    return Err(Report::msg("Entries is being updated somewhere at the moment!"));
                                                                }
                                                            }
                                                        }
                                                        Ok(vec![])
                                                    })
                                                ).into()
                                            )
                                        )
                                    ]
                                );
                            }
                            _ => {
                            }
                        }
                    } else {
                    }
                }
                _=>{
                }
            }
            Ok(Vec::new())
        }
    }
}

impl Interactable for GlyphReader {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        self.set_focus(false);
                        return Ok(Vec::new());
                    }
                }
                _ => {}
            }
            Ok(Vec::new())
        }
    }
}