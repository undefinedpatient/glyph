use std::any::Any;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use color_eyre::eyre::Result;
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, Widget};
use crate::app::{AppCommand, Command, Component, Container, DrawFlag, Drawable, Focusable, Interactable, PageCommand};
use crate::app::widget::text_field::TextField;
use crate::theme::Theme;
use crate::utils::cycle_offset;

pub struct SearchEntryDialogState {
    pub is_focused: bool,
    pub hovered_index: usize,
    /// (entry_name, availability)
    pub entries_name: Vec<(String, bool)>,
}
pub struct SearchEntryDialog {
    state: SearchEntryDialogState,
    text_field: TextField,
}
impl SearchEntryDialog {
    pub fn new(entries_name: Vec<String>) -> SearchEntryDialog {
        Self {
            state: SearchEntryDialogState {
                is_focused: true,
                hovered_index: 0usize,
                entries_name: entries_name.into_iter().map(|name|{(name, true)}).collect(),
            },
            text_field:TextField::new(
                "Search Entry",
                "",
                Box::new(|value|{true})
            )
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
        let dialog_area: Rect = area.centered(Constraint::Length(64), Constraint::Length(12));
        let layout: Layout = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]);
        let [text_field_area, list_area] = layout.areas(dialog_area);
        Clear.render(dialog_area, frame.buffer_mut());

        self.text_field.render(frame, text_field_area, draw_flag, theme);

        // List Area
        let list_border: Block = Block::bordered().title("Entries");
        let list_inner_area: Rect = list_border.inner(list_area);
        list_border.render(list_area, frame.buffer_mut());
        for (index, row) in list_inner_area.rows().enumerate() {
            if index >= self.state.entries_name.len() {
                break;
            }
            Line::from(self.state.entries_name[index].0.as_str()).render(row, frame.buffer_mut());
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
                        cycle_offset(self.state.hovered_index as u16, -1, self.state.entries_name.iter().filter(|(name, ava)|{*ava}).count() as u16);
                    }
                    if let KeyCode::Char('p') = key.code {
                        cycle_offset(self.state.hovered_index as u16, 1, self.state.entries_name.iter().filter(|(name, ava)|{*ava}).count() as u16);
                    }
                }
                if let KeyCode::Enter = key.code {
                    
                    return Ok(vec![]);
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