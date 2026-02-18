use crate::app::widget::button::Button;
use crate::app::widget::number_field::{NumberField, NumberFieldState};
use crate::app::widget::option_menu::{OptionMenu, OptionMenuState};
use crate::app::widget::text_field::{TextField, TextFieldState};
use crate::app::Command::GlyphCommand;
use crate::app::GlyphCommand::{RefreshLayoutEditPanel, SetEntryUnsavedState};
use crate::app::{get_draw_flag, is_cycle_backward_hover_key, is_cycle_forward_hover_key, Command, Component, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::block;
use crate::models::entry::Entry;
use crate::models::layout::{BorderMode, Layout, LayoutOrientation, SizeMode};
use crate::services::LocalEntryState;
use crate::theme::Theme;
use crate::utils::cycle_offset;
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect, Size};
use ratatui::prelude::Stylize;
use ratatui::prelude::{Line, Style};
use ratatui::widgets::{Block, BorderType, Padding, StatefulWidget, Widget};
use ratatui::Frame;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

pub struct GlyphLayoutState {
    pub shared_focus: Rc<RefCell<bool>>, // Shared state across all layout view
    pub is_editing: bool, // It is either Ordering or Editing

    // Shared Data
    pub selected_coordinate: Rc<RefCell<Vec<usize>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
impl GlyphLayoutState{
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
pub struct GlyphLayoutView {
    pub dialogs: Vec<Box<dyn Container>>,
    pub containers: Vec<Box<dyn Container>>,

    pub state: GlyphLayoutState,
}
impl GlyphLayoutView {
    pub fn new(shared_focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        let selected_coordinate: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(vec![]));
        let is_editing: bool = false;
        Self {
            dialogs: vec![],
            containers: vec![
                GlyphLayoutOverview::new(selected_coordinate.clone(), entry_state.clone()).into(),
                GlyphLayoutEditView::new(selected_coordinate.clone(), entry_state.clone()).into()
            ],
            state: GlyphLayoutState {
                shared_focus,
                selected_coordinate,
                is_editing,
                entry_state
            }


        }
    }
}
impl From<GlyphLayoutView> for Box<dyn Container> {
    fn from(container: GlyphLayoutView) -> Self {
        Box::new(container)
    }
}

impl Drawable for GlyphLayoutView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let edit_areas = ratatui::layout::Layout::horizontal([Constraint::Percentage(100), Constraint::Length(1), Constraint::Length(24)]).split(area);
        self.containers[0].render(frame, edit_areas[0],
                                  if !self.is_focused() {
                                      DrawFlag::DEFAULT
                                  }
                                  else if !self.state.is_editing {
                                      DrawFlag::FOCUSED
                                  } else {
                                      DrawFlag::DEFAULT
                                  },
                                  theme
        );
        self.containers[1].render(frame, edit_areas[2],
                                  if !self.is_focused() {
                                      DrawFlag::DEFAULT
                                  }
                                  else if self.state.is_editing {
                                      DrawFlag::FOCUSED
                                  } else {
                                      DrawFlag::DEFAULT
                                  },
                                  theme
                                  ,
        );

    }

}
impl Interactable for GlyphLayoutView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        if self.state.is_editing {
            self.containers[1].as_mut().handle(key, Some(&mut self.state))
        } else {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        if self.state.selected_coordinate.borrow().is_empty() {
                            self.state.shared_focus.replace(false);
                        }
                    }
                }
                _ => {

                }
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
                                RefreshLayoutEditPanel => {
                                    (*self.containers[1]).as_any_mut().downcast_mut::<GlyphLayoutEditView>().unwrap().refresh_layout_edit_panel();
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

impl Focusable for GlyphLayoutView {
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












pub struct GlyphLayoutOverviewState {
    pub hovered_index: Option<usize>, // Note this is the hovered index for sub-layouts, not widgets.
    pub scroll_state: RefCell<ScrollViewState>,



    // Shared Data
    pub selected_coordinate: Rc<RefCell<Vec<usize>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,

}
impl GlyphLayoutOverviewState{
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
pub struct GlyphLayoutOverview {
    pub state: GlyphLayoutOverviewState,
}
impl GlyphLayoutOverview {
    pub fn new(
        selected_coordinate: Rc<RefCell<Vec<usize>>>,
        entry_state: Rc<RefCell<LocalEntryState>>,
    ) -> Self {
        let scroll_state = RefCell::new(ScrollViewState::default());
        Self {
            state: GlyphLayoutOverviewState {
                hovered_index: None,
                selected_coordinate,
                scroll_state,

                entry_state
            },
        }
    }
    pub(crate) fn cycle_layout_hover(&mut self, offset: i16) -> () {
        let select_coordinate: Vec<usize> = self.state.selected_coordinate.borrow().clone();
        let state = self.state.local_entry_state_ref().unwrap();
        let eid: i64 = state.active_entry_id.unwrap();
        let ref_layout: &Layout = &state.get_entry_ref(&eid).unwrap().layout;
        let len: usize = ref_layout.get_layout_at_ref(&select_coordinate).unwrap().sub_layouts.len();
        drop(state);
        if let Some(hover_index) = self.state.hovered_index{
            self.state.hovered_index = Some(cycle_offset(hover_index as u16, offset, len as u16) as usize);
        } else {
            if len > 0 {
                self.state.hovered_index = Some(0);
            }
        }
    }
}

impl From<GlyphLayoutOverview> for Box<dyn Container> {
    fn from(container: GlyphLayoutOverview) -> Self {
        Box::new(container)
    }
}
impl Drawable for GlyphLayoutOverview {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {

        /*
            Evaluate Page Layout via the root layout
         */

        let entry_state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
        let eid: i64 = entry_state.active_entry_id.unwrap();
        let layout = &entry_state.get_entry_ref(&eid).unwrap().layout;
        match layout.details.size_mode {
            SizeMode::Flex => {
                evaluate_layout(self, area, frame.buffer_mut(), layout, 0, Vec::new(), theme);
            }
            SizeMode::Length => {
                let height = layout.details.length;
                let mut scroll_view = ScrollView::new(Size{
                    width: area.width,
                    height: height
                }).scrollbars_visibility(ScrollbarVisibility::Never);
                evaluate_layout(self, scroll_view.area(), scroll_view.buf_mut(), layout, 0, Vec::new(), theme);
                scroll_view.render(area, frame.buffer_mut(), &mut *self.state.scroll_state.borrow_mut());
            }
        }

    }
}

impl Interactable for GlyphLayoutOverview {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    return if !self.state.selected_coordinate.borrow_mut().is_empty() {
                        let index = self.state.selected_coordinate.borrow_mut().pop();
                        self.state.hovered_index = index;
                        Ok(vec![GlyphCommand(RefreshLayoutEditPanel)])
                    } else {
                        let parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutState>().unwrap();
                        *parent_state.shared_focus.borrow_mut() = false;
                        Ok(Vec::new())
                    }
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
                // By putting Arrow key before cycle key, ignore the arrow key cycling
                if is_cycle_forward_hover_key(key) {
                    self.cycle_layout_hover(1);
                }
                if is_cycle_backward_hover_key(key) {
                    self.cycle_layout_hover(-1);
                }
                if let KeyCode::Enter = key.code {
                    if let Some(hovered_index) = self.state.hovered_index{
                        self.state.selected_coordinate.borrow_mut().push(hovered_index);
                        self.state.hovered_index = None;
                        return Ok(vec![GlyphCommand(RefreshLayoutEditPanel)]);
                    }

                }
                if let KeyCode::Char(c) = key.code {
                    match c {
                        'e' => {
                            let parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutState>().unwrap();
                            parent_state.is_editing = true;
                            return Ok(Vec::new());
                        }
                        'A' => {
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let entry = state.get_active_entry_ref().unwrap();
                            let eid = state.active_entry_id.unwrap();
                            let layout: &mut Layout = &mut state.get_entry_mut(&eid).unwrap().layout;
                            layout.insert_sublayout_under(
                                Layout::new(""),
                                &target_coor,
                            );
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        'x' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            if self.state.selected_coordinate.borrow().is_empty() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            self.state.hovered_index = None;
                            self.state.selected_coordinate.borrow_mut().pop();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            // Get Active eid
                            let eid = state.active_entry_id.unwrap();
                            // Update
                            let layout: &mut Layout = &mut state.get_entry_mut(&eid).unwrap().layout;
                            layout.remove_sublayout(&target_coor)?;
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        '+' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            // Get Active eid
                            let entry: &mut Entry = state.get_active_entry_mut().unwrap();
                            // Update
                            let layout: &mut Layout = &mut entry.layout;
                            let sublayout: &mut Layout = layout.get_layout_at_mut(&target_coor).unwrap();
                            if sublayout.section_index.is_none() {
                                sublayout.section_index = Some(0);
                            } else {
                                sublayout.section_index = Some(sublayout.section_index.unwrap() + 1);
                            }
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        '-' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            let eid: i64 = state.active_entry_id.unwrap();
                            // Get Active eid
                            let entry: &mut Entry = state.get_active_entry_mut().unwrap();
                            // Update
                            let layout: &mut Layout = &mut entry.layout;
                            let sublayout: &mut Layout = layout.get_layout_at_mut(&target_coor).unwrap();
                            if let Some(index) = sublayout.section_index {
                                if index == 0 {
                                    sublayout.section_index = None;
                                } else {
                                    sublayout.section_index = Some(index - 1);
                                }
                            }
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                        }
                        // Transpose the alignment
                        't' => {
                            if self.state.entry_state.try_borrow_mut()?.active_entry_id.is_none() {
                                return Ok(Vec::new());
                            }
                            // Get the target coord copy
                            let target_coor = self.state.selected_coordinate.borrow_mut().clone();
                            let mut state = self.state.local_entry_state_mut().unwrap();
                            // Get Active eid
                            let eid: i64 = state.active_entry_id.unwrap();
                            let entry: &mut Entry = state.get_active_entry_mut().unwrap();
                            // Update
                            let layout: &mut Layout = &mut entry.layout;
                            let mut sublayout: &mut Layout = layout.get_layout_at_mut(&target_coor).unwrap();
                            match sublayout.details.orientation {
                                LayoutOrientation::Horizontal => {
                                    sublayout.details.orientation = LayoutOrientation::Vertical;
                                }
                                LayoutOrientation::Vertical => {
                                    sublayout.details.orientation = LayoutOrientation::Horizontal;

                                }
                            }
                            return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);

                        }
                        _ => {}
                    }
                }
                Ok(Vec::new())
            }
            _ => {
                Ok(Vec::new())
            }
        }
    }
}



impl Focusable for GlyphLayoutOverview {
    fn is_focused(&self) -> bool {
        false
    }
    fn set_focus(&mut self, value: bool) -> () {
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





pub struct GlyphLayoutEditState {
    pub hovered_index: Option<usize>,

    // Shared Data
    pub selected_coordinate: Rc<RefCell<Vec<usize>>>,
    pub entry_state: Rc<RefCell<LocalEntryState>>,
}
impl GlyphLayoutEditState{
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
pub struct GlyphLayoutEditView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphLayoutEditState,
}
impl GlyphLayoutEditView {
    pub fn new(
        selected_coordinate: Rc<RefCell<Vec<usize>>>,
        entry_state: Rc<RefCell<LocalEntryState>>,
    ) -> Self {
        Self {
            containers: vec![
                TextField::new("Label", "", Box::new(|value|{true})).on_exit(
                    Box::new(
                        |parent_state, state| {
                            let _parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutEditState>().unwrap();
                            let _state = state.unwrap().downcast_mut::<TextFieldState>().unwrap();

                            let coor: Vec<usize> = _parent_state.selected_coordinate.borrow().clone();
                            let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                            let eid: i64 = local_entry_state.active_entry_id.unwrap();
                            let entry: &mut Entry = local_entry_state.get_entry_mut(&eid).unwrap();
                            let sublayout_to_update = entry.layout.get_layout_at_mut(&coor).unwrap();
                            let new_label: String = _state.chars.iter().collect();
                            if sublayout_to_update.label != new_label {
                                sublayout_to_update.label = new_label;
                                return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                            }
                            Ok(Vec::new())
                        }
                    )
                ).into(),
                NumberField::new("Length", 0, Box::new(|value|{
                    let parse = value.parse::<i64>();
                    if let Ok(value) = parse {
                        value.is_positive()
                    } else {
                        false
                    }
                })).on_exit(
                    Box::new(
                        |parent_state, state| {
                            let _parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutEditState>().unwrap();
                            let _state = state.unwrap().downcast_mut::<NumberFieldState>().unwrap();

                            let coor: Vec<usize> = _parent_state.selected_coordinate.borrow().clone();
                            let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                            let eid: i64 = local_entry_state.active_entry_id.unwrap();
                            let entry: &mut Entry = local_entry_state.get_entry_mut(&eid).unwrap();
                            let sublayout_to_update = entry.layout.get_layout_at_mut(&coor).unwrap();
                            let new_value: u16 = _state.chars.iter().collect::<String>().parse().unwrap_or(42);
                            if sublayout_to_update.details.length != new_value{
                                sublayout_to_update.details.length = new_value;
                                return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                            }
                            Ok(Vec::new())
                        }
                    )
                ).into(),
                NumberField::new("Flex", 0, Box::new(|value| {
                    let parse = value.parse::<i64>();
                    if let Ok(value) = parse {
                        value.is_positive()
                    } else {
                        false
                    }
                }
                )).on_exit(
                    Box::new(
                        |parent_state, state| {
                            let _parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutEditState>().unwrap();
                            let _state = state.unwrap().downcast_mut::<NumberFieldState>().unwrap();

                            let coor: Vec<usize> = _parent_state.selected_coordinate.borrow().clone();
                            let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                            let eid: i64 = local_entry_state.active_entry_id.unwrap();
                            let entry: &mut Entry = local_entry_state.get_entry_mut(&eid).unwrap();
                            let sublayout_to_update = entry.layout.get_layout_at_mut(&coor).unwrap();
                            let new_value: u16 = _state.chars.iter().collect::<String>().parse().unwrap();
                            if sublayout_to_update.details.flex != new_value{
                                sublayout_to_update.details.flex = new_value;
                                return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                            }
                            Ok(Vec::new())
                        }
                    )
                ).into()
            ],
            components: vec![
                OptionMenu::new(vec![
                    ("Flex".to_string(), 0),
                    ("Length".to_string(), 1)
                ], 0).on_interact(
                    Box::new(
                        |parent_state, state| {
                            let _parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutEditState>().unwrap();
                            let _state = state.unwrap().downcast_mut::<OptionMenuState>().unwrap();

                            let coor: Vec<usize> = _parent_state.selected_coordinate.borrow().clone();
                            let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                            let eid: i64 = local_entry_state.active_entry_id.unwrap();
                            let entry: &mut Entry = local_entry_state.get_entry_mut(&eid).unwrap();
                            let sublayout_to_update = entry.layout.get_layout_at_mut(&coor).unwrap();

                            let selected_item_index: u8 = _state.current_index;
                            let selected_item_value: u8 = _state.options[selected_item_index as usize].1;
                            let parsed_selected_item = match selected_item_value {
                                0 => {
                                    SizeMode::Flex
                                }
                                1 => {
                                    SizeMode::Length
                                }
                                _ => {
                                    return Err(Report::msg("Impossible to have another value for size mode."))
                                }
                            };
                            if sublayout_to_update.details.size_mode != parsed_selected_item {
                                sublayout_to_update.details.size_mode = parsed_selected_item;
                                return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                            }
                            Ok(Vec::new())
                        }
                    )
                ).into(),
                OptionMenu::new(vec![
                    ("Border: None".to_string(), 0),
                    ("Border: Plain".to_string(), 1),
                    ("Border: Dashed".to_string(), 2),
                    ("Border: Rounded".to_string(), 3),
                ], 0).on_interact(
                    Box::new(
                        |parent_state, state| {
                            let _parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutEditState>().unwrap();
                            let _state = state.unwrap().downcast_mut::<OptionMenuState>().unwrap();

                            let coor: Vec<usize> = _parent_state.selected_coordinate.borrow().clone();
                            let mut local_entry_state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                            let eid: i64 = local_entry_state.active_entry_id.unwrap();
                            let entry: &mut Entry = local_entry_state.get_entry_mut(&eid).unwrap();
                            let sublayout_to_update: &mut Layout = entry.layout.get_layout_at_mut(&coor).unwrap();

                            let new_item_index: u8 = _state.current_index;
                            let new_item_value: u8 = _state.options[new_item_index as usize].1;
                            let parsed_item: BorderMode = match new_item_value {
                                0 => {
                                    BorderMode::None
                                }
                                1 => {
                                    BorderMode::Plain
                                }
                                2 => {
                                    BorderMode::Dashed
                                }
                                3 => {
                                    BorderMode::Rounded
                                }
                                _ => {
                                    return Err(Report::msg("Impossible to have another value for border."))
                                }
                            };
                            if sublayout_to_update.details.border_mode != parsed_item {
                                sublayout_to_update.details.border_mode = parsed_item;
                                return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                            }
                            Ok(Vec::new())
                        }
                    )
                ).into(),
                Button::new("Revert")
                    .on_interact(Box::new(
                        |parent_state| {
                            let _parent_state: &mut GlyphLayoutEditState= parent_state.unwrap().downcast_mut::<GlyphLayoutEditState>().unwrap();
                            Ok(vec![GlyphCommand(RefreshLayoutEditPanel)])
                        }
                    ))
                    .into(),
            ],

            state: GlyphLayoutEditState{
                hovered_index: None,
                selected_coordinate,

                entry_state
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

    pub fn refresh_layout_edit_panel(&mut self) -> () {
        let coor: Vec<usize> = self.state.selected_coordinate.borrow().clone();
        let state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
        let eid = state.active_entry_id.unwrap();
        // let length: u16 = layout.
        let entry: &Entry = state.get_active_entry_ref().unwrap();
        let sub_layout: &Layout = entry.layout.get_layout_at_ref(&coor).unwrap();
        let root_layout_label: String = sub_layout.label.clone();
        let root_layout_length: u16  = sub_layout.details.length;
        let root_layout_flex: u16  = sub_layout.details.flex;
        let root_layout_size_mode: u8 = match sub_layout.details.size_mode {
            SizeMode::Flex => 0,
            SizeMode::Length => 1,
        };
        let root_layout_border_mode: u8 = match sub_layout.details.border_mode {
            BorderMode::None => 0,
            BorderMode::Plain => 1,
            BorderMode::Dashed => 2,
            BorderMode::Rounded => 3,
        };
        (*self.containers[0]).as_any_mut().downcast_mut::<TextField>().unwrap().replace(root_layout_label);
        (*self.components[0]).as_any_mut().downcast_mut::<OptionMenu>().unwrap().replace(root_layout_size_mode);
        (*self.components[1]).as_any_mut().downcast_mut::<OptionMenu>().unwrap().replace(root_layout_border_mode);
        (*self.containers[1]).as_any_mut().downcast_mut::<NumberField>().unwrap().replace(root_layout_length as i16);
        (*self.containers[2]).as_any_mut().downcast_mut::<NumberField>().unwrap().replace(root_layout_flex as i16);
    }
}
impl From<GlyphLayoutEditView> for Box<dyn Container> {
    fn from(container: GlyphLayoutEditView) -> Self {
        Box::new(container)
    }
}

impl Drawable for GlyphLayoutEditView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("Setting", draw_flag, theme);
        let inner_area = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());

        let chunks = ratatui::layout::Layout::vertical([Constraint::Percentage(100), Constraint::Length(1)]).split(inner_area);
        let field_areas = ratatui::layout::Layout::vertical([
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3)
        ]).split(chunks[0]);

        // Label Field
        self.containers[0].render(frame, field_areas[0],
                                  get_draw_flag(self.state.hovered_index, 0, Some(self.containers[0].is_focused())),
                                  theme
        );
        // Size Mode Field
        self.components[0].render(frame,field_areas[1],
                                  get_draw_flag(self.state.hovered_index, 1, None),
                                  theme
        );
        // Border Mode Field
        self.components[1].render(frame,field_areas[2],
                                  get_draw_flag(self.state.hovered_index, 2, None),
                                  theme
        );
        // Length Field
        self.containers[1].render(frame, field_areas[3],
                                  get_draw_flag(self.state.hovered_index, 3, Some(self.containers[1].is_focused())),
                                  theme
        );
        // Flex Field
        self.containers[2].render(frame, field_areas[4],
                                  get_draw_flag(self.state.hovered_index, 4, Some(self.containers[2].is_focused())),
                                  theme
        );


        // Revert Button
        self.components[2].render(frame, chunks[1],
                                  get_draw_flag(self.state.hovered_index, 5, None),
                                  theme
        );

    }
}
fn evaluate_layout(me: &GlyphLayoutOverview, area: Rect, buffer: &mut Buffer, layout: &crate::models::layout::Layout, depth: u16, at: Vec<usize>, theme: &dyn Theme) -> Vec<(u16, Rect)>{
    let mut target_section_text: String = "None".to_string();
    if let Some(position_target) = layout.section_index {
        target_section_text = position_target.to_string();
    }

    let focused_coordinate = &me.state.selected_coordinate.borrow().clone();


    // Generate Border to render visualization.
    let mut block: Block = Block::bordered().title(layout.label.as_str()).padding(Padding {left: 1, right: 1, top: 1, bottom: 1}).style(theme.on_surface()).bg(theme.background());
    if at == *focused_coordinate {
        block = block.border_type(BorderType::Thick).title_style(Style::new().bold());

    } else if let Some(hovered_index) = me.state.hovered_index {
        let mut hover_coordinate = me.state.selected_coordinate.borrow().clone();
        hover_coordinate.push(hovered_index);
        if hover_coordinate == at {
            block = block.border_type(BorderType::Double);
        }
    }

    // Determine the section index render.
    if !layout.sub_layouts.is_empty() {
        block = block.title_bottom(Line::from("(Disabled)").dim());
    } else {
        block = block.title_bottom(Line::from(target_section_text));
    }

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

    let recursive_area: Rect = block.inner(area);
    block.render(area, buffer);


    let sub_areas =
        match layout.details.orientation {
            LayoutOrientation::Vertical => {
                ratatui::layout::Layout::vertical(constraints).split(recursive_area)
            }
            LayoutOrientation::Horizontal => {
                ratatui::layout::Layout::horizontal(constraints).split(recursive_area)
            }
        };

    let mut areas: Vec<(u16, Rect)> = vec![];
    if let Some(section_index) = layout.section_index {
        areas.push((section_index, recursive_area));
    }
    for (i, sub_layout) in layout.sub_layouts.iter().enumerate() {
        let mut sub_at = at.clone();
        sub_at.push(i);
        areas = [areas, evaluate_layout(
            me,
            sub_areas[i],
            buffer,
            sub_layout,
            depth + 1,
            sub_at,
            theme
        )].concat()
    }
    areas
}
impl Interactable for GlyphLayoutEditView {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        let parent_state = parent_state.unwrap().downcast_mut::<GlyphLayoutState>().unwrap();
                        parent_state.is_editing = false;
                        return Ok(Vec::new());
                    }
                    if is_cycle_forward_hover_key(key) {
                        self.cycle_hover(1);
                        return Ok(Vec::new());
                    }
                    if is_cycle_backward_hover_key(key) {
                        self.cycle_hover(-1);
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            match index {
                                0 => { // Label Field
                                    self.containers[0].set_focus(true);
                                }
                                1 => { // Size Field
                                    return self.components[0].handle(key, Some(&mut self.state));
                                }
                                2 => { // Size Field
                                    return self.components[1].handle(key, Some(&mut self.state));
                                }
                                3 => { // Length Field
                                    self.containers[1].set_focus(true);
                                }
                                4 => { // Flex Field
                                    self.containers[2].set_focus(true);
                                }
                                5 => {
                                    return self.components[2].handle(key, Some(&mut self.state));
                                }
                                _ => {
                                }
                            }
                        }
                    }
                    return Ok(Vec::new());
                }
                _ => {

                    return Ok(Vec::new());
                }
            }
        } else {
            let index: usize = self.focused_child_index().unwrap();
            let mut result =
                self.containers[index].handle(key, Some(&mut self.state));
            result
        }
    }
}
impl Focusable for GlyphLayoutEditView {
    fn is_focused(&self) -> bool {
        false
    }
    fn set_focus(&mut self, value: bool) -> () {
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(&**container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        for (index, container) in self.containers.iter().enumerate() {
            if container.is_focused() {
                return Some(index);
            }
        }
        None
    }
}
