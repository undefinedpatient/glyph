use crate::app::dialog::confirm_dialog::ConfirmDialog;
use crate::app::dialog::text_input_dialog::{TextInputDialog, TextInputDialogState};
use crate::app::page::glyph_viewer::GlyphViewer;
use crate::app::AppCommand::PopPage;
use crate::app::Command::{AppCommand, PageCommand};
use crate::app::PageCommand::{PopDialog, PushDialog};
use crate::app::{get_draw_flag, is_cycle_backward_hover_key, is_cycle_forward_hover_key, Command, Component, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::block;
use crate::models::entry::Entry;
use crate::services::LocalEntryState;
use crate::theme::Theme;
use crate::utils::cycle_offset;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Offset, Rect};
use ratatui::prelude::{Line, Span, Widget};
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::Frame;
use rusqlite::Connection;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct GlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub hidden_container_index: HashSet<u8>,

    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
impl GlyphPageState {
    pub(crate) fn local_entry_state_ref(&'_  self) -> Option<Ref<'_, LocalEntryState>> {
        Ref::filter_map(
            self.entry_state.try_borrow().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
    pub(crate) fn local_entry_state_mut(&'_ mut self) -> Option<RefMut<'_, LocalEntryState>> {
        RefMut::filter_map(
            self.entry_state.try_borrow_mut().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
}
pub struct GlyphPage {
    pub dialogs: Vec<Box<dyn Container>>,
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphPageState
}

impl GlyphPage {
    pub fn new(connection: Connection) -> Self {
        let entry_state: Rc<RefCell<LocalEntryState>> = Rc::new(RefCell::new(LocalEntryState::new(connection)));
        Self {
            dialogs: Vec::new(),
            containers: vec![
                GlyphNavigationBar::new(entry_state.clone()).into(),
                GlyphViewer::new(entry_state.clone()).into(),
            ],
            components: Vec::new(),
            state: GlyphPageState {
                is_focused: false,
                is_hovered: false,
                hidden_container_index: HashSet::new(),
                hovered_index: None,
                entry_state
            }
        }
    }
    pub(crate) fn cycle_hover(&mut self, offset: i16) -> () {
        let max: u16 = (self.containers.len() + self.components.len()) as u16;
        if let Some(hover_index) = self.state.hovered_index {
            self.state.hovered_index = Some(cycle_offset(hover_index as u16, offset, max) as usize);
        } else {
            self.state.hovered_index = Some(0);
        }
    }
}

impl From<GlyphPage> for Box<dyn Container> {
    fn from(container: GlyphPage) -> Self {
        Box::new(container)
    }
}

impl Drawable for GlyphPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Outer Frame
        */
        let page_frame: Block = block!("Glyph", draw_flag, theme);



        let page_area: Rect = page_frame.inner(area);
        let mut content_areas = Layout::horizontal([Constraint::Length(24), Constraint::Min(24)]).split(page_area);
        if self.state.hidden_container_index.contains(&0u8) {
            content_areas = Layout::horizontal([Constraint::Length(0), Constraint::Min(24)]).split(page_area);
        }


        page_frame.render(area, frame.buffer_mut());
        self.containers[0].render(frame, content_areas[0], get_draw_flag(self.state.hovered_index, 0, Some(self.containers[0].is_focused())), theme);
        self.containers[1].render(frame, content_areas[1], get_draw_flag(self.state.hovered_index, 1, Some(self.containers[1].is_focused())), theme);


        /*
            Dialog
         */
        if !self.dialogs.is_empty() {
            for (i, dialog) in self.dialogs.iter().enumerate() {
                if i == self.dialogs.len() - 1 {
                    dialog.render(frame, area, DrawFlag::FOCUSED, theme);
                } else {
                    dialog.render(frame, area, DrawFlag::DEFAULT, theme);
                }
            }
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
                                    Box::new(|_, _| {
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
                    if let KeyCode::Char('b') = key.code {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            if self.state.hidden_container_index.contains(&0u8) {
                                self.state.hidden_container_index.remove(&0u8);
                            } else {
                                self.state.hidden_container_index.insert(0);
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

    fn keymap(&self) -> Vec<(&str, &str)>{
        [
            ("j/k/up/down/tab/backtab","Navigate"),
            ("c-B", "Fold Navigation Bar"),
            ("Enter","Interact"),
        ].into()
    }
}

impl Focusable for GlyphPage {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(&**container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        for (index, container) in self.containers.iter().enumerate() {
            if container.is_focused() {
                return Some(index);
            }
        }
        None
    }
}











pub struct GlyphNavigationBarState {
    pub is_focused: bool,
    pub line_height: usize,
    pub hovered_index: Option<usize>,
    pub offset: usize,
    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
impl GlyphNavigationBarState {
    pub(crate) fn local_entry_state_ref(&'_  self) -> Option<Ref<'_, LocalEntryState>> {
        Ref::filter_map(
            self.entry_state.try_borrow().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
    pub(crate) fn local_entry_state_mut(&'_ mut self) -> Option<RefMut<'_, LocalEntryState>> {
        RefMut::filter_map(
            self.entry_state.try_borrow_mut().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
}


pub struct GlyphNavigationBar {
    pub state: GlyphNavigationBarState
}

impl GlyphNavigationBar {
    pub fn new(entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        Self {
            state: GlyphNavigationBarState {
                is_focused: false,
                line_height: 1,
                hovered_index: None,
                offset: 0,

                entry_state
            }
        }
    }
    pub fn next_entry(&mut self) -> () {
        if let Ok(state) = self.state.entry_state.try_borrow() {
            let num_entries = state.ordered_entries.len();
            if let Some(index) = self.state.hovered_index {
                self.state.hovered_index = Some(cycle_offset(index as u16, 1, num_entries as u16) as usize);
            } else {
                self.state.hovered_index = Some(0);
            }
        }
    }
    pub fn previous_entry(&mut self) -> () {
        if let Ok(state) = self.state.entry_state.try_borrow() {
            let num_entries = state.ordered_entries.len();
            if let Some(index) = self.state.hovered_index {
                self.state.hovered_index = Some(cycle_offset(index as u16, -1, num_entries as u16) as usize);
            } else {
                self.state.hovered_index = Some(0);
            }
        }
    }
    pub fn get_focused_entry_ref(&'_ mut self) -> Option<Ref<'_, Entry>> {
        if let Ok(state) = self.state.entry_state.try_borrow() {
            Some(Ref::map(state, |local_entry_state| {
                local_entry_state.get_active_entry_ref().unwrap()
            }))
        } else {
            None
        }
    }
    pub fn get_focused_entry_mut(&'_ mut self) -> Option<RefMut<'_, Entry>> {
        if let Ok(state) = self.state.entry_state.try_borrow_mut() {
            Some(RefMut::map(state, |local_entry_state| {
                local_entry_state.get_active_entry_mut().unwrap()
            }))
        } else {
            None
        }
    }

}

impl From<GlyphNavigationBar> for Box<dyn Container> {
    fn from(component: GlyphNavigationBar) -> Self {
        Box::new(component)
    }
}

impl Drawable for GlyphNavigationBar {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let widget_frame: Block = block!("Entries", draw_flag, theme);
        /*
            List Items (Entry)
         */
        let ref_entry_state = self.state.entry_state.borrow();
        // let plain_entries: Vec<(i64, String)> = ref_entry_state.entries.iter().map(
        //     |(id,entry)| {
        //         (id.clone(), entry.entry_name.clone())
        //     }
        // ).collect();
        let plain_entries: &Vec<(i64, String)> = &ref_entry_state.ordered_entries;
        let mut list_items: Vec<Line> = plain_entries
            .iter()
            .enumerate()
            .map(|(i, (id, name)): (usize, &(i64, String))| {
                let is_selected = ref_entry_state.active_entry_id == Some(*id);
                let is_hovered   = self.state.hovered_index == Some(i);
                let mut line: Line;
                let prefix = match (is_selected, is_hovered) {
                    (true, true)   => " >[",
                    (true, false)  => "  [",
                    (false, true)  => " > ",
                    (false, false) => "   ",
                };
                let suffix = if is_selected { "] " } else { "  " };

                let content = format!("{prefix}{}{suffix}", name);

                let mut line = Line::from(content);
                if is_selected {
                    line = line.bold();
                }
                if self.state.local_entry_state_ref().unwrap().updated_entries.contains(&id) {
                    line.push_span(Span::from(String::from(" (Unsaved)")).italic().not_bold());
                }
                line
            }).collect();


        let inner_area: Rect = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());
        for (i, line) in list_items[self.state.offset..].iter().enumerate() {
            if i * self.state.line_height >= inner_area.height as usize {
                break;
            }
            (line as &Line).render(
                inner_area.offset(Offset {
                    x: 0,
                    y: (i * &self.state.line_height) as i32,
                }),
                frame.buffer_mut(),
            );
        }
    }

}
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
                        if self.state.hovered_index.is_some() {
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
                                                TextInputDialog::new( "New Entry Name", "untitled", Box::new(|value|{!value.is_empty()})).on_submit(
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
                            'F' => {
                                return Ok(
                                    vec![
                                        PageCommand(
                                            PushDialog(
                                                TextInputDialog::new( "Filter Entry", "", Box::new(|value|{true})).on_submit(
                                                    // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                                    Box::new(|parent_state, state| {
                                                        let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                                        let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                                                        let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                                        local_entry_state.filter_entry_order_by(
                                                            &|name|{
                                                                name.contains(_state.text_input.as_str())
                                                            }
                                                        );
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
                                                TextInputDialog::new( "Rename Entry", active_entry_name.as_str(), Box::new(|value|{!value.is_empty()})).on_submit(
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
    fn keymap(&self) -> Vec<(&str, &str)> {
        [
            ("R","Rename Active Entry"),
            ("A","Create Entry"),
            ("Enter","Open Entry"),
        ].into()
    }
}
impl Focusable for GlyphNavigationBar {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}
