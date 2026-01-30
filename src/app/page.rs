use crate::app::popup::ConfirmPopup;
use crate::app::widget::{Button, DirectoryList};
use crate::app::AppCommand::{PopPage, PushPage, PushPopup};
use crate::app::Command::AppCommand;
use crate::app::{Component, Container};
use crate::model::{Entry, EntryRepository, GlyphRepository};
use crate::state::page::{CreateGlyphPageState, EntrancePageState, GlyphEditorState, GlyphNavigationBarState, GlyphPageState, GlyphReaderState, OpenGlyphPageState};
use crate::state::widget::DirectoryListState;
use crate::state::AppState;
use crate::utils::{cycle_offset};
use rusqlite::Connection;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use color_eyre::Report;
use color_eyre::eyre::Result;
use edtui::EditorState;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use crate::event_handler::Focusable;

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
        let entries: Rc<RefCell<HashMap<i64, Entry>>> = Rc::new(RefCell::new(EntryRepository::read_all(&connection).unwrap_or(HashMap::new())));
        let entry_id: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(None));
        let updated_entry_ids: Rc<RefCell<Vec<i64>>> = Rc::new(RefCell::new(Vec::new()));
        Self {
            dialogs: Vec::new(),
            containers: vec![
                GlyphNavigationBar::new(entry_id.clone(), entries.clone(), updated_entry_ids.clone()).into(),
                GlyphReader::new(entry_id.clone(), entries.clone(), updated_entry_ids.clone()).into()
            ],
            components: Vec::new(),
            state: GlyphPageState {
                is_focused: false,
                is_hovered: false,
                hovered_index: None,
                connection,

                active_entry_id: entry_id,
                entries,
                updated_entry_ids
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
    pub fn new(entry_id: Rc<RefCell<Option<i64>>>,ref_entries: Rc<RefCell<HashMap<i64, Entry>>>, updated_entry_ids: Rc<RefCell<Vec<i64>>>) -> Self {
        Self {
            dialogs: Vec::new(),
            state: GlyphNavigationBarState {
                is_focused: false,
                line_height: 1,
                hovered_index: None,
                offset: 0,

                active_entry_id: entry_id,
                ref_entries,
                updated_entry_ids
            }
        }
    }
    pub fn next_entry(&mut self) -> () {
        if let Some(index) = self.state.hovered_index {
        let num_entries = self.state.ref_entries.borrow().len();
            self.state.hovered_index = Some(cycle_offset(index as u16, 1, num_entries as u16) as usize);
        } else {
            self.state.hovered_index = Some(0);
        }
    }
    pub fn previous_entry(&mut self) -> () {
        let num_entries = self.state.ref_entries.borrow().len();
        if let Some(index) = self.state.hovered_index {
            self.state.hovered_index = Some(cycle_offset(index as u16, -1, num_entries as u16) as usize);
        } else {
            self.state.hovered_index = Some(num_entries - 1usize);
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
pub struct GlyphReader {
    pub(crate) state: GlyphReaderState

}
impl GlyphReader {
    pub fn new(entry_id: Rc<RefCell<Option<i64>>>,ref_entries: Rc<RefCell<HashMap<i64, Entry>>>, updated_entry_ids: Rc<RefCell<Vec<i64>>>) -> Self {
        Self {
            state: GlyphReaderState {
                is_focused: false,
                hovered_index: None,

                active_entry_id: entry_id,
                ref_entries,
                updated_entry_ids
            }
        }
    }
    pub fn convert_to_edit(self) -> Result<GlyphEditor> {
        let id_borrow = self.state.active_entry_id.try_borrow();
        if let Ok(id_result) = id_borrow {
            if let Some(id) = *id_result {
                let entries_borrow = self.state.ref_entries.try_borrow();
                if let Ok(result) = entries_borrow {
                    let entry = (*result).get(&id).unwrap().content;
                    Ok(GlyphEditor {
                        state: GlyphEditorState {
                            is_focused: self.state.is_focused,
                            hovered_index: self.state.hovered_index,

                            active_entry_id: self.state.active_entry_id,
                            ref_entries: self.state.ref_entries,
                            updated_entry_ids: self.state.updated_entry_ids,
                            editor_state: EditorState::default(),
                        }

                    })
                } else {
                    entries_borrow
                }
            } else {
                Err(Report::msg("No active entry"))
            }

        } else {
            id_borrow
        }
    }
}
impl From<GlyphReader> for Box<dyn Container> {
    fn from(container: GlyphReader) -> Self {
        Box::new(container)
    }
}

/*
    Glyph Editor
 */
pub struct GlyphEditor {
    pub state: GlyphEditorState
}
impl GlyphEditor {
    pub fn new(entry_id: Rc<RefCell<Option<i64>>>,ref_entries: Rc<RefCell<HashMap<i64, Entry>>>, updated_entry_ids: Rc<RefCell<Vec<i64>>>) -> Self {
        Self {
            state: GlyphEditorState{
                is_focused: false,
                hovered_index: None,

                active_entry_id: entry_id,
                ref_entries,
                updated_entry_ids,


                editor_state: EditorState::default(),

            }
        }
    }
}
impl From<GlyphEditor> for Box<dyn Container> {
    fn from(container: GlyphEditor) -> Self {
        Box::new(container)
    }
}