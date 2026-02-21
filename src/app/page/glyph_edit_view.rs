use crate::app::dialog::text_input_dialog::{TextInputDialog, TextInputDialogState};
use crate::app::page::glyph_page::GlyphPageState;
use crate::app::widget::text_editor::{TextEditor, TextEditorState};
use crate::app::Command::{GlyphCommand, PageCommand};
use crate::app::GlyphCommand::{RefreshEditSectionEditor, SetEntryUnsavedState};
use crate::app::PageCommand::PushDialog;
use crate::app::{get_draw_flag, is_cycle_backward_hover_key, is_cycle_forward_hover_key, Command, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::models::section::Section;
use crate::services::LocalEntryState;
use crate::theme::Theme;
use crate::utils::cycle_offset;
use crate::utils::markdown_renderer::MarkdownRenderer;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Stylize};
use ratatui::widgets::{Block, BorderType, Widget};
use ratatui::Frame;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct GlyphEditState {
    pub shared_focus: Rc<RefCell<bool>>, // Shared state across all view
    pub hovered_index: Option<usize>,
    // Shared Data

    pub is_editing: bool, // It is either Ordering or Editing
    pub active_sid: Rc<RefCell<Option<i64>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,
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


pub struct GlyphEditView {
    pub containers: Vec<Box<dyn Container>>,
    pub state: GlyphEditState,
}
impl From<GlyphEditView> for Box<dyn Container> {
    fn from(container: GlyphEditView) -> Self {
        Box::new(container)
    }
}
impl GlyphEditView {
    pub fn new(shared_focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        let editing_sid : Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(None));
        let is_editing: bool = false;
        Self {
            containers: vec![
                GlyphEditOrderView::new(editing_sid.clone(), entry_state.clone()).into(),
                TextEditor::new("Editor", "")
                    .on_exit(Box::new(
                        |parent_state, state| {
                            let _parent_state: &mut GlyphEditState = parent_state.unwrap().downcast_mut::<GlyphEditState>().unwrap();
                            // When no editing section exist
                            if _parent_state.active_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let _state: &mut TextEditorState = state.unwrap().downcast_mut::<TextEditorState>().unwrap();
                            let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.entry_state.try_borrow_mut().unwrap();
                            let eid: i64 = local_entry_state.active_entry_id.unwrap();
                            let sid: i64 = _parent_state.active_sid.borrow().unwrap();
                            let section: &mut Section = local_entry_state.get_section_mut(&eid, &sid).unwrap();
                            let mut lines: Vec<Vec<char>> = (*_state).lines.clone();
                            let line_number = lines.len();
                            for line in &mut lines[0..line_number-1] {
                                line.push('\n');
                            }
                            let buffer_content = lines.concat().iter().collect::<String>();
                            _parent_state.is_editing = false;
                            _state.is_focused = false;
                            if section.content != buffer_content {
                                section.content = buffer_content;
                                return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                            }
                            return Ok(vec![]);
                        } )
                    )
                    .into()
            ],
            state: GlyphEditState {
                shared_focus: shared_focus,
                is_editing,
                hovered_index: None,

                active_sid: editing_sid,
                entry_state
            }
        }
    }
    pub fn refresh_section_buffer(&mut self) -> () {
        match self.state.active_sid.borrow().as_ref() {
            Some(sid) => {
                let state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
                let eid: i64 = state.active_entry_id.unwrap();
                let sections: &Vec<(i64, Section)> = state.get_sections_ref(&eid);
                let section: Section = state.get_section_ref(&eid, &sid).unwrap().clone();
                drop(state);
                (*self.containers[1]).as_any_mut().downcast_mut::<TextEditor>().unwrap().replace(section.content.clone());
            }
            None => {
                (*self.containers[1]).as_any_mut().downcast_mut::<TextEditor>().unwrap().replace(String::new());
            }
        }
    }
}
impl Drawable for GlyphEditView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let edit_areas = Layout::horizontal([Constraint::Length(32), Constraint::Length(1), Constraint::Fill(1)]).split(area);
        self.containers[0].render(frame, edit_areas[0],
                                  get_draw_flag(
                                      if self.state.is_editing {
                                          Some(1)
                                      } else {
                                          Some(0)
                                      }
                                      ,0,Some(!self.state.is_editing)),
                                  theme
        );
        self.containers[1].render(frame, edit_areas[2],
                                  get_draw_flag(
                                      if self.state.is_editing {
                                          Some(1)
                                      } else {
                                          Some(0)
                                      }
                                      ,1,Some(self.state.is_editing)),
                                  theme
                                  ,
        );

    }
}

impl Interactable for GlyphEditView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        if self.state.is_editing {
            self.containers[1].handle(key, Some(&mut self.state))
        } else {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        self.state.shared_focus.replace(false);
                    }
                    if let KeyCode::Char(c) = key.code {
                        if c == 'e' && self.state.active_sid.borrow().is_some() {
                            self.state.is_editing = true;
                            self.containers[1].set_focus(true);
                        }
                    }
                }
                _ => {}
            }

            let result = self.containers[0].as_mut().handle(key, Some(&mut self.state));
            return if result.is_err() {
                result
            } else {
                let mut processed_commands: Vec<Command> = Vec::new();
                let mut commands = result?;
                while let Some(command) = commands.pop() {
                    match command {
                        GlyphCommand(com) => {
                            match com {
                                RefreshEditSectionEditor => {
                                    self.refresh_section_buffer();
                                }
                                _ => {
                                    processed_commands.insert(0, GlyphCommand(com));
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

impl Focusable for GlyphEditView {

    fn is_focused(&self) -> bool {
        self.state.shared_focus.borrow().clone()
    }
    fn set_focus(&mut self, value: bool) -> () {
        let mut focus = self.state.shared_focus.borrow_mut();
        *focus = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        if self.state.is_editing {
            Some(self.containers[1].as_ref())
        } else {
            Some(self.containers[0].as_ref())
        }
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        if self.state.is_editing {
            Some(self.containers[1].as_mut())
        } else {
            Some(self.containers[0].as_mut())
        }
    }
    fn focused_child_index(&self) -> Option<usize> {
        if self.state.is_editing {
            Some(1)
        } else {
            Some(0)
        }
    }
}



pub struct GlyphEditOrderState {
    pub hovered_index: Option<usize>,
    pub scroll_offset: usize,


    // Shared Data
    pub active_sid: Rc<RefCell<Option<i64>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
impl GlyphEditOrderState{
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
pub struct GlyphEditOrderView {
    pub state: GlyphEditOrderState,
}
impl GlyphEditOrderView {
    pub fn new(
        editing_sid: Rc<RefCell<Option<i64>>>,
        entry_state: Rc<RefCell<LocalEntryState>>,
    ) -> Self {
        Self {
            state: GlyphEditOrderState {
                hovered_index: None,
                scroll_offset: 0,

                active_sid: editing_sid,
                entry_state
            }
        }
    }
    pub(crate) fn cycle_section_hover(&mut self, offset: i16) -> () {
        let state = self.state.local_entry_state_mut().unwrap();
        let eid = state.active_entry_id.unwrap();
        let len = state.get_num_sections(&eid);
        drop(state);
        if let Some(hover_index) = self.state.hovered_index {
            self.state.hovered_index = Some(cycle_offset(hover_index as u16, offset, len as u16) as usize);
        } else {
            self.state.hovered_index = Some(0);
        }
    }

    /// Return the active selected section as Mutable Reference
    pub(crate) fn get_editing_section_mut(&'_ mut self) -> RefMut<'_, Section> {
        let editing_sid: i64 = self.state.active_sid.borrow().unwrap().clone();
        let entry_state: RefMut<LocalEntryState> = self.state.local_entry_state_mut().unwrap();
        let active_entry_id: i64 = entry_state.active_entry_id.unwrap();
        RefMut::map(entry_state, |state|{
            state.get_section_mut(&active_entry_id, &editing_sid).unwrap()
        })
    }

}

impl From<GlyphEditOrderView> for Box<dyn Container> {
    fn from(container: GlyphEditOrderView) -> Self {
        Box::new(container)
    }
}
impl Drawable for GlyphEditOrderView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let mut block: Block = Block::bordered().bg(theme.surface_low());
        match draw_flag {
            DrawFlag::DEFAULT => {

            }
            DrawFlag::HIGHLIGHTING => {

            }
            DrawFlag::FOCUSED => {
                block = block.border_type(BorderType::Thick);
            }
        }
        let inner_area: Rect = block.inner(area);
        block.render(area, frame.buffer_mut());


        /*

         */
        let state = self.state.entry_state.borrow();
        let eid: i64 = state.active_entry_id.unwrap();
        let section_list: &Vec<(i64, Section)> = state.get_sections_ref(&eid);
        let edit_area: Rect = inner_area.centered_horizontally(Constraint::Percentage(90));

        let mut stack_height = 0;
        let section_constraints: Vec<Constraint> = section_list.iter().skip(self.state.scroll_offset).take_while(
            |(i64, section)|{
                if stack_height>inner_area.height {
                    return false;
                }
                let estimate_num_of_lines: usize = section.content.lines().count().clamp(3,12) + 2; // Adding 2 counts for the border space.
                stack_height += estimate_num_of_lines as u16;
                true
            }).map(
            |(i64, section)| {
                let estimate_num_of_lines: usize = section.content.lines().count().clamp(3,12) + 2;
                Constraint::Length(estimate_num_of_lines as u16)
            }
        ).collect();
        let section_areas = Layout::vertical(section_constraints).split(edit_area);

        for (index, (sid ,section)) in section_list.iter().skip(self.state.scroll_offset).enumerate() {
            if index >= section_areas.len() {
                break;
            }
            let mut border_type = BorderType::Plain;
            if let Some(hovered_index) = self.state.hovered_index {
                if index as isize == hovered_index as isize - self.state.scroll_offset as isize {
                    border_type = BorderType::Double;
                }
            }
            if let Some(focused_sid) = *self.state.active_sid.borrow() {
                if focused_sid == *sid {
                    border_type = BorderType::Thick
                }
            }
            let block: Block = Block::bordered().border_type(
                border_type
            );
            let inner_area = block.inner(section_areas[index]);
            block.title(
                if section.title.is_empty() {
                    "<Empty Title>".dim()
                } else {
                    section.title.as_str().not_dim()

                }
            ).title_top(Line::from(String::from("Pos: ") + format!("{}", section.position).as_str()).right_aligned())
                .render(section_areas[index], frame.buffer_mut());
            MarkdownRenderer::render_markdown(section.content.as_str(), &inner_area, frame.buffer_mut(), theme);
        }
    }
}
impl Interactable for GlyphEditOrderView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if is_cycle_forward_hover_key(key) {
                    self.cycle_section_hover(1);
                }
                if is_cycle_backward_hover_key(key) {
                    self.cycle_section_hover(-1);
                }
                if let KeyCode::PageUp = key.code {
                    self.state.scroll_offset = self.state.scroll_offset.saturating_sub(1);
                }
                if let KeyCode::PageDown = key.code {
                    self.state.scroll_offset = self.state.scroll_offset.saturating_add(1);
                }
                if let KeyCode::Enter = key.code {
                    if let Some(index) = self.state.hovered_index {
                        let state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
                        let eid = state.active_entry_id.unwrap();
                        let sections: &Vec<(i64, Section)> = state.get_sections_ref(&eid);
                        if sections.len() == 0 {
                            return Ok(Vec::new());
                        }
                        if self.state.active_sid.borrow().is_some() {
                            let editing_sid: i64 = self.state.active_sid.borrow().unwrap();
                            if let Some(selected_section) = sections.get(index) {
                                if editing_sid == selected_section.0 {
                                    *self.state.active_sid.borrow_mut() = None;
                                    return Ok(vec![GlyphCommand(RefreshEditSectionEditor)]);
                                }
                            }
                        }
                        *self.state.active_sid.borrow_mut() = Some((*sections.get(index).unwrap()).0);
                        return Ok(vec![GlyphCommand(RefreshEditSectionEditor)]);
                    }
                }
                if let KeyCode::Esc = key.code {
                    // Directly mutating parent state to lose focus
                    let parent_state = parent_state.unwrap().downcast_mut::<GlyphEditState>().unwrap();
                    *parent_state.shared_focus.borrow_mut() = false;
                    return Ok(Vec::new());
                }
                if let KeyCode::Char(c) = key.code {
                    match c {
                        '+' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            if self.state.active_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let mut section: RefMut<Section> = self.get_editing_section_mut();
                            section.position = section.position + 1;
                            drop(section);
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            state.sort_sections_by_position(&eid);
                            drop(state);

                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        '-' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            if self.state.active_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let mut section: RefMut<Section> = self.get_editing_section_mut();
                            section.position = section.position - 1;
                            drop(section);
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            state.sort_sections_by_position(&eid);
                            drop(state);
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        'x' => {
                            if self.state.active_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let sid: i64 = self.state.active_sid.borrow().as_ref().unwrap().clone();
                            self.state.active_sid.replace(None);
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            state.delete_section_db(&sid)?;
                            return Ok(Vec::new());
                        }
                        'A' => {
                            let mut local_entry_state: RefMut<LocalEntryState> = self.state.local_entry_state_mut().unwrap();
                            local_entry_state.create_section_to_active_entry_db(
                                "untitled",
                                "Blank"
                            )?;
                            return Ok(Vec::new());
                        }
                        'R' => {
                            let local_entry_state = self.state.local_entry_state_ref().unwrap();
                            if self.state.active_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let sid = self.state.active_sid.borrow().as_ref().unwrap().clone();
                            let eid = local_entry_state.active_entry_id.unwrap();
                            let active_section_name: String = local_entry_state.get_section_ref(&eid, &sid).unwrap().title.clone();
                            return Ok(
                                vec![
                                    PageCommand(
                                        PushDialog(
                                            TextInputDialog::new( "Rename Section Title", active_section_name.as_str(), Box::new(|value|{true})).on_submit(
                                                // Since it is bubbling a PushDialog command up, its parent state is actually GlyphPageState
                                                Box::new(move |parent_state, state| {
                                                    let _parent_state = parent_state.unwrap().downcast_mut::<GlyphPageState>().unwrap();
                                                    let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                                                    let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();
                                                    let new_name: &str = _state.text_input.as_str();
                                                    local_entry_state.update_section_name_db(&sid, new_name)?;
                                                    Ok(vec![])
                                                })
                                            ).into()
                                        )
                                    )
                                ]
                            );
                        }
                        _ => {
                            return Ok(Vec::new());
                        }
                    }
                }
                return Ok(Vec::new());
            }
            _ => {
                Ok(Vec::new())
            }
        }
    }
}
impl Focusable for GlyphEditOrderView {
    fn is_focused(&self) -> bool {
        false
    }
    fn set_focus(&mut self, value: bool) -> () {}
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
