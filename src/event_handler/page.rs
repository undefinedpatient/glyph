use crate::app::dialog::{ConfirmDialog, TextInputDialog};
use crate::app::page::{CreateGlyphPage, GlyphLayoutEditView, GlyphLayoutOverview, GlyphNavigationBar, GlyphPage, OpenGlyphPage};
use crate::app::page::{EntrancePage, GlyphEditContentView, GlyphEditOrderView, GlyphEditView, GlyphLayoutView, GlyphReadView, GlyphViewer};
use crate::app::popup::ConfirmPopup;

use crate::app::AppCommand::*;
use crate::app::Command::{self, *};
use crate::app::GlyphCommand::*;
use crate::app::PageCommand::*;

use crate::app::Convertible;
use crate::event_handler::{is_cycle_backward_hover_key, is_cycle_forward_hover_key, Focusable, Interactable};
use crate::model::{Entry, GlyphRepository, Layout, LayoutOrientation, LocalEntryState, Section};
use crate::state::dialog::TextInputDialogState;
use crate::state::page::{CreateGlyphPageState, GlyphEditState, GlyphLayoutState, GlyphMode, GlyphPageState};
use crate::state::AppState;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rusqlite::fallible_iterator::FallibleIterator;
use std::any::Any;
use std::cell::{Ref, RefMut};

impl Interactable for EntrancePage {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if is_cycle_forward_hover_key(key) {
                    self.cycle_hover(1);
                }
                if is_cycle_backward_hover_key(key) {
                    self.cycle_hover(-1);
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![
                        AppCommand(PushPopup(
                            ConfirmPopup::new("Exit Glyph?").on_confirm(
                                Box::new(
                                    |app_state| {
                                        let _app_state = app_state.unwrap().downcast_mut::<AppState>().unwrap();
                                        _app_state.should_quit = true;
                                        Ok(Vec::new())
                                    }
                                )
                            ).into()
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
                    if is_cycle_forward_hover_key(key) {
                        self.cycle_hover(1);
                    }
                    if is_cycle_backward_hover_key(key) {
                        self.cycle_hover(-1);
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
                    if is_cycle_forward_hover_key(key) {
                        self.cycle_hover(1);
                    }
                    if is_cycle_backward_hover_key(key) {
                        self.cycle_hover(-1);
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
                    if is_cycle_forward_hover_key(key) {
                        self.cycle_hover(1);
                    }
                    if is_cycle_backward_hover_key(key) {
                        self.cycle_hover(-1);
                    }
                    if let KeyCode::Esc = key.code {
                        if !self.state.local_entry_state_ref().unwrap().updated_entries.is_empty() {
                            self.dialogs.push(
                                ConfirmDialog::new(
                                    "You have unsaved change! Exit anyway?"
                                ).on_submit(
                                    Box::new(|_,_| {
                                        Ok(vec![AppCommand(PopPage)])
                                    })
                                )
                                    .into()
                            );
                        } else {
                            return Ok(vec![AppCommand(PopPage)]);
                        }
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
 */

impl Interactable for GlyphNavigationBar {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            match key.kind {
                KeyEventKind::Press => {
                    if is_cycle_forward_hover_key(key) {
                        self.next_entry();
                    }
                    if is_cycle_backward_hover_key(key) {
                        self.previous_entry();
                    }
                    if let KeyCode::Esc = key.code {
                        self.set_focus(false);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Enter = key.code {
                        if self.state.hovered_index.is_none() {

                        } else {
                            let index: usize = self.state.hovered_index.unwrap();
                            let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                            let selected_id: i64 = self.state.local_entry_state_ref().unwrap().ordered_entries[index].0;
                            let mut local_entry_state = self.state.local_entry_state_mut().unwrap();
                            local_entry_state.toggle_active_entry_id(selected_id)
                        }
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Char(c) = key.code {
                        match c {
                            'A' => {
                                return Ok(
                                    vec![
                                        PageCommand(
                                            PushDialog(
                                                TextInputDialog::new( "New Entry Name", "untitled").on_submit(
                                                    // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                                    Box::new(|parent_state, state| {
                                                        let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                                        let mut local_entry_state = _parent_state.local_entry_state_mut().unwrap();
                                                        let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                                        let id = local_entry_state.create_default_entry_db(_state.text_input.as_str())?;

                                                        // Reconstruct the list of entry display
                                                        Ok(vec![])
                                                    })
                                                ).into()
                                            )
                                        )
                                    ]
                                );
                            }
                            'R' => {
                                let active_entry_name: String = self.get_focused_entry_ref().unwrap().entry_name.clone();
                                return Ok(
                                    vec![
                                        PageCommand(
                                            PushDialog(
                                                TextInputDialog::new( "Rename Entry", active_entry_name.as_str()).on_submit(
                                                    // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                                    Box::new(|parent_state, state| {
                                                        let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                                        let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                                                        let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                                        let new_entry_name: &str = _state.text_input.as_str();
                                                        let mut eid: i64 = local_entry_state.active_entry_id.unwrap();
                                                        local_entry_state.update_entry_name_db(&eid, new_entry_name)?;
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
                                                        local_entry_state.delete_active_entry_db()?;
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

                                        // Dangerous Cheating here
                                        (*(*self.containers[2])
                                            .as_any_mut().downcast_mut::<GlyphLayoutView>().unwrap().containers[1])
                                            .as_any_mut().downcast_mut::<GlyphLayoutEditView>().unwrap().refresh_layout_edit_panel();
                                    }
                                    GlyphMode::Layout => {
                                        self.state.mode = GlyphMode::Read;
                                    }
                                }
                            }
                            's' => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    let mut state: RefMut<LocalEntryState> = self.state.local_entry_state_mut().unwrap();
                                    let eid = state.active_entry_id.unwrap();
                                    state.updated_entries.remove(&eid);

                                    state.save_entry_db(&eid)?;
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
                    let result = self.containers[1].as_mut().handle(key, parent_state);
                    if result.is_err() {
                        return result;
                    } else {
                        let mut processed_commands: Vec<Command> = Vec::new();
                        let mut commands = result?;
                        while let Some(command) = commands.pop() {
                            match command {
                                GlyphCommand(page_command) => {
                                    match page_command {
                                        SetEntryUnsavedState(eid, is_changed)=> {
                                            let mut state = self.state.local_entry_state_mut().unwrap();
                                            if is_changed {
                                                state.updated_entries.insert(eid);
                                            } else {
                                                state.updated_entries.remove(&eid);
                                            }
                                        }
                                        _ => {
                                            processed_commands.insert(0, GlyphCommand(page_command));
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
                GlyphMode::Layout => {
                    let result = self.containers[2].as_mut().handle(key, parent_state);
                    if result.is_err() {
                        return result;
                    } else {
                        let mut processed_commands: Vec<Command> = Vec::new();
                        let mut commands = result?;
                        while let Some(command) = commands.pop() {
                            match command {
                                GlyphCommand(page_command) => {
                                    match page_command {
                                        SetEntryUnsavedState(eid, is_changed)=> {
                                            let mut state = self.state.local_entry_state_mut().unwrap();
                                            if is_changed {
                                                state.updated_entries.insert(eid);
                                            } else {
                                                state.updated_entries.remove(&eid);
                                            }
                                        }
                                        _ => {
                                            processed_commands.insert(0, GlyphCommand(page_command));
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
                if let KeyCode::PageUp = key.code {
                    self.state.scroll_state.borrow_mut().scroll_page_up();
                    return Ok(Vec::new());
                }
                if let KeyCode::PageDown = key.code {
                    self.state.scroll_state.borrow_mut().scroll_page_down();
                    return Ok(Vec::new());
                }
                if let KeyCode::Up = key.code {
                    self.state.scroll_state.borrow_mut().scroll_up();
                    return Ok(Vec::new());
                }
                if let KeyCode::Down = key.code {
                    self.state.scroll_state.borrow_mut().scroll_down();
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
        match key.kind {
            KeyEventKind::Press => {
                if is_cycle_forward_hover_key(key) {
                    self.cycle_section_hover(1);
                }
                if is_cycle_backward_hover_key(key) {
                    self.cycle_section_hover(-1);
                }
                if let KeyCode::Enter = key.code {
                    if let Some(index) = self.state.hovered_index {
                        let state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
                        let eid = state.active_entry_id.unwrap();
                        let sections: &Vec<(i64, Section)> = state.get_sections_ref(&eid);
                        if self.state.editing_sid.borrow().is_some() {
                            if self.state.editing_sid.borrow().unwrap() == sections.get(index).unwrap().0 {
                                *self.state.editing_sid.borrow_mut() = None;
                                return Ok(vec![GlyphCommand(RefreshEditSection)]);
                            }
                        }
                        *self.state.editing_sid.borrow_mut() = Some((*sections.get(index).unwrap()).0);
                        return Ok(vec![GlyphCommand(RefreshEditSection)]);
                    }
                }
                if let KeyCode::Esc = key.code {
                    // Directly mutating parent state to lose focus
                    let parent_state = parent_state.unwrap().downcast_mut::<GlyphEditState>().unwrap();
                    *parent_state.shared_focus.borrow_mut() = false;
                    return Ok(Vec::new());
                }
                if let KeyCode::Char(c) = key.code {
                    match c {
                        'e' => {
                            *self.state.focused_panel_index.borrow_mut() = 1;
                            return Ok(Vec::new());
                        }
                        '+' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            if self.state.editing_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let mut section: RefMut<Section> = self.get_editing_section_mut();
                            section.position = section.position + 1;
                            drop(section);
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            state.sort_sections_by_position(&eid);
                            drop(state);

                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);

                        }
                        '-' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            if self.state.editing_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let mut section: RefMut<Section> = self.get_editing_section_mut();
                            section.position = section.position - 1;
                            drop(section);
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            state.sort_sections_by_position(&eid);
                            drop(state);
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        'x' => {
                            if self.state.editing_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let sid: i64 = self.state.editing_sid.borrow().as_ref().unwrap().clone();
                            self.state.editing_sid.replace(None);
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            state.delete_section_db(&eid, &sid)?;
                            return Ok(Vec::new());
                        }
                        'A' => {
                            let mut local_entry_state: RefMut<LocalEntryState> = self.state.local_entry_state_mut().unwrap();
                            local_entry_state.create_section_to_active_entry_db(
                                "untitled",
                                "Blank"
                            )?;
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
                    if is_cycle_forward_hover_key(key) {
                        self.cycle_hover(1);
                    }
                    if is_cycle_backward_hover_key(key) {
                        self.cycle_hover(-1);
                    }
                    if let KeyCode::Esc = key.code {
                        // Directly mutating parent state to lose focus
                        let parent_state = parent_state.unwrap().downcast_mut::<GlyphEditState>().unwrap();
                        *parent_state.shared_focus.borrow_mut() = false;
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
                                RefreshLayoutEditPanel => {
                                    (*self.containers[1]).as_any_mut().downcast_mut::<GlyphLayoutEditView>().unwrap().refresh_layout_edit_panel();
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
                                RefreshLayoutEditPanel => {
                                    (*self.containers[1]).as_any_mut().downcast_mut::<GlyphLayoutEditView>().unwrap().refresh_layout_edit_panel();
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
impl Interactable for GlyphLayoutOverview {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    return if !self.state.selected_coordinate.borrow_mut().is_empty() {
                        let index = self.state.selected_coordinate.borrow_mut().pop();
                        self.state.hovered_index = index;
                        Ok(vec![GlyphCommand(RefreshLayoutEditPanel)])
                    } else {
                        let parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutState>().unwrap();
                        *parent_state.shared_focus.borrow_mut() = false;
                        Ok(Vec::new())
                    }
                }
                if let KeyCode::PageUp = key.code {
                    self.state.scroll_state.borrow_mut().scroll_page_up();
                    return Ok(Vec::new());
                }
                if let KeyCode::PageDown = key.code {
                    self.state.scroll_state.borrow_mut().scroll_page_down();
                    return Ok(Vec::new());
                }
                if let KeyCode::Up = key.code {
                    self.state.scroll_state.borrow_mut().scroll_up();
                    return Ok(Vec::new());
                }
                if let KeyCode::Down = key.code {
                    self.state.scroll_state.borrow_mut().scroll_down();
                    return Ok(Vec::new());
                }
                // By putting Arrow key before cycle key, ignore the arrow key cycling
                if is_cycle_forward_hover_key(key) {
                    self.cycle_layout_hover(1);
                }
                if is_cycle_backward_hover_key(key) {
                    self.cycle_layout_hover(-1);
                }
                if let KeyCode::Enter = key.code {
                    if let Some(hovered_index) = self.state.hovered_index{
                        self.state.selected_coordinate.borrow_mut().push(hovered_index);
                        self.state.hovered_index = None;
                        return Ok(vec![GlyphCommand(RefreshLayoutEditPanel)]);
                    }

                }
                if let KeyCode::Char(c) = key.code {
                    match c {
                        'e' => {
                            *self.state.focused_panel_index.borrow_mut() = 1;
                            return Ok(Vec::new());
                        }
                        'A' => {
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let entry = state.get_active_entry_ref().unwrap();
                            let eid = state.active_entry_id.unwrap();
                            let layout: &mut Layout = &mut state.get_entry_mut(&eid).unwrap().layout;
                            layout.insert_sublayout_under(
                                Layout::new(""),
                                &target_coor,
                            );
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        'x' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            if self.state.selected_coordinate.borrow().is_empty() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            self.state.hovered_index = None;
                            self.state.selected_coordinate.borrow_mut().pop();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            // Get Active eid
                            let eid = state.active_entry_id.unwrap();
                            // Update
                            let layout: &mut Layout = &mut state.get_entry_mut(&eid).unwrap().layout;
                            layout.remove_sublayout(&target_coor)?;
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        '+' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            // Get Active eid
                            let entry: &mut Entry = state.get_active_entry_mut().unwrap();
                            // Update
                            let layout: &mut Layout = &mut entry.layout;
                            let sublayout: &mut Layout = layout.get_layout_at_mut(&target_coor).unwrap();
                            if sublayout.section_index.is_none() {
                                sublayout.section_index = Some(0);
                            } else {
                                sublayout.section_index = Some(sublayout.section_index.unwrap() + 1);
                            }
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        '-' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            // Get Active eid
                            let entry: &mut Entry = state.get_active_entry_mut().unwrap();
                            // Update
                            let layout: &mut Layout = &mut entry.layout;
                            let sublayout: &mut Layout = layout.get_layout_at_mut(&target_coor).unwrap();
                            if let Some(index) = sublayout.section_index {
                                if index == 0 {
                                    sublayout.section_index = None;
                                } else {
                                    sublayout.section_index = Some(index - 1);
                                }
                            }
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        // Transpose the alignment
                        't' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            // Get Active eid
                            let eid: i64 = state.active_entry_id.unwrap();
                            let entry: &mut Entry = state.get_active_entry_mut().unwrap();
                            // Update
                            let layout: &mut Layout = &mut entry.layout;
                            let mut sublayout: &mut Layout = layout.get_layout_at_mut(&target_coor).unwrap();
                            match sublayout.details.orientation {
                                LayoutOrientation::Horizontal => {
                                    sublayout.details.orientation = LayoutOrientation::Vertical;
                                }
                                LayoutOrientation::Vertical => {
                                    sublayout.details.orientation = LayoutOrientation::Horizontal;

                                }
                            }
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);

                        }
                        _ => {}
                    }
                }
                Ok(Vec::new())
            }
            _ => {
                Ok(Vec::new())
            }
        }
    }
}

impl Interactable for GlyphLayoutEditView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        let parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutState>().unwrap();
                        *parent_state.shared_focus.borrow_mut() = false;
                        return Ok(Vec::new());
                    }
                    if is_cycle_forward_hover_key(key) {
                        self.cycle_hover(1);
                        return Ok(Vec::new());
                    }
                    if is_cycle_backward_hover_key(key) {
                        self.cycle_hover(-1);
                        return Ok(Vec::new());
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
                            match index {

                                0 => { // Label Field
                                    self.containers[0].set_focus(true);
                                }
                                1 => { // Size Field
                                    return self.components[0].handle(key, Some(&mut self.state));
                                }
                                2 => { // Length Field
                                    self.containers[1].set_focus(true);
                                }
                                3 => { // Flex Field
                                    self.containers[2].set_focus(true);
                                }
                                4 => {
                                    return self.components[1].handle(key, Some(&mut self.state));
                                }
                                _ => {
                                }
                            }
                        }
                    }
                    return Ok(Vec::new());
                }
                _ => {

                    return Ok(Vec::new());
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