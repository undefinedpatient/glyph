use crate::app::dialog::{ConfirmDialog, TextInputDialog};
use crate::app::page::{CreateGlyphPage, GlyphNavigationBar, GlyphPage, OpenGlyphPage};
use crate::app::page::{EntrancePage, GlyphEditContentView, GlyphEditOrderView, GlyphEditView, GlyphLayoutView, GlyphReadView, GlyphViewer};
use crate::app::popup::ConfirmPopup;

use crate::app::AppCommand::*;
use crate::app::Command::{self, *};
use crate::app::GlyphCommand::*;
use crate::app::PageCommand::*;

use crate::app::Convertible;
use crate::event_handler::{Focusable, Interactable};
use crate::model::{GlyphRepository, Layout, LayoutOrientation, LocalEntryState, Section};
use crate::state::dialog::TextInputDialogState;
use crate::state::page::{CreateGlyphPageState, GlyphEditContentState, GlyphEditState, GlyphLayoutState, GlyphMode, GlyphPageState};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use rusqlite::fallible_iterator::FallibleIterator;
use std::any::Any;
use std::cell::RefMut;

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
    - Remove Entry
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
                                    let selected_id: i64 = self.state.local_entry_state_ref().unwrap().ordered_entries[index].0;
                                    let mut local_entry_state = self.state.local_entry_state_mut().unwrap();
                                    local_entry_state.toggle_local_active_entry_id(selected_id)
                                }
                                return Ok(Vec::new());

                            }
                            'A' => {
                                return Ok(
                                    vec![
                                        PageCommand(
                                            PushDialog(
                                                TextInputDialog::new( "Entry Name", "untitled").on_submit(
                                                    // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                                    Box::new(|parent_state, state| {
                                                        let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                                        let mut local_entry_state = _parent_state.local_entry_state_mut().unwrap();
                                                        let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                                        let id = local_entry_state.create_new_entry(_state.text_input.as_str())?;

                                                        // Reconstruct the list of entry display
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
                                                ConfirmDialog::new( "Delete Selected Entry?", ).on_submit(
                                                    // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                                    Box::new(|parent_state, state| {
                                                        let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                                        let mut local_entry_state = _parent_state.local_entry_state_mut().unwrap();
                                                        local_entry_state.delete_active_entry()?;
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
            /*
                Switch Mode Key
             */
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Char(c) = key.code {
                        match c {
                            '`' => {
                                match self.state.mode {
                                    GlyphMode::Read => {
                                        self.state.mode = GlyphMode::Edit;
                                    }
                                    GlyphMode::Edit => {
                                        self.state.mode = GlyphMode::Layout;
                                    }
                                    GlyphMode::Layout => {
                                        self.state.mode = GlyphMode::Read;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            match self.state.mode {
                GlyphMode::Read => {
                    self.containers[0].as_mut().handle(key, parent_state)
                }
                GlyphMode::Edit => {
                    self.containers[1].as_mut().handle(key, parent_state)
                }
                GlyphMode::Layout => {
                    self.containers[2].as_mut().handle(key, parent_state)
                }
            }
        }
    }
}
impl Interactable for GlyphReadView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    self.set_focus(false);
                    return Ok(Vec::new());
                }
                Ok(Vec::new())
            }
            _ => {
                Ok(Vec::new())
            }
        }

    }
}

impl Interactable for GlyphEditOrderView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Tab = key.code {
                        self.cycle_section_hover(1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::BackTab = key.code {
                        self.cycle_section_hover(-1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            let mut editing_sid = self.state.editing_sid.borrow_mut();
                            *editing_sid = Some(self.get_sids()[index]);
                            return Ok(vec![GlyphCommand(RefreshEditSection)]);
                        }
                    }
                    if let KeyCode::Esc = key.code {
                        // Directly mutating parent state to lose focus
                        let parent_state = parent_state.unwrap().downcast_mut::<GlyphEditState>().unwrap();
                        *parent_state.is_focused.borrow_mut() = false;
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Char(c) = key.code {
                        match c {
                            '+' => {
                                if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                    return Ok(Vec::new());
                                }
                                if self.state.editing_sid.borrow().is_none() {
                                    return Ok(Vec::new());
                                }
                                let sid = self.state.editing_sid.borrow().unwrap().clone();
                                let mut section: Section = self.get_editing_section_mut().clone();
                                section.position = section.position + 1;
                                
                                let mut state = self.state.local_entry_state_mut().unwrap();
                                state.update_section_by_sid(&sid, section)?;
                                return Ok(Vec::new());

                            }
                            '-' => {
                                if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                    return Ok(Vec::new());
                                }
                                if self.state.editing_sid.borrow().is_none() {
                                    return Ok(Vec::new());
                                }
                                let sid = self.state.editing_sid.borrow().unwrap().clone();
                                let mut section: Section = self.get_editing_section_mut().clone();
                                section.position = section.position - 1;

                                let mut state = self.state.local_entry_state_mut().unwrap();
                                state.update_section_by_sid(&sid, section)?;
                                return Ok(Vec::new());
                            }
                            'j' => {
                                self.cycle_section_hover(1);
                                return Ok(Vec::new());
                            }
                            'k' => {
                                self.cycle_section_hover(-1);
                                return Ok(Vec::new());
                            }
                            'A' => {
                                let state: &mut GlyphEditState = parent_state.unwrap().downcast_mut::<GlyphEditState>().unwrap();
                                let mut entry_state: RefMut<LocalEntryState> = state.local_entry_state_mut().unwrap();
                                entry_state.create_section_to_active_entry(
                                    "Section",
                                    "Blank"
                                )?;
                                return Ok(Vec::new());
                            }
                            'e' => {
                                *self.state.focused_panel_index.borrow_mut() = 1;
                                return Ok(Vec::new());
                            }
                            _ => {
                                return Ok(Vec::new());

                            }
                        }
                    }
                    return Ok(Vec::new());
                }
                _ => {
                    Ok(Vec::new())
                }
            }
        } else {
            let index: usize = self.focused_child_index().unwrap();
            let mut result =
                self.containers[index].handle(key, Some(&mut self.state));
            result
        }
    }
}
impl Interactable for GlyphEditView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
        let focused_panel_index = *self.state.focused_panel_index.borrow();
        if focused_panel_index == 1 {
            let result = self.containers[1].as_mut().handle(key, Some(&mut self.state));
            return if result.is_err() {
                result
            } else {
                let mut processed_commands: Vec<Command> = Vec::new();
                let mut commands = result?;
                while let Some(command) = commands.pop() {
                    match command {
                        GlyphCommand(com) => {
                            match com {
                                RefreshEditSection => {
                                    (*self.containers[1]).as_any_mut().downcast_mut::<GlyphEditContentView>().unwrap().refresh_section();
                                }
                                _ => {
                                    processed_commands.insert(0, GlyphCommand(com));
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
        } else {
            let result = self.containers[0].as_mut().handle(key, Some(&mut self.state));
            return if result.is_err() {
                result
            } else {
                let mut processed_commands: Vec<Command> = Vec::new();
                let mut commands = result?;
                while let Some(command) = commands.pop() {
                    match command {
                        GlyphCommand(com) => {
                            match com {
                                RefreshEditSection => {
                                    (*self.containers[1]).as_any_mut().downcast_mut::<GlyphEditContentView>().unwrap().refresh_section();
                                }
                                _ => {
                                    processed_commands.insert(0, GlyphCommand(com));
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



impl Interactable for GlyphEditContentView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
        // When no editing section exist, none of the child widget should process user input
        let has_section_editing =  self.state.editing_sid.borrow().is_some();
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
                        // Directly mutating parent state to lose focus
                        let parent_state = parent_state.unwrap().downcast_mut::<GlyphEditState>().unwrap();
                        *parent_state.is_focused.borrow_mut() = false;
                    }
                    if let KeyCode::Char(c) = key.code {
                        match c {
                            'q' => {
                                *self.state.focused_panel_index.borrow_mut() = 0;
                                return Ok(Vec::new());
                            }
                            _ => {
                                return Ok(Vec::new());
                            }
                        }
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            if !has_section_editing {
                                return Ok(Vec::new());
                            }
                            match index {
                                0 => {
                                    self.containers[0].set_focus(true);
                                }
                                1 => {
                                    self.containers[1].set_focus(true);
                                }
                                2 => {
                                    return self.components[0].handle(key, Some(&mut self.state))
                                }
                                3 => {
                                    return self.components[1].handle(key, Some(&mut self.state))
                                }
                                _ => {
                                }
                            }
                        }
                    }
                    return Ok(Vec::new());
                }
                _ => {
                    Ok(Vec::new())
                }
            }
        } else {


            let index: usize = self.focused_child_index().unwrap();
            let mut result =
                self.containers[index].handle(key, Some(&mut self.state));
            result
        }
    }
}
impl Interactable for GlyphLayoutView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
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
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    if self.state.selected_coordinate.is_empty() {
                        self.set_focus(false);
                    } else {
                        let index = self.state.selected_coordinate.pop();
                        self.state.hovered_index = index;
                    }
                    return Ok(Vec::new());
                }
                if let KeyCode::Tab = key.code {
                    self.cycle_layout_hover(1);
                    return Ok(Vec::new());
                }
                if let KeyCode::BackTab = key.code {
                    self.cycle_layout_hover(-1);
                    return Ok(Vec::new());
                }
                if let KeyCode::Enter = key.code {
                    if let Some(hovered_index) = self.state.hovered_index{
                        self.state.selected_coordinate.push(hovered_index);
                        self.state.hovered_index = None;
                    }
                }
                if let KeyCode::Char(c) = key.code {
                    match c {
                        'j' => {
                            self.cycle_layout_hover(1);
                            return Ok(Vec::new());
                        }
                        'k' => {
                            self.cycle_layout_hover(-1);
                            return Ok(Vec::new());
                        }
                        'A' => {
                            let target_coor = self.state.selected_coordinate.clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid = state.active_entry_id.unwrap();
                            let lid = state.get_active_entry_lid().unwrap();
                            let mut layout = state.get_layout_ref(&lid).unwrap().clone();
                            layout.insert_sublayout_under(
                                Layout::new(),
                                &target_coor,
                            );
                            state.update_layout_by_lid(&lid, layout)?;
                            return Ok(Vec::new());
                        }
                        'x' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            if self.state.selected_coordinate.is_empty() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.clone();
                            let parent_index = self.state.selected_coordinate.pop();
                            self.state.hovered_index = None;
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            // Get Active eid
                            let eid = state.active_entry_id.unwrap();
                            let lid = state.get_active_entry_lid().unwrap();
                            // Update
                            let mut layout = state.get_layout_ref(&lid).unwrap().clone();
                            layout.remove_sublayout(&target_coor)?;
                            state.update_layout_by_lid(&lid, layout)?;
                            return Ok(Vec::new());
                        }
                        '+' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            // Get Active eid
                            let eid = state.active_entry_id.unwrap();
                            let lid = state.get_active_entry_lid().unwrap();
                            // Update
                            let mut layout = state.get_layout_ref(&lid).unwrap().clone();
                            let mut sublayout = layout.get_layout_at_mut(&target_coor).unwrap();
                            if sublayout.section_index.is_none() {
                                sublayout.section_index = Some(0);
                            } else {
                                sublayout.section_index = Some(sublayout.section_index.unwrap() + 1);
                            }
                            state.update_layout_by_lid(&lid, layout)?;
                            return Ok(Vec::new());

                        }
                        '-' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            // Get Active eid
                            let eid = state.active_entry_id.unwrap();
                            let lid = state.get_active_entry_lid().unwrap();
                            // Update
                            let mut layout = state.get_layout_ref(&lid).unwrap().clone();
                            let mut sublayout = layout.get_layout_at_mut(&target_coor).unwrap();
                            if let Some(index) = sublayout.section_index {
                                if index == 0 {
                                    sublayout.section_index = None;
                                } else {
                                    sublayout.section_index = Some(index - 1);
                                }
                            }
                            state.update_layout_by_lid(&lid, layout)?;
                            return Ok(Vec::new());
                        }
                        // Transpose the alignment
                        't' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            // Get Active eid
                            let eid = state.active_entry_id.unwrap();
                            let lid = state.get_active_entry_lid().unwrap();
                            // Update
                            let mut layout = state.get_layout_ref(&lid).unwrap().clone();
                            let mut sublayout = layout.get_layout_at_mut(&target_coor).unwrap();
                            match sublayout.details.orientation {
                                LayoutOrientation::Horizontal => {
                                    sublayout.details.orientation = LayoutOrientation::Vertical;
                                }
                                LayoutOrientation::Vertical => {
                                    sublayout.details.orientation = LayoutOrientation::Horizontal;

                                }
                            }
                            state.update_layout_by_lid(&lid, layout)?;
                            return Ok(Vec::new());

                        }
                        'r' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }

                            // This code retrieve the Original Layout Label
                            let state = self.state.local_entry_state_ref().unwrap();
                            let target_co: &Vec<usize> = &self.state.selected_coordinate;
                            let eid: &i64 = &state.active_entry_id.unwrap();
                            let layout: Layout = state.get_entry_layout_ref(eid).unwrap().clone();
                            let original_name: String = layout.get_layout_at_ref(target_co).unwrap().label.clone();
                            self.dialogs.push(
                                TextInputDialog::new( "Rename Layout", original_name.as_str()).on_submit(
                                    // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                    Box::new(|parent_state, state| {
                                        let _parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutState>().unwrap();
                                        let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();


                                        let target_coord = _parent_state.selected_coordinate.clone();
                                        let mut local_entry_state = _parent_state.local_entry_state_mut().unwrap();

                                        let eid : i64 = local_entry_state.active_entry_id.unwrap();
                                        let lid: i64 = local_entry_state.get_active_entry_lid().unwrap();
                                        let mut new_layout = local_entry_state.get_entry_layout_ref(&eid).unwrap().clone();
                                        new_layout.get_layout_at_mut(&target_coord).unwrap().label = _state.text_input.clone();


                                        local_entry_state.update_layout_by_lid(&lid, new_layout)?;

                                        Ok(vec![])
                                    })
                                ).into()
                            );
                        }
                        _ => {}
                    }
                }
                return Ok(Vec::new());
                Ok(Vec::new())
            }
            _ => {
                Ok(Vec::new())
            }
        }

    }
}