use crate::app::popup::ConfirmPopup;
use crate::app::widget::{Button, DirectoryList};
use crate::app::AppCommand::{PopPage, PushPage, PushPopup};
use crate::app::Command::AppCommand;
use crate::app::{Component, Container};
use crate::model::{GlyphRepository, LocalEntryState};
use crate::state::page::{CreateGlyphPageState, EntrancePageState, GlyphMode, GlyphNavigationBarState, GlyphPageState, GlyphViewerState, OpenGlyphPageState};
use crate::state::widget::DirectoryListState;
use crate::state::AppState;
use crate::utils::cycle_offset;
use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;
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
                GlyphViewer::new(entry_state.clone()).into()
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
    Glyph Reader
 */
pub struct GlyphViewer {
    pub(crate) state: GlyphViewerState

}
impl GlyphViewer {
    pub fn new(entry_state: Rc<RefCell<LocalEntryState>>) -> Self {

        Self {
            state: GlyphViewerState {
                is_focused: false,

                edit_hovered_index: None,
                edit_selected_index: None,

                scroll_state: RefCell::new(ScrollViewState::new()),
                layout_hovered_index: None,
                layout_selected_coordinate: Vec::new(),
                mode: GlyphMode::READ,
                entry_state
            }
        }
    }
    pub(crate) fn cycle_section_hover(&mut self, offset: i16) -> () {
        let len = (&self.state.active_entry_ref().unwrap()).sections.len() as u16;
        if let Some(hover_index) = self.state.edit_hovered_index {
            self.state.edit_hovered_index = Some(cycle_offset(hover_index as u16, offset, len) as usize);
        } else {
            self.state.edit_hovered_index = Some(0);
        }
    }
    pub(crate) fn cycle_layout_hover(&mut self, offset: i16) -> () {
        let select_coordinate: &mut Vec<usize> = &mut self.state.layout_selected_coordinate;
        let id = self.state.local_entry_state_ref().unwrap().active_entry_id.unwrap();
        // When no hover index exist at that depth yet
        let len = self.state.local_entry_state_ref().unwrap().get_num_sublayout_at(&id, &self.state.layout_selected_coordinate);
        if let Some(hover_index) = self.state.layout_hovered_index{
            self.state.layout_hovered_index = Some(cycle_offset(hover_index as u16, offset, len as u16) as usize);
        } else {
            if len > 0 {
                self.state.layout_hovered_index = Some(0);
            }
        }
    }
}
impl From<GlyphViewer> for Box<dyn Container> {
    fn from(container: GlyphViewer) -> Self {
        Box::new(container)
    }
}
