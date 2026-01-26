use crate::app::dialog::CreateGlyphDialog;
use crate::app::popup::ExitConfirmPopup;
use crate::app::widget::{DirectoryList, SimpleButton};
use crate::app::{Command, Component, Container};
use crate::state::page::{CreateGlyphPageState, EntrancePageState, GlyphPageState, OpenGlyphPageState};
use std::path::PathBuf;

pub struct EntrancePage {
    pub components: Vec<Box<dyn Component>>,
    pub state: EntrancePageState,
}
impl EntrancePage {
    pub fn new() -> Self {
        Self {
            components: vec![
                Box::new(SimpleButton::new("Create").on_interact(Box::new(|_| {
                    Ok(vec![Command::PushPage(Box::new(CreateGlyphPage::new()))])
                }))),
                Box::new(SimpleButton::new("Open").on_interact(Box::new(|_| {
                    Ok(vec![Command::PushPage(Box::new(OpenGlyphPage::new()))])
                }))),
                Box::new(SimpleButton::new("Quit").on_interact(Box::new(|_| {
                    Ok(vec![Command::PushPopup(Box::new(ExitConfirmPopup::new(
                        true,
                    )))])
                }))),
            ],
            state: EntrancePageState {
                is_focused: true,
                is_hovered: false,
                hover_index: None,
            },
        }
    }
}
pub struct CreateGlyphPage {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: CreateGlyphPageState,
}
impl CreateGlyphPage {
    pub fn new() -> Self {
        Self {
            containers: vec![Box::new(DirectoryList::new("Directory"))],
            components: vec![
                Box::new(
                    SimpleButton::new("Back").on_interact(Box::new(|_| Ok(vec![Command::PopPage]))),
                ),
                Box::new(
                    SimpleButton::new("Create").on_interact(Box::new(|state_data| {
                        let state = state_data
                            .unwrap()
                            .downcast_mut::<CreateGlyphPageState>()
                            .unwrap();
                        Ok(vec![Command::PushDialog(Box::new(CreateGlyphDialog::new(
                            state.path_to_create.clone(),
                        )))])
                    })),
                ),
            ],
            state: CreateGlyphPageState {
                is_focused: true,
                is_hovered: false,
                hover_index: None,
                path_to_create: std::env::current_dir().unwrap(),
            },
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
            containers: vec![Box::new(DirectoryList::new("Directory"))],
            components: vec![
                Box::new(
                    SimpleButton::new("Back")
                        .on_interact(Box::new(|_| Ok(vec![Command::PopPage]))),
                ),
                Box::new(
                    SimpleButton::new("Open").on_interact(Box::new(
                            |state_data|
                                {
                                    let state = state_data
                                        .unwrap()
                                        .downcast_mut::<OpenGlyphPageState>()
                                        .unwrap();

                                    Ok(vec![
                                        Command::PushPage(
                                            Box::new(
                                                GlyphPage::new(state.path_to_open.clone())
                                            )
                                        )
                                    ])
                                }
                        )),
                ),
            ],
            state: OpenGlyphPageState {
                is_focused: true,
                is_hovered: false,
                hover_index: None,
                path_to_open: std::env::current_dir().unwrap(),
            },
        }
    }
}

pub struct GlyphPage {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphPageState
}

impl GlyphPage {
    pub fn new(root: PathBuf) -> Self {
        Self {
            containers: Vec::new(),
            components: Vec::new(),
            state: GlyphPageState {
                is_focused: false,
                is_hovered: false,
                hover_index: None,
                root_path: PathBuf::from(root),
                current_gpath: "".to_string()
            }
        }
    }
}
