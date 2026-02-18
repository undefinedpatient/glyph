use std::cell::{Ref, RefCell, RefMut};
use std::path::PathBuf;
use std::rc::Rc;
use tui_scrollview::ScrollViewState;
use crate::services::LocalEntryState;

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


/*

    Glyph Views

 */

pub enum GlyphMode {
    Read,
    Layout,
    Edit,
}
// This is the mediator of all views
pub struct GlyphViewerState {
    pub is_focused: Rc<RefCell<bool>>, // Shared state across all view
    pub mode: GlyphMode,

    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}

pub struct GlyphReadState {
    pub is_focused: Rc<RefCell<bool>>, // Shared state across all view
    pub scroll_state: RefCell<ScrollViewState>,

    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}


/*
    Edit State
 */

pub struct GlyphEditState {
    pub shared_focus: Rc<RefCell<bool>>, // Shared state across all view
    pub hovered_index: Option<usize>,
    // Shared Data

    pub is_editing: bool, // It is either Ordering or Editing
    pub active_sid: Rc<RefCell<Option<i64>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}


pub struct GlyphEditOrderState {
    pub hovered_index: Option<usize>,


    // Shared Data
    pub active_sid: Rc<RefCell<Option<i64>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
/*
    Layout State
 */

pub struct GlyphLayoutState {
    pub shared_focus: Rc<RefCell<bool>>, // Shared state across all layout view
    pub is_editing: bool, // It is either Ordering or Editing

    // Shared Data
    pub selected_coordinate: Rc<RefCell<Vec<usize>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}

pub struct GlyphLayoutOverviewState {
    pub hovered_index: Option<usize>, // Note this is the hovered index for sub-layouts, not widgets.
    pub scroll_state: RefCell<ScrollViewState>,



    // Shared Data
    pub selected_coordinate: Rc<RefCell<Vec<usize>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,

}

pub struct GlyphLayoutEditState {
    pub hovered_index: Option<usize>,

    // Shared Data
    pub selected_coordinate: Rc<RefCell<Vec<usize>>>,
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
impl GlyphViewerState{
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
impl GlyphReadState{
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

impl GlyphEditState{
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
impl GlyphEditOrderState {
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
impl GlyphLayoutState{
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
impl GlyphLayoutOverviewState{
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

impl GlyphLayoutEditState{
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
