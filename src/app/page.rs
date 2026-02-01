use crate::app::popup::ConfirmPopup;
use crate::app::widget::{Button, DirectoryList, TextField};
use crate::app::AppCommand::{PopPage, PushPage, PushPopup};
use crate::app::Command::AppCommand;
use crate::app::{Component, Container};
use crate::model::{Entry, Glyph, GlyphRepository, LocalEntryState, Section};
use crate::state::page::{CreateGlyphPageState, EntrancePageState, GlyphEditContentState, GlyphMode, GlyphNavigationBarState, GlyphPageState, GlyphOldViewerState, OpenGlyphPageState, GlyphViewerState, GlyphReadState, GlyphLayoutState, GlyphEditState};
use crate::state::widget::DirectoryListState;
use crate::state::AppState;
use crate::utils::cycle_offset;
use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;
use rusqlite::types::Type::Text;
use tui_scrollview::ScrollViewState;

pub struct EntrancePage {
    pub components: Vec<Box<dyn Component>>,
    pub state: EntrancePageState,
}
impl EntrancePage {
    pub fn new() -> Self {
        Self {
            components: vec![
                Button::new("Create").on_interact(Box::new(|_| {
                    Ok(
                        vec![
                            AppCommand(
                                PushPage(
                                    CreateGlyphPage::new().into()
                                )

                            )
                        ]
                    )
                })).into(),
                Button::new("Open").on_interact(Box::new(|_| {
                    Ok(
                        vec![
                            AppCommand(
                                PushPage(
                                    OpenGlyphPage::new().into()
                                )

                            )
                        ]
                    )
                })).into(),
                Button::new("Quit").on_interact(Box::new(|_| {
                    Ok(vec![
                        AppCommand(PushPopup( ConfirmPopup::new(
                            "Exit Glyph?"
                        ).on_confirm(
                            Box::new(
                                |app_state| {
                                    let _app_state = app_state.unwrap().downcast_mut::<AppState>().unwrap();
                                    _app_state.should_quit = true;
                                    Ok(Vec::new())
                                }
                            )
                        ).into()
                        ))])
                })).into(),
            ],
            state: EntrancePageState {
                is_focused: true,
                is_hovered: false,
                hovered_index: None,
            },
        }
    }
    pub(crate) fn cycle_hover(&mut self, offset: i16) -> () {
        let max: u16 = self.components.len() as u16;
        if let Some(hover_index) = self.state.hovered_index {
            self.state.hovered_index = Some(cycle_offset(hover_index as u16, offset, max) as usize);
        } else {
            self.state.hovered_index = Some(0);
        }
    }
}
impl From<EntrancePage> for Box<dyn Container> {
    fn from(page: EntrancePage) -> Self {
        Box::new(page)
    }
}

pub struct CreateGlyphPage {
    pub dialogs: Vec<Box<dyn Container>>,
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: CreateGlyphPageState,
}
impl CreateGlyphPage {
    pub fn new() -> Self {
        Self {
            dialogs: Vec::new(),
            containers: vec![
                DirectoryList::new("Directory", false,true)
                    .on_exit(
                        Box::new(
                            |parent_state, state| {
                                let _parent_state = parent_state.unwrap().downcast_mut::<CreateGlyphPageState>().unwrap();
                                let _state = state.unwrap().downcast_mut::<DirectoryListState>().unwrap();
                                _parent_state.path_to_create = _state.selected_file_path.clone().unwrap();
                                Ok(Vec::new())
                            }
                        )
                    )
                    .into()
            ],
            components: vec![
                Button::new("Back").on_interact(Box::new(|_| Ok(vec![AppCommand(PopPage)]))).into(),
                Button::new("Create").on_interact(Box::new(|_| { Ok(Vec::new()) } )).into(),
            ],
            state: CreateGlyphPageState {
                is_focused: true,
                is_hovered: false,
                hovered_index: None,
                path_to_create: std::env::current_dir().unwrap(),
            },
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
impl From<CreateGlyphPage> for Box<dyn Container> {
    fn from(page: CreateGlyphPage) -> Self {
        Box::new(page)
    }
}
pub struct OpenGlyphPage {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: OpenGlyphPageState,
}
impl OpenGlyphPage {
    pub fn new() -> Self {
        Self {
            containers: vec![Box::new(DirectoryList::new("Directory", true,false)
                .on_exit(
                    Box::new(
                        |parent_state, state| {
                            let _parent_state = parent_state.unwrap().downcast_mut::<OpenGlyphPageState>().unwrap();
                            let _state = state.unwrap().downcast_mut::<DirectoryListState>().unwrap();
                            _parent_state.path_to_open = _state.selected_file_path.clone().unwrap();
                            Ok(Vec::new())
                        }
                    )
                ))],

            components: vec![
                Button::new("Back")
                    .on_interact(Box::new(|_| Ok(vec![AppCommand(PopPage)]))).into(),
                Button::new("Open").on_interact(Box::new(
                    |parent_state|
                        {
                            let _parent_state = parent_state
                                .unwrap()
                                .downcast_mut::<OpenGlyphPageState>()
                                .unwrap();
                            let connection = GlyphRepository::init_glyph_db(&_parent_state.path_to_open)?;
                            Ok(vec![
                                AppCommand(PushPage(
                                    Box::new(
                                        GlyphPage::new(connection)
                                    )
                                )),
                                AppCommand(PopPage)
                            ])
                        }
                ),
                ).into(),
            ],
            state: OpenGlyphPageState {
                is_focused: true,
                is_hovered: false,
                hovered_index: None,
                path_to_open: std::env::current_dir().unwrap(),
            },
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
impl From<OpenGlyphPage> for Box<dyn Container> {
    fn from(page: OpenGlyphPage) -> Self {
        Box::new(page)
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
                // GlyphOldViewer::new(entry_state.clone()).into()
            ],
            components: Vec::new(),
            state: GlyphPageState {
                is_focused: false,
                is_hovered: false,
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

/*
    Glyph Navigation Bar (SubPage)
 */

pub struct GlyphNavigationBar {
    pub dialogs: Vec<Box<dyn Container>>,
    pub state: GlyphNavigationBarState
}

impl GlyphNavigationBar {
    pub fn new(entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        Self {
            dialogs: Vec::new(),
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
            let num_entries = state.entries.len();
            if let Some(index) = self.state.hovered_index {
                self.state.hovered_index = Some(cycle_offset(index as u16, 1, num_entries as u16) as usize);
            } else {
                self.state.hovered_index = Some(0);
            }
        }
    }
    pub fn previous_entry(&mut self) -> () {
        if let Ok(state) = self.state.entry_state.try_borrow() {
            let num_entries = state.entries.len();
            if let Some(index) = self.state.hovered_index {
                self.state.hovered_index = Some(cycle_offset(index as u16, -1, num_entries as u16) as usize);
            } else {
                self.state.hovered_index = Some(0);
            }
        }
    }

}

impl From<GlyphNavigationBar> for Box<dyn Container> {
    fn from(component: GlyphNavigationBar) -> Self {
        Box::new(component)
    }
}

/*

    Glyph Viewers

 */



// Mediator
pub struct GlyphViewer {
    pub(crate) state: GlyphViewerState,
    pub(crate) containers: [Box<dyn Container>; 3],
}
pub struct GlyphReadView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphReadState,

}

pub struct GlyphLayoutView{
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphLayoutState,

}

pub struct GlyphEditView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphEditState,
}

pub struct GlyphEditContentView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphEditContentState,
}

impl GlyphViewer {
    pub fn new(entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        let focus_state = Rc::new(RefCell::new(false));
        Self {
            containers: [
                GlyphReadView::new(focus_state.clone(), entry_state.clone()).into(),
                GlyphReadView::new(focus_state.clone(), entry_state.clone()).into(),
                GlyphReadView::new(focus_state.clone(), entry_state.clone()).into(),
            ],
            state: GlyphViewerState {
                is_focused: focus_state,
                mode: GlyphMode::Read,
                entry_state: entry_state
            }
        }
    }
}



impl GlyphReadView {
    pub fn new(focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        Self {
            containers: vec![],
            components: vec![],
            state: GlyphReadState {
                is_focused: focus,


                entry_state
            }
        }
    }
}
impl GlyphLayoutView {
    pub fn new(focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        Self {
            containers: vec![],
            components: vec![],
            state: GlyphLayoutState {
                is_focused: focus,
                hovered_index: None,
                selected_coordinate: Vec::new(),

                entry_state
            }
        }
    }
}
impl GlyphEditView {
    pub fn new(focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        Self {
            containers: vec![],
            components: vec![],
            state: GlyphEditState {
                is_focused: focus,
                hovered_index: None,
                selected_sid: None,

                entry_state
            }
        }
    }
}
impl GlyphEditContentView {
    pub fn new(focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        Self {
            containers: vec![
                TextField::new("title", String::from("")).into(),
                TextField::new("Content", String::from("")).into(),
            ],
            components: vec![
                Button::new("Back").into(),
                Button::new("Confirm").into(),
            ],
            state: GlyphEditContentState {
                is_focused: focus,
                hovered_index: None,
                offset: 0,
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
impl From<GlyphViewer> for Box<dyn Container> {
    fn from(container: GlyphViewer) -> Self {
        Box::new(container)
    }
}
impl From<GlyphReadView> for Box<dyn Container> {
    fn from(container: GlyphReadView) -> Self {
        Box::new(container)
    }
}
impl From<GlyphLayoutView> for Box<dyn Container> {
    fn from(container: GlyphLayoutView) -> Self {
        Box::new(container)
    }
}
impl From<GlyphEditView> for Box<dyn Container> {
    fn from(container: GlyphEditView) -> Self {
        Box::new(container)
    }
}
impl From<GlyphEditContentView> for Box<dyn Container> {
    fn from(container: GlyphEditContentView) -> Self {
        Box::new(container)
    }
}




/*












 */
pub struct GlyphOldViewer {
    pub(crate) dialogs: Vec<Box<dyn Container>>,
    pub(crate) state: GlyphOldViewerState,
    pub(crate) mode_views: [Option<Box<dyn Container>>; 4],

}
impl GlyphOldViewer {
    pub fn new(entry_state: Rc<RefCell<LocalEntryState>>) -> Self {

        Self {
            dialogs: Vec::new(),
            state: GlyphOldViewerState {
                is_focused: false,

                edit_hovered_index: None,
                edit_selected_sid: None,

                scroll_state: RefCell::new(ScrollViewState::new()),
                layout_hovered_index: None,
                layout_selected_coordinate: Vec::new(),
                mode: GlyphMode::Read,
                entry_state
            },
            mode_views: [const { None }; 4],
        }
    }
    pub(crate) fn cycle_section_hover(&mut self, offset: i16) -> () {
        let state = self.state.local_entry_state_mut().unwrap();
        let eid = state.active_entry_id.unwrap();
        let len = state.get_local_num_sections(&eid);
        drop(state);
        if let Some(hover_index) = self.state.edit_hovered_index {
            self.state.edit_hovered_index = Some(cycle_offset(hover_index as u16, offset, len as u16) as usize);
        } else {
            self.state.edit_hovered_index = Some(0);
        }
    }
    pub(crate) fn cycle_layout_hover(&mut self, offset: i16) -> () {
        let select_coordinate: Vec<usize> = self.state.layout_selected_coordinate.clone();
        let state = self.state.local_entry_state_ref().unwrap();
        let eid = state.active_entry_id.unwrap();
        let ref_layout = state.get_entry_layout_ref(&eid).unwrap();
        let len = ref_layout.get_layout_at_ref(&select_coordinate).unwrap().sub_layouts.len();
        drop(state);
        if let Some(hover_index) = self.state.layout_hovered_index{
            self.state.layout_hovered_index = Some(cycle_offset(hover_index as u16, offset, len as u16) as usize);
        } else {
            if len > 0 {
                self.state.layout_hovered_index = Some(0);
            }
        }
    }
}
// impl From<GlyphOldViewer> for Box<dyn Container> {
//     fn from(container: GlyphOldViewer) -> Self {
//         Box::new(container)
//     }
// }
//
//


