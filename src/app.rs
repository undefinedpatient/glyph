use rusqlite::Connection;
use std::any::Any;
use std::collections::HashMap;
use std::path::PathBuf;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};
use log::info;
use ratatui::text::{Line, Span};
use crate::theme::{Iceberg, Theme};
use page::entrance_page::EntrancePage;
use page::glyph_page::GlyphPage;
use crate::app::popup::message_popup::MessagePopup;

pub mod popup;
pub mod dialog;
pub mod widget;
pub mod page;

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
    RefreshLayoutEditPanel,
}
pub enum PageCommand {
    PushDialog(Box<dyn Container>),
    PopDialog,
}

#[macro_export]
macro_rules! block {
    ($title: expr, $flag: expr, $theme: expr) => {
        match $flag {
            DrawFlag::DEFAULT => {
                Block::bordered().title($title).style($theme.on_surface())
            }
            DrawFlag::HIGHLIGHTING => {
                Block::bordered().title(Line::from($title).bold()).border_type(BorderType::Double).style($theme.on_surface())
            }
            DrawFlag::FOCUSED => {
                Block::bordered().title(Line::from($title).bold()).border_type(BorderType::Thick).style($theme.on_surface())
            }
        }
    };
}
pub enum DrawFlag {
    DEFAULT = 0b0000_0000,
    HIGHLIGHTING = 0b0000_0001,
    FOCUSED = 0b0000_0010,
    // DISABLED = 0b0000_0100,
}

pub(crate) fn get_draw_flag(
    current_hover_index: Option<usize>,
    widget_index: usize,
    focused: Option<bool>,
) -> DrawFlag {
    if let Some(should_focus) = focused {
        if should_focus {
            return DrawFlag::FOCUSED;
        }
    }
    if let Some(index) = current_hover_index {
        if index == widget_index {
            DrawFlag::HIGHLIGHTING
        } else {
            DrawFlag::DEFAULT
        }
    } else {
        DrawFlag::DEFAULT
    }
}

pub trait Drawable {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme);
}
pub trait Interactable: Convertible {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>>;


    /// Get a descriptive key bindings action, default to none,
    /// It does nothing but telling users the key available.
    fn keymap(&self) -> Vec<(&str, &str)>{
        Vec::new()
    }

}
pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focus(&mut self, value: bool) -> ();
    fn focused_child_ref(&self) -> Option<&dyn Container>;
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container>;
    fn focused_child_index(&self) -> Option<usize>;
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

    /// Recursively find the bottom container user is interacting.
    pub(crate) fn focused_container_ref(&self) -> Option<&dyn Container> {
        let mut temp: Option<&dyn Container> = None;
        if let Some(view) = self.view_to_focus_ref() {
            temp = Some(view);
            while temp.unwrap().focused_child_ref().is_some() {
                temp = Some(temp.unwrap().focused_child_ref().unwrap());
            }
        }
        temp
    }
}
pub fn draw(frame: &mut Frame, app: &mut Application) {
    let background: Block = Block::default().bg(app.state.theme.background());
    frame.render_widget(background, frame.area());
    let vertical_constraints = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
    let [app_area, info_area]: [Rect;2] = vertical_constraints.areas(frame.area());
    let latest_page = (*app.page_states).last().unwrap();
    latest_page.as_drawable_ref().render(frame, app_area, DrawFlag::DEFAULT, &app.state.theme);
    if let Some(focused_container) = app.focused_container_ref() {
        keymap_to_line(focused_container.keymap()).render(info_area, frame.buffer_mut());
    }
    for popup in (*app.popup_states).iter_mut() {
        popup
            .as_drawable_mut()
            .render(frame, app_area, DrawFlag::DEFAULT, &app.state.theme);
    }
}
pub fn keymap_to_line<'a>(key_map: Vec<(&'a str, &'a str)>) -> Line<'a> {
    let mut line: Line = Line::default();
    for (key, value) in key_map {
        line.push_span(Span::from(key).bold());
        line.push_span(Span::from(": "));
        line.push_span(Span::from(value));
        line.push_span(Span::from("    "));
    }
    line.dim()
}
pub fn handle_key_events(key: &KeyEvent, app: &mut Application) -> () {
    handle_global_events(key, app);
    if (*app).view_to_focus_mut().is_none() {
        return;
    }

    // Retrieve the Command from Page/Popup
    let mut commands: Vec<Command> = Vec::new();
    if let Some(popup_index) = (*app).focused_popup_index() {
        commands = (*app).popup_states[popup_index].handle(key, Some(&mut app.state)).unwrap_or_else(
            |report|{
                return vec![Command::AppCommand(AppCommand::PushPopup(
                    MessagePopup::new( report.to_string().as_str(), Color::Red).into()
                ))]}
        );
    } else if let Some(page_index) = (*app).focused_page_index() {
        commands = (*app).page_states[page_index].handle(key, Some(&mut app.state)).unwrap_or_else(
            |report|{
                return vec![Command::AppCommand(AppCommand::PushPopup(
                    MessagePopup::new( report.to_string().as_str(), Color::Red).into()
                ))]}
        );
    }
    app.q_commands.append(&mut commands);

    process_command(app);
}
fn process_command(app: &mut Application) {
    // Process the Command
    while app.q_commands.len() > 0 {
        let command: Command = app.q_commands.pop().unwrap();
        match command {
            Command::AppCommand(app_command)=> {
                match app_command {
                    AppCommand::PushPage(view) => {
                        app.page_states.push(view);
                    }
                    AppCommand::PopPage => {
                        app.page_states.pop();
                    }
                    AppCommand::PushPopup(popup) => {
                        app.popup_states.push(popup);
                    }
                    AppCommand::PopPopup => {
                        app.popup_states.pop();
                    }
                    AppCommand::Quit => {
                        app.state.should_quit = true;
                    }
                }
            }
            _ => {
                app.popup_states.push(
                    MessagePopup::new( "Unexpected Command!", Color::Red).into()
                );

            }
        }
    }

}
fn handle_global_events(key: &KeyEvent, app: &mut Application) -> () {
    match (*key).kind {
        KeyEventKind::Press => {
            if let KeyCode::F(num) = (*key).code {
                match num {
                    1 => {
                        app.state.should_quit = true;
                    }
                    2 => {}
                    3 => {}
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

/*
    Helper Function
 */
pub fn is_cycle_forward_hover_key(key_event: &KeyEvent) -> bool {
    if let KeyCode::Char(c) = key_event.code {
        return match c {
            'j' => true,
            _ => false
        }
    }
    if let KeyCode::Down = key_event.code {
        return true;
    }
    if let KeyCode::Right = key_event.code {
        return true;
    }
    if let KeyCode::Tab = key_event.code {
        return true;
    }
    false
}

pub fn is_cycle_backward_hover_key(key_event: &KeyEvent) -> bool {
    if let KeyCode::Char(c) = key_event.code {
        return match c {
            'k' => true,
            _ => false
        }
    }
    if let KeyCode::Up = key_event.code {
        return true;
    }
    if let KeyCode::Left = key_event.code {
        return true;
    }
    if let KeyCode::BackTab = key_event.code {
        return true;
    }
    false
}
