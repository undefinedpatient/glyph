use crate::app::popup::ConfirmPopup;
use crate::app::widget::{Button, DirectoryList, NumberField, OptionMenu, TextEditor, TextField};
use crate::app::AppCommand::{PopPage, PushPage, PushPopup};
use crate::app::Command::{AppCommand, GlyphCommand};
use crate::app::GlyphCommand::{RefreshEditSection, RefreshLayoutEditPanel, SetEntryUnsavedState};
use crate::app::{Component, Container, Convertible};
use crate::model::{Entry, GlyphRepository, Layout, LocalEntryState, Section, SizeMode};
use crate::state::page::{CreateGlyphPageState, EntrancePageState, GlyphEditContentState, GlyphEditOrderState, GlyphEditState, GlyphLayoutEditState, GlyphLayoutOverviewState, GlyphLayoutState, GlyphMode, GlyphNavigationBarState, GlyphPageState, GlyphReadState, GlyphViewerState, OpenGlyphPageState};
use crate::state::widget::{DirectoryListState, NumberFieldState, OptionMenuState, TextEditorState, TextFieldState};
use crate::state::AppState;
use crate::utils::cycle_offset;
use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::Connection;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::PartialEq;
use std::rc::Rc;
use color_eyre::Report;
use tui_scrollview::ScrollViewState;

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
        let entry_state: Rc<RefCell<LocalEntryState>> = Rc::new(RefCell::new(LocalEntryState::new(connection)));
        Self {
            dialogs: Vec::new(),
            containers: vec![
                GlyphNavigationBar::new(entry_state.clone()).into(),
                GlyphViewer::new(entry_state.clone()).into(),
                // GlyphOldViewer::new(entry_state.clone()).into()
            ],
            components: Vec::new(),
            state: GlyphPageState {
                is_focused: false,
                is_hovered: false,
                hovered_index: None,
                entry_state
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
    pub state: GlyphNavigationBarState
}

impl GlyphNavigationBar {
    pub fn new(entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        Self {
            state: GlyphNavigationBarState {
                is_focused: false,
                line_height: 1,
                hovered_index: None,
                offset: 0,

                entry_state
            }
        }
    }
    pub fn next_entry(&mut self) -> () {
        if let Ok(state) = self.state.entry_state.try_borrow() {
            let num_entries = state.entries.len();
            if let Some(index) = self.state.hovered_index {
                self.state.hovered_index = Some(cycle_offset(index as u16, 1, num_entries as u16) as usize);
            } else {
                self.state.hovered_index = Some(0);
            }
        }
    }
    pub fn previous_entry(&mut self) -> () {
        if let Ok(state) = self.state.entry_state.try_borrow() {
            let num_entries = state.entries.len();
            if let Some(index) = self.state.hovered_index {
                self.state.hovered_index = Some(cycle_offset(index as u16, -1, num_entries as u16) as usize);
            } else {
                self.state.hovered_index = Some(0);
            }
        }
    }
    pub fn get_focused_entry_ref(&'_ mut self) -> Option<Ref<'_, Entry>> {
        if let Ok(state) = self.state.entry_state.try_borrow() {
            Some(Ref::map(state, |local_entry_state| {
                local_entry_state.get_active_entry_ref().unwrap()
            }))
        } else {
            None
        }
    }
    pub fn get_focused_entry_mut(&'_ mut self) -> Option<RefMut<'_, Entry>> {
        if let Ok(state) = self.state.entry_state.try_borrow_mut() {
            Some(RefMut::map(state, |local_entry_state| {
                local_entry_state.get_active_entry_mut().unwrap()
            }))
        } else {
            None
        }
    }

}

impl From<GlyphNavigationBar> for Box<dyn Container> {
    fn from(component: GlyphNavigationBar) -> Self {
        Box::new(component)
    }
}

/*

    Glyph Viewers

 */



// Mediator
pub struct GlyphViewer {
    pub(crate) state: GlyphViewerState,
    pub(crate) containers: [Box<dyn Container>; 3],
}
pub struct GlyphReadView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphReadState,

}



pub struct GlyphEditView {
    pub containers: Vec<Box<dyn Container>>,
    pub state: GlyphEditState,
}

pub struct GlyphEditOrderView{
    pub state: GlyphEditOrderState,
}

pub struct GlyphEditContentView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphEditContentState,
}

pub struct GlyphLayoutView{
    pub dialogs: Vec<Box<dyn Container>>,
    pub containers: Vec<Box<dyn Container>>,

    pub state: GlyphLayoutState,
}

pub struct GlyphLayoutOverview {
    pub state: GlyphLayoutOverviewState,
}

pub struct GlyphLayoutEditView {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphLayoutEditState,
}


impl GlyphViewer {
    pub fn new(entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        let shared_focus: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        Self {
            containers: [
                GlyphReadView::new(shared_focus.clone(), entry_state.clone()).into(),
                GlyphEditView::new(shared_focus.clone(), entry_state.clone()).into(),
                GlyphLayoutView::new(shared_focus.clone(), entry_state.clone()).into(),
            ],
            state: GlyphViewerState {
                is_focused: shared_focus,
                mode: GlyphMode::Read,
                entry_state
            }
        }
    }
}
impl GlyphReadView {
    pub fn new(shared_focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        Self {
            containers: vec![],
            components: vec![],
            state: GlyphReadState {
                is_focused: shared_focus,

                entry_state
            }
        }
    }
}
impl GlyphEditView {
    pub fn new(shared_focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        let editing_sid : Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(None));
        let focused_panel_index: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
        Self {
            containers: vec![
                GlyphEditOrderView::new(editing_sid.clone(), entry_state.clone(), focused_panel_index.clone()).into(),
                GlyphEditContentView::new(editing_sid.clone(), entry_state.clone(), focused_panel_index.clone()).into()
            ],
            state: GlyphEditState {
                shared_focus: shared_focus,
                focused_panel_index,
                hovered_index: None,

                editing_sid,
                entry_state
            }
        }
    }
}
impl GlyphEditOrderView{
    pub fn new(
        editing_sid: Rc<RefCell<Option<i64>>>,
        entry_state: Rc<RefCell<LocalEntryState>>,
        focused_panel_index: Rc<RefCell<usize>>,
    ) -> Self {
        Self {
            state: GlyphEditOrderState {
                focused_panel_index,
                hovered_index: None,

                editing_sid,
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

    // Return the active selected section as Mutable Reference
    pub(crate) fn get_editing_section_mut(&mut self) -> RefMut<Section> {
        let editing_sid: i64 = self.state.editing_sid.borrow().unwrap().clone();
        let entry_state: RefMut<LocalEntryState> = self.state.local_entry_state_mut().unwrap();
        let active_entry_id: i64 = entry_state.active_entry_id.unwrap();
        RefMut::map(entry_state, |state|{
            state.get_section_mut(&active_entry_id, &editing_sid).unwrap()
        })
    }

}
impl GlyphEditContentView {
    pub fn new(
        editing_sid: Rc<RefCell<Option<i64>>>,
        entry_state: Rc<RefCell<LocalEntryState>>,
        focused_panel_index: Rc<RefCell<usize>>,
    ) -> Self {
        Self {
            containers: vec![
                TextField::new("title", "")
                    .on_exit(Box::new(
                        |parent_state, state| {
                            let _parent_state: &mut GlyphEditContentState = parent_state.unwrap().downcast_mut::<GlyphEditContentState>().unwrap();
                            // When no editing section exist
                            if _parent_state.editing_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let _state: &mut TextFieldState = state.unwrap().downcast_mut::<TextFieldState>().unwrap();
                            let section: &mut Section = _parent_state.section_buffer.as_mut().unwrap();
                            section.title = _state.chars.iter().collect::<String>();

                            Ok(Vec::new())
                        } )
                    )
                    .into(),
                TextEditor::new("Editor", "")
                    .on_exit(Box::new(
                        |parent_state, state| {
                            let _parent_state: &mut GlyphEditContentState = parent_state.unwrap().downcast_mut::<GlyphEditContentState>().unwrap();
                            // When no editing section exist
                            if _parent_state.editing_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let _state: &mut TextEditorState = state.unwrap().downcast_mut::<TextEditorState>().unwrap();

                            let section: &mut Section = _parent_state.section_buffer.as_mut().unwrap();
                            let mut lines: Vec<Vec<char>> = (*_state).lines.clone();
                            let line_number = lines.len();
                            for line in &mut lines[0..line_number] {
                                line.push('\n');
                            }
                            section.content = lines.concat().iter().collect::<String>();
                            Ok(Vec::new())
                        } )
                    )
                    .into()
            ],
            components: vec![
                Button::new("Revert")
                    .on_interact(Box::new(
                        |parent_state| {
                            let _parent_state: &mut GlyphEditContentState = parent_state.unwrap().downcast_mut::<GlyphEditContentState>().unwrap();
                            // When no editing section exist
                            if _parent_state.editing_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            Ok(vec![GlyphCommand(RefreshEditSection)])
                        }
                    ))
                    .into(),
                Button::new("Apply")
                    .on_interact(Box::new(
                        |parent_state| {
                            let _parent_state: &mut GlyphEditContentState = parent_state.unwrap().downcast_mut::<GlyphEditContentState>().unwrap();
                            // When no editing section exist
                            if _parent_state.editing_sid.borrow().is_none() {
                                return Ok(Vec::new());
                            }
                            let eid = _parent_state.local_entry_state_ref().unwrap().active_entry_id.unwrap();
                            let editing_sid = _parent_state.editing_sid.borrow_mut().unwrap();
                            let section_buffer: Section = _parent_state.section_buffer.as_mut().unwrap().clone();
                            let mut state: RefMut<LocalEntryState> = _parent_state.local_entry_state_mut().unwrap();
                            let mut entry = state.get_active_entry_mut().unwrap();

                            for (sid, section) in &mut entry.sections {
                                if editing_sid == *sid {
                                    *section = section_buffer;
                                    break;
                                }
                            }
                            Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))])
                        }
                    ))
                    .into(),
            ],
            state: GlyphEditContentState {
                focused_panel_index,
                hovered_index: None,
                section_buffer: None,


                editing_sid,
                entry_state
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

    pub fn refresh_section(&mut self) -> () {
        match self.state.editing_sid.borrow().as_ref() {
            Some(sid) => {
                let state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
                let eid: i64 = state.active_entry_id.unwrap();
                let sections: &Vec<(i64, Section)> = state.get_sections_ref(&eid);
                let section: Section = state.get_section_ref(&eid, &sid).unwrap().clone();
                drop(state);
                (*self.containers[0]).as_any_mut().downcast_mut::<TextField>().unwrap().replace(section.title.clone());
                (*self.containers[1]).as_any_mut().downcast_mut::<TextEditor>().unwrap().replace(section.content.clone());
                self.state.section_buffer = Some(section);
            }
            None => {
                (*self.containers[0]).as_any_mut().downcast_mut::<TextField>().unwrap().replace(String::new());
                (*self.containers[1]).as_any_mut().downcast_mut::<TextEditor>().unwrap().replace(String::new());
                self.state.section_buffer = None;
            }
        }
    }
}

impl GlyphLayoutView {
    pub fn new(shared_focus: Rc<RefCell<bool>>, entry_state: Rc<RefCell<LocalEntryState>>) -> Self {
        let focused_panel_index = Rc::new(RefCell::new(0));
        let selected_coordinate: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(vec![]));
        Self {
            dialogs: vec![],
            containers: vec![
                GlyphLayoutOverview::new(selected_coordinate.clone(), entry_state.clone(), focused_panel_index.clone()).into(),
                GlyphLayoutEditView::new(selected_coordinate.clone(), entry_state.clone(), focused_panel_index.clone()).into()
            ],
            state: GlyphLayoutState {
                shared_focus,
                focused_panel_index,
                selected_coordinate,


                entry_state
            }


        }
    }
}

impl GlyphLayoutOverview {
    pub fn new(
        selected_coordinate: Rc<RefCell<Vec<usize>>>,
        entry_state: Rc<RefCell<LocalEntryState>>,
        focused_panel_index: Rc<RefCell<usize>>,
    ) -> Self {
        let scroll_state = RefCell::new(ScrollViewState::default());
        Self {
            state: GlyphLayoutOverviewState {
                focused_panel_index,
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

impl GlyphLayoutEditView {
    pub fn new(
        selected_coordinate: Rc<RefCell<Vec<usize>>>,
        entry_state: Rc<RefCell<LocalEntryState>>,
        focused_panel_index: Rc<RefCell<usize>>,
    ) -> Self {
        Self {
            containers: vec![
                TextField::new("Label", "").on_exit(
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
                NumberField::new("Length", 0).on_exit(
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
                            if sublayout_to_update.details.length != new_value{
                                sublayout_to_update.details.length = new_value;
                                return Ok(vec![GlyphCommand(SetEntryUnsavedState(eid, true))]);
                            }
                            Ok(Vec::new())
                        }
                    )
                ).into(),
                NumberField::new("Flex", 0).on_exit(
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
                focused_panel_index,
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
        let root_layout_size_mode: u8 = match entry.layout.details.size_mode {
            SizeMode::Flex => 0,
            SizeMode::Length => 1,
        };
        (*self.containers[0]).as_any_mut().downcast_mut::<TextField>().unwrap().replace(root_layout_label);
        (*self.components[0]).as_any_mut().downcast_mut::<OptionMenu>().unwrap().replace(root_layout_size_mode);
        (*self.containers[1]).as_any_mut().downcast_mut::<NumberField>().unwrap().replace(root_layout_length as i16);
        (*self.containers[2]).as_any_mut().downcast_mut::<NumberField>().unwrap().replace(root_layout_flex as i16);
    }
}

impl From<GlyphViewer> for Box<dyn Container> {
    fn from(container: GlyphViewer) -> Self {
        Box::new(container)
    }
}
impl From<GlyphReadView> for Box<dyn Container> {
    fn from(container: GlyphReadView) -> Self {
        Box::new(container)
    }
}
impl From<GlyphEditView> for Box<dyn Container> {
    fn from(container: GlyphEditView) -> Self {
        Box::new(container)
    }
}
impl From<GlyphEditOrderView> for Box<dyn Container> {
    fn from(container: GlyphEditOrderView) -> Self {
        Box::new(container)
    }
}
impl From<GlyphEditContentView> for Box<dyn Container> {
    fn from(container: GlyphEditContentView) -> Self {
        Box::new(container)
    }
}
impl From<GlyphLayoutView> for Box<dyn Container> {
    fn from(container: GlyphLayoutView) -> Self {
        Box::new(container)
    }
}
impl From<GlyphLayoutOverview> for Box<dyn Container> {
    fn from(container: GlyphLayoutOverview) -> Self {
        Box::new(container)
    }
}
impl From<GlyphLayoutEditView> for Box<dyn Container> {
    fn from(container: GlyphLayoutEditView) -> Self {
        Box::new(container)
    }
}
