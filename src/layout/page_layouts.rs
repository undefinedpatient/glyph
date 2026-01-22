use ratatui::layout::{Alignment, Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::prelude::{Line, StatefulWidget, Style, Stylize, Text, Widget};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use std::rc::Rc;
use tui_big_text::{BigText, PixelSize};

use crate::app::{App, PageState, view_type::PageView};
use crate::utils::get_dir_names;

// Widget itself must not own any resources, and never outlive the AppState it references to.
pub struct EntrancePageLayout<'a>{
    ref_mut_app: &'a mut App,
}
impl<'a> EntrancePageLayout<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { ref_mut_app: app }
    }
    pub fn draw(&mut self, frame: &mut Frame) -> () {
        // Widget/Data Section
        let title = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec![
                "Glyph".magenta().into(),
            ])
            .alignment(HorizontalAlignment::Center)
            .build();
        let text_actions: Text = Text::from(vec![
            Line::from("Create (a)"),
            Line::from("Open   (o)"),
            Line::from("Quit   (q)"),
        ]).centered();
        let block: Block = Block::default().borders(Borders::ALL);
        let area_inner: Rect = block.inner(frame.area());
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let rects: Rc<[Rect]> = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(3)
        ])
            .flex(Flex::Center)
            .split(rect);
        // Render Section
        frame.render_widget(block, frame.area());
        frame.render_widget(title, rects[0]);
        frame.render_widget(text_actions, rects[1]);
    }
}

pub struct CreateGlyphLayout<'a>{
    ref_mut_app: &'a mut App
}
impl<'a> CreateGlyphLayout<'a>{
    pub fn new(app: &'a mut App) -> Self {
        CreateGlyphLayout {
            ref_mut_app: app
        }
    }
    pub fn draw(&mut self, frame: &mut Frame) -> () {
        let areas: Rc<[Rect]> = Layout::vertical(
            [
                Constraint::Fill(1),
                Constraint::Length(1)
            ]
        ).split(frame.area());

        let _frame: Block = Block::default().borders(Borders::ALL).title(" Create Glyph ");
        let inner_area: Rect = _frame.inner(areas[0]);
        
        
        frame.render_widget(_frame, areas[0]);

        let file_explorer_area = inner_area
            .centered(Constraint::Max(42), Constraint::Percentage(50));
        frame.render_widget(DirectoryWidget::new(self.ref_mut_app), file_explorer_area);
        frame.render_widget(Paragraph::new("Create (c) Back (q)").alignment(Alignment::Right), areas[1]);
    }
}

struct DirectoryWidget<'a> {
    ref_mut_app: &'a mut App
}
impl<'a> DirectoryWidget<'a> {
    fn new(app: &'a mut App) -> Self {
        DirectoryWidget {
            ref_mut_app: app
        }
    }
}
impl<'a> Widget for DirectoryWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let list_items: Vec<ListItem> = get_dir_names(self.ref_mut_app.state.get_current_path())
            .unwrap_or(Vec::new())
            .iter()
            .enumerate()
            .map(
                |(i, item)| {
                    return ListItem::new(item.clone());
                }
            )
            .collect();


        let current_path: String = self.ref_mut_app.state.get_current_path().clone().to_str().unwrap_or("Invalid Path").to_string();
        let list = List::new(list_items)
            .block(
                Block::bordered().title(current_path)
            )
            .highlight_style(Style::new().bold());
        if let Some(state) = self.ref_mut_app.h_page_states.get_mut(&PageView::CreateGlyph) {
            match state {
                PageState::CreateGlyph { list_state} => {
                    StatefulWidget::render(list, area, buf, list_state);

                }
                _ => {}
            }
        }
    }
}

