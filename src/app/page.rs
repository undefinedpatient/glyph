use crate::app::popup::ConfirmPopup;
use crate::app::widget::{Button, DirectoryList};
use crate::app::{Command, Component, Container};
use crate::model::GlyphRepository;
use crate::state::page::{CreateGlyphPageState, EntrancePageState, GlyphNavigationBarState, GlyphPageState, OpenGlyphPageState};
use crate::utils::cycle_offset;
use rusqlite::Connection;
use crate::state::widget::DirectoryListState;

pub struct EntrancePage {
    pub components: Vec<Box<dyn Component>>,
    pub state: EntrancePageState,
}
impl EntrancePage {
    pub fn new() -> Self {
        Self {
            components: vec![
                Button::new("Create").on_interact(Box::new(|_| {
                    Ok(vec![Command::PushPage(Box::new(CreateGlyphPage::new()))])
                })).into(),
                Button::new("Open").on_interact(Box::new(|_| {
                    Ok(vec![Command::PushPage(Box::new(OpenGlyphPage::new()))])
                })).into(),
                Button::new("Quit").on_interact(Box::new(|_| {
                    Ok(vec![Command::PushPopup(Box::new(ConfirmPopup::new(
                        "Exit Glyph?"
                    )))])
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
                                _parent_state.path_to_create = _state.current_path.clone();
                                Ok(Vec::new())
                            }
                        )
                    )
                    .into()
            ],
            components: vec![
                Button::new("Back").on_interact(Box::new(|_| Ok(vec![Command::PopPage]))).into(),
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
pub struct OpenGlyphPage {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: OpenGlyphPageState,
}
impl OpenGlyphPage {
    pub fn new() -> Self {
        Self {
            containers: vec![Box::new(DirectoryList::new("Directory", true,false))],
            components: vec![
                Button::new("Back")
                    .on_interact(Box::new(|_| Ok(vec![Command::PopPage]))).into(),
                Button::new("Open").on_interact(Box::new(
                    |parent_state|
                        {
                            let _parent_state = parent_state
                                .unwrap()
                                .downcast_mut::<OpenGlyphPageState>()
                                .unwrap();
                            let connection = GlyphRepository::init_glyph_db(&_parent_state.path_to_open)?;
                            Ok(vec![
                                Command::PushPage(
                                    Box::new(
                                        GlyphPage::new(connection)
                                    )
                                ),
                                Command::PopPage,
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

pub struct GlyphPage {
    pub dialogs: Vec<Box<dyn Container>>,
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphPageState
}

impl GlyphPage {
    pub fn new(connection: Connection) -> Self {
        Self {
            dialogs: Vec::new(),
            containers: vec![
                GlyphNavigationBar::new().into()
            ],
            components: Vec::new(),
            state: GlyphPageState {
                is_focused: false,
                is_hovered: false,
                hovered_index: None,
                connection
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
    pub fn new() -> Self {
        Self {
            dialogs: Vec::new(),
            state: GlyphNavigationBarState {
                is_focused: false,
                hovered_index: None,
            }
        }
    }
}

impl From<GlyphNavigationBar> for Box<dyn Container> {
    fn from(component: GlyphNavigationBar) -> Self {
        Box::new(component)
    }
}
