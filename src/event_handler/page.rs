use crate::app::dialog::{ConfirmDialog, TextInputDialog};
use crate::app::page::{CreateGlyphPage, GlyphNavigationBar, GlyphPage, GlyphViewer, OpenGlyphPage};
use crate::app::popup::ConfirmPopup;
use crate::app::page::EntrancePage;

use crate::app::AppCommand::*;
use crate::app::Command::{self, *};
use crate::app::GlyphCommand::*;
use crate::app::PageCommand::*;

use crate::event_handler::{Focusable, Interactable};
use crate::model::{Entry, EntryRepository, GlyphRepository, LocalEntryState, Section, SectionRepository};
use crate::state::dialog::TextInputDialogState;
use crate::state::page::{CreateGlyphPageState, GlyphMode, GlyphPageState};
use color_eyre::eyre::Result;
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::any::Any;
use std::cell::{Ref, RefMut};
use ratatui::prelude::Line;
use rusqlite::Connection;
use rusqlite::fallible_iterator::FallibleIterator;

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
                                1 => if self.state.local_entry_state_ref().unwrap().active_entry_id.is_some() {
                                    self.containers[1].set_focus(true)
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
                            'j' => {
                                self.next_entry();
                                return Ok(Vec::new());
                            }
                            'k' => {
                                self.previous_entry();
                                return Ok(Vec::new());
                            }
                            ' ' => {
                                if self.state.hovered_index.is_none() {

                                } else {
                                    let index: usize = self.state.hovered_index.unwrap();
                                    let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                    let selected_id: i64 = self.state.to_entry_state_ref().unwrap().ordered_entries[index].0;
                                    let mut local_entry_state = self.state.to_entry_state_mut().unwrap();
                                    local_entry_state.set_active_entry_id(selected_id)
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
                                                        let mut local_entry_state = _parent_state.local_entry_state_mut().unwrap();
                                                        let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                                        let id = local_entry_state.create_new_entry(_state.text_input.as_str())?;

                                                        // Reconstruct the list of entry display
                                                        local_entry_state.reconstruct_entry_order();

                                                        Ok(vec![])
                                                    })
                                                ).into()
                                            )
                                        )
                                    ]
                                );
                            }
                            'x' => {
                                if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                    return Ok(Vec::new());
                                }
                                return Ok(
                                    vec![
                                        PageCommand(
                                            PushDialog(
                                                ConfirmDialog::new(
                                                    "Delete Selected Entry?",
                                                ).on_submit(
                                                    // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                                    Box::new(|parent_state, state| {
                                                        let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                                        let mut local_entry_state = _parent_state.local_entry_state_mut().unwrap();
                                                        let id = local_entry_state.active_entry_id.unwrap();

                                                        let ref_connection: &Connection = &local_entry_state.connection;
                                                        EntryRepository::delete_by_id(ref_connection, &id)?;
                                                        local_entry_state.entries.remove(&id);
                                                        local_entry_state.active_entry_id = None;
                                                        local_entry_state.reconstruct_entry_order();
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

impl Interactable for GlyphViewer {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            match key.kind {
                KeyEventKind::Press => {
                    // Mode Wide Key
                    if let KeyCode::Esc = key.code {
                        self.set_focus(false);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Char(c) = key.code {
                        match c {
                            '`' => {
                                match self.state.mode {
                                    GlyphMode::READ => {
                                        self.state.mode = GlyphMode::LAYOUT;
                                    }
                                    GlyphMode::LAYOUT => {
                                        self.state.mode = GlyphMode::REORDERING;
                                    }
                                    GlyphMode::REORDERING => {
                                        self.state.mode = GlyphMode::READ;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    //


                    // Mode Specific Action
                    match self.state.mode {
                        GlyphMode::READ => {

                        }
                        GlyphMode::LAYOUT => {

                        }
                        GlyphMode::REORDERING => {
                            if let KeyCode::Tab = key.code {
                                self.cycle_section_hover(1);
                                return Ok(Vec::new());
                            }
                            if let KeyCode::BackTab = key.code {
                                self.cycle_section_hover(-1);
                                return Ok(Vec::new());
                            }
                            if let KeyCode::Char(c) = key.code {
                                match c {
                                    'j' => {
                                        self.cycle_section_hover(1);
                                        return Ok(Vec::new());
                                    }
                                    'k' => {
                                        self.cycle_section_hover(-1);
                                        return Ok(Vec::new());
                                    }
                                    'A' => {
                                        let state: &mut GlyphPageState = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                        let mut entry_state: RefMut<LocalEntryState> = state.local_entry_state_mut().unwrap();
                                        let active_entry_id: i64 = entry_state.active_entry_id.unwrap().clone();
                                        let new_section: Section = Section::new("untitled", "Write Something");
                                        let sid: i64 = SectionRepository::create_section(&entry_state.connection, &active_entry_id, &new_section)?;
                                        let active_entry = entry_state.get_active_entry_mut().unwrap();
                                        active_entry.sections.insert(sid, new_section);
                                    }
                                    ' ' => {
                                        if self.state.reordering_selected_index.is_some() {
                                            self.state.reordering_selected_index = None;
                                        } else if let Some(hovered_index) = self.state.reordering_hovered_index {
                                            self.state.reordering_selected_index = Some(hovered_index);
                                        }
                                    }

                                    _ => {

                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(Vec::new())
        }
    }
}