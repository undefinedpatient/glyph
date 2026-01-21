use std::rc::Rc;
use ratatui::layout::{Alignment, Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::prelude::{Line, StatefulWidget, Style, Stylize, Text, Widget};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui_big_text::{BigText, PixelSize};
use crate::app::App;
use crate::utils::get_dir_names;

// Widget itself must not own any resources, and never outlive the AppState it references to.
pub(crate) struct EntrancePageLayout<'a>{
    ref_app: &'a App
}
impl<'a> EntrancePageLayout<'a>{
    pub fn new(app: &'a mut App) -> Self {
        EntrancePageLayout {
            ref_app: app
        }
    }
}
impl<'a> Widget for EntrancePageLayout<'a>{
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {

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
        let area_inner: Rect = block.inner(area);
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let rects: Rc<[Rect]> = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(3)
        ])
            .flex(Flex::Center)
            .split(rect);
        // Render Section
        block.render(area, buf);
        title.render(rects[0], buf);
        text_actions.render(rects[1], buf);

    }
}

pub(crate) struct CreateGlyphView<'a>{
    mut_ref_app: &'a mut App
}
impl<'a> CreateGlyphView<'a>{
    pub fn new(app: &'a mut App) -> Self {
        CreateGlyphView {
            mut_ref_app: app
        }
    }
}
impl<'a> Widget for CreateGlyphView<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let areas: Rc<[Rect]> = Layout::vertical(
            [
                Constraint::Fill(1),
                Constraint::Length(1)
            ]
        ).split(area);

        let frame: Block = Block::default().borders(Borders::ALL).title(" Create Glyph ");
        let inner_area: Rect = frame.inner(areas[0]);

        frame.render(areas[0], buf);

        let file_explorer_area = inner_area
            .centered(Constraint::Max(42), Constraint::Percentage(50));
        DirectoryWidget::new(self.mut_ref_app).render(file_explorer_area, buf);
        Paragraph::new("Create (c) Back (q)").alignment(Alignment::Right).render(areas[1], buf);
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
        StatefulWidget::render(list, area, buf, self.ref_mut_app.widget_states.active_list_state_mut().unwrap());
    }
}

