use crate::app::{Command, Component, Container, Convertible, DrawFlag, Drawable, Focusable, Interactable};
use crate::models::layout::{BorderMode, LayoutOrientation, SizeMode};
use crate::models::section::Section;
use crate::services::LocalEntryState;
use crate::theme::{Iceberg, Theme};
use crate::utils::markdown_renderer::MarkdownRenderer;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Position, Rect, Size};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Padding, StatefulWidget, Widget};
use ratatui::Frame;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use ratatui::buffer::Buffer;
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};
use crate::app::Command::PageCommand;
use crate::app::dialog::text_input_dialog::{TextInputDialog, TextInputDialogState};
use crate::app::page::glyph_page::{GlyphPage, GlyphPageState};
use crate::app::PageCommand::PushDialog;
use crate::app::widget::directory_list::DirectoryList;

pub struct GlyphReadState {
    pub is_focused: Rc<RefCell<bool>>, // Shared state across all view
    pub scroll_state: RefCell<ScrollViewState>,

    // Shared Data
    pub entry_state: Rc<RefCell<LocalEntryState>>,
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
pub struct GlyphReadView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphReadState,
}
impl From<GlyphReadView> for Box<dyn Container> {
    fn from(container: GlyphReadView) -> Self {
        Box::new(container)
    }
}
impl GlyphReadView {
    pub fn new(shared_focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        let scroll_state = RefCell::new(ScrollViewState::default());
        Self {
            containers: vec![],
            components: vec![],
            state: GlyphReadState {
                is_focused: shared_focus,
                scroll_state,
                entry_state
            }
        }
    }
}
impl Drawable for GlyphReadView {
    fn render(&self, frame: &mut Frame, area: Rect, _draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
            Evaluate Page Layout via the root layout
         */

        let entry_state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
        let eid: i64 = entry_state.active_entry_id.unwrap();
        let layout = &entry_state.get_entry_ref(&eid).unwrap().layout;
        match layout.details.size_mode {
            SizeMode::Flex => {
                let areas: Vec<(u16, Rect, BorderMode, u16)> = evaluate_read_areas(area, layout, 0);
                let ref_sections: &Vec<(i64, Section)> = &entry_state.get_sections_ref(&eid);
                for (_sid, _section) in ref_sections {
                    if let Some((_position, area, border_mode, padding)) = areas.iter().find(
                        |(_position, _area, _border_mode, _padding)|{
                            *_position as i64 == _section.position
                        }
                    ) {
                        let block = match border_mode {
                            BorderMode::None => {
                                Block::new().title(_section.title.clone().bold())
                            }
                            BorderMode::Plain => {
                                Block::bordered().title(_section.title.clone().bold())
                            }
                            BorderMode::Dashed => {
                                Block::bordered().border_type(BorderType::LightDoubleDashed).title(_section.title.clone().bold())
                            }
                            BorderMode::Rounded => {
                                Block::bordered().border_type(BorderType::Rounded).title(_section.title.clone().bold())
                            }
                        }.padding(Padding::uniform(*padding));
                        let inner_area: Rect = block.inner(*area);
                        block.render(*area, frame.buffer_mut());
                        MarkdownRenderer::create(inner_area.clone(), theme).render(_section.content.as_str(), frame.buffer_mut());
                    }
                }
            }
            SizeMode::Length => {
                let height = layout.details.length;
                let mut scroll_view: ScrollView = ScrollView::new(Size{
                    width: area.width,
                    height
                }).scrollbars_visibility(ScrollbarVisibility::Never);
                let background: Block = Block::new().bg(theme.background());
                background.render(scroll_view.area(), scroll_view.buf_mut());
                let areas: Vec<(u16, Rect, BorderMode, u16)> = evaluate_read_areas(scroll_view.area(), layout, 0);
                let ref_sections: &Vec<(i64, Section)> = &entry_state.get_sections_ref(&eid);
                for (_sid, section) in ref_sections {
                    if let Some((_position, area, border_mode, padding)) = areas.iter().find(
                        |(_position, _area, _border_mode, _padding)|{
                            *_position as i64 == section.position
                        }
                    ) {
                        let block = match border_mode {
                            BorderMode::None => {
                                Block::new().title(section.title.clone().bold())
                            }
                            BorderMode::Plain => {
                                Block::bordered().title(section.title.clone().bold())
                            }
                            BorderMode::Dashed => {
                                Block::bordered().border_type(BorderType::LightDoubleDashed).title(section.title.clone().bold())
                            }
                            BorderMode::Rounded => {
                                Block::bordered().border_type(BorderType::Rounded).title(section.title.clone().bold())
                            }
                        }.padding(Padding::uniform(*padding));
                        let inner_area: Rect = block.inner(*area);
                        block.render(*area, scroll_view.buf_mut());
                        MarkdownRenderer::create(inner_area.clone(), theme).render(section.content.as_str(), scroll_view.buf_mut());
                    }
                }
                scroll_view.render(area, frame.buffer_mut(), &mut *self.state.scroll_state.borrow_mut());
            }
        }

    }
}
fn evaluate_read_areas(area: Rect, layout: &crate::models::layout::Layout, depth: u16) -> Vec<(u16, Rect, BorderMode, u16)> {
    let recursive_area: Rect = Block::default().inner(area);

    // Process the child
    let constraints: Vec<Constraint> = layout.sub_layouts.iter().enumerate().map(
        |(_index, sub)| {
            match sub.details.size_mode {
                SizeMode::Flex => {
                    Constraint::Fill(sub.details.flex)
                }
                SizeMode::Length => {
                    Constraint::Length(sub.details.length)
                }
            }
        }
    ).collect();
    let sub_areas =
        match layout.details.orientation {
            LayoutOrientation::Vertical => {
                Layout::vertical(constraints).split(recursive_area)
            }
            LayoutOrientation::Horizontal => {
                Layout::horizontal(constraints).split(recursive_area)
            }
        };

    let mut areas: Vec<(u16, Rect, BorderMode, u16)> = vec![];
    if let Some(section_index) = layout.section_index {
        if layout.sub_layouts.is_empty() {
            areas.push((section_index, area.inner(Margin::new(layout.details.margin, layout.details.margin)), layout.details.border_mode.clone(), layout.details.padding));
        }
    }

    for (i, sub_layout) in layout.sub_layouts.iter().enumerate() {
        areas = [areas,evaluate_read_areas(
            sub_areas[i],
            sub_layout,
            depth + 1,
        )].concat()
    }
    areas
}

impl Interactable for GlyphReadView {
    fn handle(&mut self, key: &KeyEvent, _parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    self.set_focus(false);
                    return Ok(Vec::new());
                }
                if let KeyCode::PageUp = key.code {
                    self.state.scroll_state.borrow_mut().scroll_page_up();
                    return Ok(Vec::new());
                }
                if let KeyCode::PageDown = key.code {
                    self.state.scroll_state.borrow_mut().scroll_page_down();
                    return Ok(Vec::new());
                }
                if let KeyCode::Up = key.code {
                    self.state.scroll_state.borrow_mut().scroll_up();
                    return Ok(Vec::new());
                }
                if let KeyCode::Down = key.code {
                    self.state.scroll_state.borrow_mut().scroll_down();
                    return Ok(Vec::new());
                }
                if let KeyCode::Char('P') = key.code {
                    return Ok(vec![
                        PageCommand(PushDialog(
                            TextInputDialog::new("Path (TODO: Allow user to input size)", std::env::current_dir()?.to_str().unwrap(), Box::new(|input| {!input.ends_with("/")})).on_submit(
                                Box::new(|parent_state, state|{
                                    let _parent_state = parent_state.unwrap().downcast_ref::<GlyphPageState>().unwrap();
                                    let _state = state.unwrap().downcast_ref::<TextInputDialogState>().unwrap();
                                    let mut file: File = fs::File::create(_state.text_input.clone())?;

                                    let entry_state: Ref<LocalEntryState> = _parent_state.local_entry_state_ref().unwrap();
                                    let eid: i64 = entry_state.active_entry_id.unwrap();
                                    let layout: &crate::models::layout::Layout = &entry_state.get_entry_ref(&eid).unwrap().layout;
                                    let mut buffer: Buffer = Buffer::empty(Rect::new(0,0, 96, layout.details.length));
                                    let areas: Vec<(u16, Rect, BorderMode, u16)> = evaluate_read_areas(*buffer.area(), layout, 0);
                                    let ref_sections: &Vec<(i64, Section)> = &entry_state.get_sections_ref(&eid);
                                    for (_sid, section) in ref_sections {
                                        if let Some((_position, area, border_mode, padding)) = areas.iter().find(
                                            |(_position, _area, _border_mode, _padding)|{
                                                *_position as i64 == section.position
                                            }
                                        ) {
                                            let block = match border_mode {
                                                BorderMode::None => {
                                                    Block::new().title(section.title.clone().bold())
                                                }
                                                BorderMode::Plain => {
                                                    Block::bordered().title(section.title.clone().bold())
                                                }
                                                BorderMode::Dashed => {
                                                    Block::bordered().border_type(BorderType::LightDoubleDashed).title(section.title.clone().bold())
                                                }
                                                BorderMode::Rounded => {
                                                    Block::bordered().border_type(BorderType::Rounded).title(section.title.clone().bold())
                                                }
                                            }.padding(Padding::uniform(*padding));
                                            let inner_area: Rect = block.inner(*area);
                                            block.render(*area, &mut buffer);
                                            MarkdownRenderer::create(inner_area.clone(), &mut Iceberg).render(section.content.as_str(), &mut buffer);
                                        }
                                    }
                                    let final_area = buffer.area();
                                    for y in final_area.y..final_area.y+final_area.height {
                                        let mut line_bytes = Vec::new();
                                        for x in final_area.x..final_area.x+final_area.width {
                                            let cell = Buffer::cell(&buffer, Position{x, y}).unwrap();
                                            line_bytes.extend_from_slice(cell.symbol().as_bytes());
                                        }
                                        file.write_all(&line_bytes)?;
                                        file.write_all(b"\n")?;
                                    }
                                    return Ok(vec![])
                                })
                            ).into()
                        ))
                    ]);
                }
                Ok(Vec::new())
            }
            _ => {
                Ok(Vec::new())
            }
        }

    }
}
impl Focusable for GlyphReadView {
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
