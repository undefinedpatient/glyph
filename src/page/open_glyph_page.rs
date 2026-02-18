use ratatui::prelude::Stylize;
use ratatui::widgets::BorderType;
use ratatui::text::Line;
use std::any::Any;
use std::path::PathBuf;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::widgets::{Block, Widget};
use crate::app::{Command, Component, Container};
use crate::app::AppCommand::{PopPage, PushPage};
use crate::app::Command::AppCommand;
use crate::block;
use crate::db::GlyphRepository;
use crate::drawer::{get_draw_flag, DrawFlag, Drawable};
use crate::event_handler::{is_cycle_backward_hover_key, is_cycle_forward_hover_key, Interactable};
use crate::focus_handler::Focusable;
use crate::page::glyph_page::GlyphPage;
use crate::theme::Theme;
use crate::utils::cycle_offset;
use crate::widget::button::Button;
use crate::widget::directory_list::{DirectoryList, DirectoryListState};

pub struct OpenGlyphPageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub path_to_open: PathBuf,
}
pub struct GOpenPage {
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: OpenGlyphPageState,
}
impl GOpenPage {
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
impl From<GOpenPage> for Box<dyn Container> {
    fn from(page: GOpenPage) -> Self {
        Box::new(page)
    }
}
impl Drawable for GOpenPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Outer Frame
        */
        let page_frame: Block = block!("Open Glyph", draw_flag, theme);
        /*
           Chucks
        */
        let inner_area: Rect = page_frame.inner(area);
        page_frame.render(area, frame.buffer_mut());
        let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Max(3)])
            .flex(Flex::Center)
            .spacing(3)
            .split(inner_area);

        let file_explorer_area = chunks[0].centered(Constraint::Max(64), Constraint::Min(42));
        let button_areas = Layout::horizontal([Constraint::Max(20), Constraint::Max(20)])
            .flex(Flex::Center)
            .split(chunks[1]);
        self.containers[0].render(
            frame,
            file_explorer_area,
            get_draw_flag(
                self.state.hovered_index,
                0,
                Some(self.containers[0].is_focused()),
            ),
            theme
        );
        self.components[0].render(
            frame,
            button_areas[0],
            get_draw_flag(self.state.hovered_index, 1, None),
            theme
        );
        self.components[1].render(
            frame,
            button_areas[1],
            get_draw_flag(self.state.hovered_index, 2, None),
            theme
        );
    }
}
impl Interactable for GOpenPage {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if is_cycle_forward_hover_key(key) {
                        self.cycle_hover(1);
                    }
                    if is_cycle_backward_hover_key(key) {
                        self.cycle_hover(-1);
                    }
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![AppCommand(PopPage)]);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            match index {
                                0 => {
                                    // Directory List
                                    self.containers[index].set_focus(true);
                                }
                                1 => {
                                    // Back Button
                                    return self.components[0].handle(key, None);
                                }
                                2 => {
                                    // Open Button
                                    return self.components[1].handle(key, Some(&mut self.state));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(Vec::new())
        } else {
            let index: usize = self.focused_child_index().unwrap();
            let mut result =
                self.containers[index].handle(key, Some(&mut self.state));
            result
        }
    }
}
impl Focusable for GOpenPage {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
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
