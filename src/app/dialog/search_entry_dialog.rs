use ratatui::widgets::BorderType;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use color_eyre::eyre::Result;
use color_eyre::owo_colors::OwoColorize;
use ratatui::prelude::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, Widget};
use crate::app::{AppCommand, Command, Component, Container, DrawFlag, Drawable, Focusable, Interactable, PageCommand};
use crate::app::page::glyph_page::GlyphPage;
use crate::app::widget::text_field::{TextField, TextFieldState};
use crate::block;
use crate::services::LocalEntryState;
use crate::theme::Theme;
use crate::utils::cycle_offset;

pub struct SearchEntryDialogState {
    pub is_focused: bool,
    pub hovered_index: usize,
    /// ((eid, entry_name), availability)
    pub entries_name: Vec<((i64, String), bool)>,
    pub local_entry_state: Rc<RefCell<LocalEntryState>>,
}
pub struct SearchEntryDialog {
    state: SearchEntryDialogState,
    text_field: TextField,
}
impl SearchEntryDialog {
    pub fn new(local_entry_state: Rc<RefCell<LocalEntryState>>)-> SearchEntryDialog {
        let entries: Vec<(i64, String)> = (*local_entry_state.borrow().entries).iter().map(|(id,entry)|{(*id, entry.entry_name.clone())}).collect();
        Self {
            state: SearchEntryDialogState {
                is_focused: true,
                hovered_index: 0usize,
                entries_name: entries.into_iter().map(|item|{(item, true)}).collect(),
                local_entry_state,
            },
            text_field:TextField::new(
                "",
                "",
                Box::new(|value|{true})
            ).on_update(Box::new(|parent_state, state| {
                let mut _parent_state = parent_state.unwrap().downcast_mut::<SearchEntryDialogState>().unwrap();
                let mut _state = state.unwrap().downcast_mut::<TextFieldState>().unwrap();
                let mut ava_count: usize = 0;
                for ((eid, name), ava) in _parent_state.entries_name.iter_mut() {
                    if name.contains(_state.chars.iter().collect::<String>().as_str()) {
                        *ava = true;
                        ava_count += 1;
                    } else {
                        *ava = false;
                    }
                }
                _parent_state.hovered_index = _parent_state.hovered_index.clamp(0, ava_count).saturating_sub(1);
                Ok(vec![])
            }))
        }
    }
}
impl From<SearchEntryDialog> for Box<dyn Container> {
    fn from(dialog: SearchEntryDialog) -> Self {
        Box::new(dialog)
    }
}

impl Drawable for SearchEntryDialog {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let dialog_frame = block!("Search Entry", draw_flag, theme);
        let dialog_area: Rect = area.centered(Constraint::Length(64), Constraint::Percentage(50));
        let dialog_inner_area: Rect = dialog_frame.inner(dialog_area);
        let layout: Layout = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]);
        let [text_field_area, list_area] = layout.areas(dialog_inner_area);
        Clear.render(dialog_area, frame.buffer_mut());
        dialog_frame.render(dialog_area, frame.buffer_mut());

        self.text_field.render(frame, text_field_area, draw_flag, theme);

        // List Area
        let list_border: Block = Block::bordered().title("Entries");
        let list_inner_area: Rect = list_border.inner(list_area);
        list_border.render(list_area, frame.buffer_mut());

        let rows = list_inner_area.rows().collect::<Vec<Rect>>();
        for (index, (item, ava)) in self.state.entries_name.iter().filter(|(_, ava)| {*ava}).enumerate() {
            if index >= rows.len() {
                break;
            }
            let prefix = match index == self.state.hovered_index {
                true => "> ",
                false => "  "
            };
            let mut line: Line = Line::from([prefix, (*item).1.as_str()].concat()).dim();

            if index == self.state.hovered_index {
                line = line.bold().not_dim();
            }
            line.render(rows[index], frame.buffer_mut());

        }
    }
}

impl Interactable for SearchEntryDialog {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> Result<Vec<Command>> {

        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Esc = key.code {
                    return Ok(vec![Command::PageCommand(PageCommand::PopDialog)]);
                }
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if let KeyCode::Char('n') = key.code {
                        self.state.hovered_index = cycle_offset(self.state.hovered_index as u16, 1, self.state.entries_name.iter().filter(|(name, ava)|{*ava}).count() as u16) as usize;
                    }
                    if let KeyCode::Char('p') = key.code {
                        self.state.hovered_index = cycle_offset(self.state.hovered_index as u16, -1, self.state.entries_name.iter().filter(|(name, ava)|{*ava}).count() as u16) as usize;
                    }
                    return Ok(vec![]);
                }
                if let KeyCode::Enter = key.code {
                    if self.state.entries_name.is_empty() {
                        return Ok(vec![]);
                    }
                    let cloned_id = self.state.entries_name.iter().filter_map(|((eid, name),ava)|{
                        if *ava {
                            return Some(*eid)
                        }
                        None
                    }).collect::<Vec<i64>>();
                    self.state.local_entry_state.borrow_mut().active_entry_id = Some(
                        cloned_id.get(self.state.hovered_index).unwrap().clone()
                    );

                    return Ok(vec![Command::PageCommand(PageCommand::PopDialog)]);
                }

                self.text_field.handle(key, Some(&mut self.state))?;
                Ok(vec![])
            }
            _ => {
                Ok(vec![])
            }
        }

    }
    fn keymap(&self) -> Vec<(&str, &str)> {
        [
            ("c-n","Next Item"),
            ("c-p","Previous Item"),
            ("Enter", "Open Entry")
        ].into()
    }
}
impl Focusable for SearchEntryDialog {
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