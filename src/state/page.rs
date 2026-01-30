use crate::model::{Entry, LocalEntryState};
use rusqlite::Connection;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use crate::state::widget::EditorState;

pub struct EntrancePageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
}
pub struct CreateGlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub path_to_create: PathBuf,
}
pub struct OpenGlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub path_to_open: PathBuf,
}
pub struct GlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub connection: Connection,

    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}





pub struct GlyphNavigationBarState {
    pub is_focused: bool,
    pub line_height: usize,
    pub hovered_index: Option<usize>,
    pub offset: usize,
    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
pub enum GlyphMode {
    READ,
    LAYOUT,
    EDIT,
}
pub struct GlyphViewerState {
    pub is_focused: bool,
    pub section_hover_index: Option<usize>,
    pub mode: GlyphMode,

    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
impl GlyphViewerState {
    pub(crate) fn active_entry_mut(&mut self) -> Option<RefMut<Entry>> {
        let entry_state: RefMut<LocalEntryState> = self.entry_state.try_borrow_mut().ok()?;

        let id = &entry_state.active_entry_id?;
        RefMut::filter_map(entry_state, |state| {
            state.entries.get_mut(id)
        }).ok()
    }
    pub(crate) fn active_entry_ref(&self) -> Option<Ref<Entry>> {
        let entry_state: Ref<LocalEntryState> = self.entry_state.try_borrow().ok()?;

        let id = &entry_state.active_entry_id?;
        Ref::filter_map(entry_state, |state| {
            state.entries.get(id)
        }).ok()
    }
}
impl LocalEntryState {
}
impl GlyphPageState {
    pub(crate) fn to_entry_state_ref(&'_  self) -> Option<Ref<'_, LocalEntryState>> {
        Ref::filter_map(
            self.entry_state.try_borrow().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
    pub(crate) fn to_entry_state_mut(&'_ mut self) -> Option<RefMut<'_, LocalEntryState>> {
        RefMut::filter_map(
            self.entry_state.try_borrow_mut().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
}
impl GlyphNavigationBarState {
    pub(crate) fn to_entry_state_ref(&'_  self) -> Option<Ref<'_, LocalEntryState>> {
        Ref::filter_map(
            self.entry_state.try_borrow().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
    pub(crate) fn to_entry_state_mut(&'_ mut self) -> Option<RefMut<'_, LocalEntryState>> {
        RefMut::filter_map(
            self.entry_state.try_borrow_mut().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
}
impl GlyphViewerState {
    pub(crate) fn to_entry_state_ref(&'_  self) -> Option<Ref<'_, LocalEntryState>> {
        Ref::filter_map(
            self.entry_state.try_borrow().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
    pub(crate) fn to_entry_state_mut(&'_ mut self) -> Option<RefMut<'_, LocalEntryState>> {
        RefMut::filter_map(
            self.entry_state.try_borrow_mut().ok()?,
            |state| {
                Some(state)
            }
        ).ok()
    }
}