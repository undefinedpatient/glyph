use crate::app::page::glyph_page::GlyphPage;
use crate::app::widget::button::Button;
use crate::app::widget::directory_list::{DirectoryList, DirectoryListState};
use crate::app::AppCommand::{PopPage, PushPage};
use crate::app::Command::{AppCommand, PageCommand};
use crate::app::PageCommand::{PopDialog, PushDialog};
use crate::app::{get_draw_flag, is_cycle_backward_hover_key, is_cycle_forward_hover_key, Command, Component, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::block;
use crate::db::GlyphRepository;
use crate::theme::Theme;
use crate::utils::cycle_offset;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::text::Line;
use ratatui::widgets::BorderType;
use ratatui::widgets::{Block, Widget};
use ratatui::Frame;
use std::any::Any;
use std::path::PathBuf;
use crate::app::dialog::text_input_dialog::{TextInputDialog, TextInputDialogState};

pub struct GlyphCreatePageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
    pub path_to_create: PathBuf,
}
pub struct GlyphCreatePage {
    pub dialogs: Vec<Box<dyn Container>>,
    pub containers: Vec<Box<dyn Container>>,
    pub components: Vec<Box<dyn Component>>,
    pub state: GlyphCreatePageState,
}
impl GlyphCreatePage {
    pub fn new() -> Self {
        Self {
            dialogs: Vec::new(),
            containers: vec![
                DirectoryList::new("Directory", false,true)
                    .on_exit(
                        Box::new(
                            |parent_state, state| {
                                let _parent_state = parent_state.unwrap().downcast_mut::<GlyphCreatePageState>().unwrap();
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
            state: GlyphCreatePageState {
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
impl From<GlyphCreatePage> for Box<dyn Container> {
    fn from(page: GlyphCreatePage) -> Self {
        Box::new(page)
    }
}
impl Drawable for GlyphCreatePage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Outer Frame
        */
        let page_frame: Block = block!("Create Glyph", draw_flag, theme);
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
        /*
            Dialog
         */
        if !self.dialogs.is_empty() {
            for (i, dialog) in self.dialogs.iter().enumerate() {
                if i == self.dialogs.len() - 1 {
                    dialog.render(frame, area, DrawFlag::FOCUSED, theme);
                } else {
                    dialog.render(frame, area, DrawFlag::DEFAULT, theme);
                }
            }
        }
    }
}
impl Interactable for GlyphCreatePage {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        /*
            Page's Dialog
         */
        if !self.dialogs.is_empty() {
            let result = self.dialogs.last_mut().unwrap().handle(key, Some(&mut self.state));
            return if result.is_err() {
                result
            } else {
                let mut processed_commands: Vec<Command> = Vec::new();
                let mut commands = result?;
                while let Some(command) = commands.pop() {
                    match command {
                        PageCommand(page_command) => {
                            match page_command {
                                PopDialog => {
                                    self.dialogs.pop();
                                }
                                PushDialog(dialog) => {
                                    self.dialogs.push(dialog);
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
        /*
            Page
         */
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
                                    // Create Button
                                    self.dialogs.push(
                                        TextInputDialog::new( "Glyph Name", "untitled_glyph", Box::new(|value|{!value.is_empty()}))
                                            .on_submit( Box::new(|parent_state, state| {

                                                let _parent_state = parent_state.unwrap().downcast_mut::<GlyphCreatePageState>().unwrap();
                                                let _state = state.unwrap().downcast_mut::<TextInputDialogState>().unwrap();

                                                let connection = GlyphRepository::init_glyph_db(&_parent_state.path_to_create.join(_state.text_input.clone()+".glyph"));
                                                Ok(
                                                    vec![
                                                        PageCommand(PopDialog),
                                                        AppCommand(PushPage(
                                                            GlyphPage::new(connection.unwrap()).into()
                                                        )),
                                                        AppCommand(PopPage)
                                                    ]
                                                )
                                            }
                                            )
                                            ).into());
                                    return Ok(Vec::new());
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
impl Focusable for GlyphCreatePage {
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
