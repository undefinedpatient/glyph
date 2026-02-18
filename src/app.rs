use rusqlite::Connection;
use std::any::Any;
use std::path::PathBuf;


use crate::drawer::Drawable;
use crate::event_handler::Interactable;
use crate::theme::Iceberg;
use crate::focus_handler::Focusable;
use crate::page::entrance_page::EntrancePage;
use crate::page::glyph_page::GlyphPage;

pub enum Command {
    AppCommand(AppCommand),
    GlyphCommand(GlyphCommand),
    PageCommand(PageCommand),
    Data(Box<dyn Any>),
}
pub enum AppCommand {
    Quit,
    PushPage(Box<dyn Container>),
    PopPage,
    PushPopup(Box<dyn Container>),
    PopPopup,

}
pub enum GlyphCommand {
    OpenGlyph(PathBuf), // Path to Glyph DB
    CreateGlyph(PathBuf, String), // Path to directory, name of DB
    CreateEntry(String),
    SetEntryUnsavedState(i64, bool),
    RefreshEditSectionEditor,
    RefreshLayoutEditPanel
}
pub enum PageCommand {
    PushDialog(Box<dyn Container>),
    PopDialog,
}

pub trait Convertible {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T: Any> Convertible for T {
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any
    where
        Self: Sized,
    {
        self
    }
}
pub trait Component: Interactable + Drawable {
    fn as_interactable_ref(&self) -> &dyn Interactable;
    fn as_interactable_mut(&mut self) -> &mut dyn Interactable;
    fn as_drawable_ref(&self) -> &dyn Drawable;
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable;
    fn as_component_ref(&self) -> &dyn Component;
    fn as_component_mut(&mut self) -> &mut dyn Component;
}
pub trait Container: Interactable + Drawable + Focusable {
    fn as_interactable_ref(&self) -> &dyn Interactable;
    fn as_interactable_mut(&mut self) -> &mut dyn Interactable;
    fn as_focusable_ref(&self) -> &dyn Focusable;
    fn as_focusable_mut(&mut self) -> &mut dyn Focusable;
    fn as_drawable_ref(&self) -> &dyn Drawable;
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable;
    fn as_view_ref(&self) -> &dyn Container;
    fn as_view_mut(&mut self) -> &mut dyn Container;
}
impl<T: Interactable + Drawable> Component for T {
    fn as_interactable_ref(&self) -> &dyn Interactable {
        self
    }
    fn as_interactable_mut(&mut self) -> &mut dyn Interactable {
        self
    }
    fn as_drawable_ref(&self) -> &dyn Drawable {
        self
    }
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable {
        self
    }
    fn as_component_ref(&self) -> &dyn Component {
        self
    }
    fn as_component_mut(&mut self) -> &mut dyn Component {
        self
    }
}
impl<T: Interactable + Drawable + Focusable> Container for T {
    fn as_interactable_ref(&self) -> &dyn Interactable {
        self
    }
    fn as_interactable_mut(&mut self) -> &mut dyn Interactable {
        self
    }
    fn as_focusable_ref(&self) -> &dyn Focusable {
        self
    }
    fn as_focusable_mut(&mut self) -> &mut dyn Focusable {
        self
    }
    fn as_drawable_ref(&self) -> &dyn Drawable {
        self
    }
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable {
        self
    }
    fn as_view_ref(&self) -> &dyn Container {
        self
    }
    fn as_view_mut(&mut self) -> &mut dyn Container {
        self
    }
}

impl Application {
    pub(crate) fn view_to_focus_ref(&self) -> Option<&dyn Container> {
        if self.popup_states.len() != 0 {
            return Some(self.popup_states.last().unwrap().as_view_ref());
        }
        if self.page_states.len() != 0 {
            return Some(self.page_states.last().unwrap().as_view_ref());
        }
        None
    }
    pub(crate) fn view_to_focus_mut(&mut self) -> Option<&mut dyn Container> {
        if self.popup_states.len() != 0 {
            return Some(self.popup_states.last_mut().unwrap().as_view_mut());
        }
        if self.page_states.len() != 0 {
            return Some((self.page_states).last_mut().unwrap().as_view_mut());
        }
        None
    }
    pub(crate) fn focused_page_index(&self) -> Option<usize> {
        if self.popup_states.len() != 0 {
            return None;
        }
        Some(self.page_states.len()-1)
    }
    pub(crate) fn focused_popup_index(&self) -> Option<usize> {
        if self.popup_states.len() == 0 {
            return None;
        }
        Some(self.popup_states.len()-1)
    }
}
// Global State of the Application
pub struct AppState {
    pub theme: Iceberg,
    pub should_quit: bool,
}
pub struct Application {
    pub page_states: Vec<Box<dyn Container>>,
    pub popup_states: Vec<Box<dyn Container>>,
    pub q_commands: Vec<Command>,
    pub state: AppState,
}

impl Application {
    pub fn new() -> Application {
        Application {
            page_states: vec![Box::new(EntrancePage::new())],
            popup_states: Vec::new(),
            state: AppState {
                theme: Iceberg,
                should_quit: false},
            q_commands: Vec::new(),
        }
    }
    pub fn from(connection: Connection) -> Application {
        Application {
            page_states: vec![EntrancePage::new().into(), GlyphPage::new(connection).into()],
            popup_states: Vec::new(),
            state: AppState {
                theme: Iceberg,
                should_quit: false},
            q_commands: Vec::new(),
        }
    }
}
