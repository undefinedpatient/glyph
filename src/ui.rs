use std::rc::Rc;

use ratatui::{Frame, text};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Widget, Block, Borders, Padding, Paragraph, BorderType};
use ratatui::layout::{self, Alignment, Constraint, Direction, Flex, HorizontalAlignment, Layout, Rect};
use tui_big_text::{BigText, PixelSize};

use crate::{App, utils::get_file_names,};

pub fn ui(frame: &mut Frame, app: &App) {
    frame.render_widget(
        EntranceSection::new(app),
        frame.area()
    );
}

// Widget itself must not own any recources, and never outlive the AppState it references to.
struct EntranceSection<'a>{
    app: &'a App
}
impl<'a> EntranceSection<'a>{
    fn new(app: &'a App) -> Self {
        EntranceSection { 
            app: app
        }
    }
}
impl<'a> Widget for EntranceSection<'a>{
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
            Line::from("Create (A)"),
            Line::from("Open   (O)"),
            Line::from("Quit   (Q)"),
        ]).centered();
        let block: Block = Block::default().borders(Borders::ALL);
        let area_inner: Rect = block.inner(area);
        let layout: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let layouts: Rc<[Rect]> = Layout::vertical([
                Constraint::Length(8),
                Constraint::Length(3)
            ])
            .flex(Flex::Center)
            .split(layout);
        // Render Section
        block.render(area, buf);
        title.render(layouts[0], buf);
        text_actions.render(layouts[1], buf);

    }
}

struct PopupWidget {

}
impl PopupWidget {

}

