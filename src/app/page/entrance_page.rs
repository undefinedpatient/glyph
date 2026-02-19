use crate::app::page::glyph_create_page::GlyphCreatePage;
use crate::app::page::glyph_open_page::GlyphOpenPage;
use crate::app::popup::confirm_popup::ConfirmPopup;
use crate::app::widget::button::Button;
use crate::app::AppCommand::{PushPage, PushPopup};
use crate::app::Command::AppCommand;
use crate::app::{is_cycle_backward_hover_key, is_cycle_forward_hover_key, AppState, Command, Component, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::block;
use crate::theme::Theme;
use crate::utils::cycle_offset;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::prelude::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::{BorderType, Widget};
use ratatui::Frame;
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use tui_big_text::{BigText, PixelSize};

pub struct EntrancePageState {
    pub is_focused: bool,
    pub is_hovered: bool,
    pub hovered_index: Option<usize>,
}
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
                                    GlyphCreatePage::new().into()
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
                                    GlyphOpenPage::new().into()
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
impl Drawable for EntrancePage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let block: Block = block!("Glyph", draw_flag, theme);

        /*

        */
        let area_inner: Rect = block.inner(area);
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let areas: Rc<[Rect]> = Layout::vertical([Constraint::Length(8), Constraint::Length(3), Constraint::Length(3)])
            .flex(Flex::Center)
            .split(rect);
        let button_rects = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
            .split(areas[2]);
        /*
          Title
        */
        let title = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(theme.on_surface())
            .lines(vec![Line::from("Glyph")])
            .alignment(HorizontalAlignment::Center)
            .build();
        let version: Line = Line::from("v0.1.0").centered();
        // Render Section
        block.render(area, frame.buffer_mut());
        title.render(areas[0], frame.buffer_mut());
        version.render(areas[1], frame.buffer_mut());
        for (i, button_interactable) in (&self.components).iter().enumerate() {
            if let Some(ci) = self.state.hovered_index {
                if i == ci {
                    button_interactable.render(frame, button_rects[i], DrawFlag::HIGHLIGHTING, theme);
                } else {
                    button_interactable.render(frame, button_rects[i], DrawFlag::DEFAULT, theme);
                }
            }
            button_interactable.render(frame, button_rects[i], DrawFlag::DEFAULT, theme);
        }
    }
}
impl Interactable for EntrancePage {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if is_cycle_forward_hover_key(key) {
                    self.cycle_hover(1);
                }
                if is_cycle_backward_hover_key(key) {
                    self.cycle_hover(-1);
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![
                        AppCommand(PushPopup(
                            ConfirmPopup::new("Exit Glyph?").on_confirm(
                                Box::new(
                                    |app_state| {
                                        let _app_state = app_state.unwrap().downcast_mut::<AppState>().unwrap();
                                        _app_state.should_quit = true;
                                        Ok(Vec::new())
                                    }
                                )
                            ).into()
                        ))
                    ]);
                }
                if let KeyCode::Enter = key.code {
                    if let Some(index) = self.state.hovered_index {
                        return self.components[index].as_mut().handle(key, None);
                    }
                }
            }
            _ => {}
        }
        Ok(Vec::new())
    }
    fn keymap(&self) -> Vec<(&str, &str)>{
        [
            ("j/k/up/down/tab/backtab","Navigate"),
            ("Enter","Interact"),
        ].into()
    }
}
impl Focusable for EntrancePage {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
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
