use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect, Size};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, StatefulWidget, Widget};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};
use crate::app::{Command, Component, Container};
use crate::drawer::{DrawFlag, Drawable};
use crate::event_handler::Interactable;
use crate::focus_handler::Focusable;
use crate::markdown_renderer::MarkdownRenderer;
use crate::model::layout::{BorderMode, LayoutOrientation, SizeMode};
use crate::model::section::Section;
use crate::services::LocalEntryState;
use crate::theme::Theme;

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
pub struct GReadView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphReadState,
}
impl From<GReadView> for Box<dyn Container> {
    fn from(container: GReadView) -> Self {
        Box::new(container)
    }
}
impl GReadView {
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
impl Drawable for GReadView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
            Evaluate Page Layout via the root layout
         */

        let entry_state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
        let eid: i64 = entry_state.active_entry_id.unwrap();
        let layout = &entry_state.get_entry_ref(&eid).unwrap().layout;
        match layout.details.size_mode {
            SizeMode::Flex => {
                let areas: Vec<(u16, Rect, BorderMode)> = evaluate_read_areas(self, area, layout, 0,0);
                let ref_sections: &Vec<(i64, Section)> = &entry_state.get_sections_ref(&eid);
                for (sid, section) in ref_sections {
                    if let Some((position, area, border_mode)) = areas.iter().find(
                        |(position, area, border_mode)|{
                            *position as i64 == section.position
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
                        };
                        let inner_area: Rect = block.inner(*area);
                        block.render(*area, frame.buffer_mut());
                        MarkdownRenderer::render_markdown(section.content.as_str(), &inner_area, frame.buffer_mut(), theme);
                    }
                }
            }
            SizeMode::Length => {
                let height = layout.details.length;
                let mut scroll_view: ScrollView = ScrollView::new(Size{
                    width: area.width,
                    height: height
                }).scrollbars_visibility(ScrollbarVisibility::Never);
                let background: Block = Block::new().bg(theme.background());
                background.render(scroll_view.area(), scroll_view.buf_mut());
                let areas: Vec<(u16, Rect, BorderMode)> = evaluate_read_areas(self, scroll_view.area(), layout, 0,0);
                let ref_sections: &Vec<(i64, Section)> = &entry_state.get_sections_ref(&eid);
                for (sid, section) in ref_sections {
                    if let Some((position, area, border_mode)) = areas.iter().find(
                        |(position, area, border_mode)|{
                            *position as i64 == section.position
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
                        };
                        let inner_area: Rect = block.inner(*area);
                        block.render(*area, scroll_view.buf_mut());
                        MarkdownRenderer::render_markdown(section.content.as_str(), &inner_area, scroll_view.buf_mut(), theme);
                    }
                }
                scroll_view.render(area, frame.buffer_mut(), &mut *self.state.scroll_state.borrow_mut());
            }
        }

    }
}
fn evaluate_read_areas(me: &GReadView, area: Rect, layout: &crate::model::layout::Layout, depth: u16, at: usize) -> Vec<(u16, Rect, BorderMode)> {
    let mut target_section_text: String = "None".to_string();
    if let Some(position_target) = layout.section_index {
        target_section_text = position_target.to_string();
    }

    let mut recursive_area: Rect = area;
    // Process the child
    let constraints: Vec<Constraint> = layout.sub_layouts.iter().enumerate().map(
        |(index, sub)| {
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

    let mut areas: Vec<(u16, Rect, BorderMode)> = vec![];
    if let Some(section_index) = layout.section_index {
        if layout.sub_layouts.is_empty() {
            areas.push((section_index, area, layout.details.border_mode.clone()));
        }
    }
    for (i, sub_layout) in layout.sub_layouts.iter().enumerate() {
        areas = [areas,evaluate_read_areas(
            me,
            sub_areas[i],
            sub_layout,
            depth + 1,
            i,
        )].concat()
    }
    areas
}

impl Interactable for GReadView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
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
                Ok(Vec::new())
            }
            _ => {
                Ok(Vec::new())
            }
        }

    }
}
impl Focusable for GReadView {
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
