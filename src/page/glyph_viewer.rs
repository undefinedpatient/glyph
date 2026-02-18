use ratatui::prelude::Stylize;
use ratatui::widgets::BorderType;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::Line;
use ratatui::widgets::{Block, Paragraph, Widget};
use crate::app::{Command, Container};
use crate::app::Command::GlyphCommand;
use crate::app::GlyphCommand::SetEntryUnsavedState;
use crate::block;
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Interactable;
use crate::focus_handler::Focusable;
use crate::page::glyph_edit_view::GEditView;
use crate::page::glyph_layout_view::{GLayoutEditView, GLayoutView};
use crate::page::glyph_read_view::GReadView;
use crate::services::LocalEntryState;
use crate::theme::Theme;

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

pub struct GViewer {
    pub(crate) state: GlyphViewerState,
    pub(crate) containers: [Box<dyn Container>; 3],
}

impl From<GViewer> for Box<dyn Container> {
    fn from(container: GViewer) -> Self {
        Box::new(container)
    }
}

impl GViewer {
    pub fn new(entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        let shared_focus: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        Self {
            containers: [
                GReadView::new(shared_focus.clone(), entry_state.clone()).into(),
                GEditView::new(shared_focus.clone(), entry_state.clone()).into(),
                GLayoutView::new(shared_focus.clone(), entry_state.clone()).into(),
            ],
            state: GlyphViewerState {
                is_focused: shared_focus,
                mode: GlyphMode::Read,
                entry_state
            }
        }
    }
}
impl Drawable for GViewer {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("Content", draw_flag, theme);
        match self.state.mode {
            GlyphMode::Read => {
                widget_frame = widget_frame.title_top(Line::from("[ READ ]").right_aligned());
            }
            GlyphMode::Edit => {
                widget_frame = widget_frame.title_top(Line::from("[ EDIT ]").right_aligned());
            }
            GlyphMode::Layout => {
                widget_frame = widget_frame.title_top(Line::from("[ LAYOUT ]").right_aligned());
            }
        }
        let inner_area = widget_frame.inner(area.centered_horizontally(Constraint::Percentage(90)));
        widget_frame.render(area, frame.buffer_mut());


        /*
            Body
         */
        if self.state.local_entry_state_ref().unwrap().active_entry_id.is_none() {
            let message: Paragraph = Paragraph::new("No Entry Selected ;_;").alignment(Alignment::Center);
            let center_area = inner_area.centered(Constraint::Fill(1), Constraint::Length(3));
            message.render(center_area, frame.buffer_mut());
            return;
        }
        match self.state.mode {
            GlyphMode::Read => {
                self.containers[0].as_ref().render(frame, inner_area, draw_flag, theme);
            }
            GlyphMode::Edit => {
                self.containers[1].as_ref().render(frame, inner_area, draw_flag, theme);
            }
            GlyphMode::Layout => {
                self.containers[2].as_ref().render(frame, inner_area, draw_flag, theme);
            }
        }
    }
}
impl Interactable for GViewer {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            /*
                Switch Mode Key
             */
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Char(c) = key.code {
                        match c {
                            '\\' => {
                                match self.state.mode {
                                    GlyphMode::Read => {
                                        self.state.mode = GlyphMode::Edit;
                                    }
                                    GlyphMode::Edit => {
                                        self.state.mode = GlyphMode::Layout;

                                        // Dangerous Cheating here
                                        (*(*self.containers[2])
                                            .as_any_mut().downcast_mut::<GLayoutView>().unwrap().containers[1])
                                            .as_any_mut().downcast_mut::<GLayoutEditView>().unwrap().refresh_layout_edit_panel();
                                    }
                                    GlyphMode::Layout => {
                                        self.state.mode = GlyphMode::Read;
                                    }
                                }
                            }
                            's' => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    let mut state: RefMut<LocalEntryState> = self.state.local_entry_state_mut().unwrap();
                                    let eid = state.active_entry_id.unwrap();
                                    state.updated_entries.remove(&eid);

                                    state.save_entry_db(&eid)?;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            match self.state.mode {
                GlyphMode::Read => {
                    self.containers[0].as_mut().handle(key, parent_state)
                }
                GlyphMode::Edit => {
                    let result = self.containers[1].as_mut().handle(key, parent_state);
                    if result.is_err() {
                        return result;
                    } else {
                        let mut processed_commands: Vec<Command> = Vec::new();
                        let mut commands = result?;
                        while let Some(command) = commands.pop() {
                            match command {
                                GlyphCommand(page_command) => {
                                    match page_command {
                                        SetEntryUnsavedState(eid, is_changed)=> {
                                            let mut state = self.state.local_entry_state_mut().unwrap();
                                            if is_changed {
                                                state.updated_entries.insert(eid);
                                            } else {
                                                state.updated_entries.remove(&eid);
                                            }
                                        }
                                        _ => {
                                            processed_commands.insert(0, GlyphCommand(page_command));
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
                GlyphMode::Layout => {
                    let result = self.containers[2].as_mut().handle(key, parent_state);
                    if result.is_err() {
                        return result;
                    } else {
                        let mut processed_commands: Vec<Command> = Vec::new();
                        let mut commands = result?;
                        while let Some(command) = commands.pop() {
                            match command {
                                GlyphCommand(page_command) => {
                                    match page_command {
                                        SetEntryUnsavedState(eid, is_changed)=> {
                                            let mut state = self.state.local_entry_state_mut().unwrap();
                                            if is_changed {
                                                state.updated_entries.insert(eid);
                                            } else {
                                                state.updated_entries.remove(&eid);
                                            }
                                        }
                                        _ => {
                                            processed_commands.insert(0, GlyphCommand(page_command));
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
        }
    }
}
impl Focusable for GViewer {
    fn is_focused(&self) -> bool {
        self.state.is_focused.borrow().clone()
    }
    fn set_focus(&mut self, value: bool) -> () {
        let mut focus = self.state.is_focused.borrow_mut();
        *focus = value;
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
