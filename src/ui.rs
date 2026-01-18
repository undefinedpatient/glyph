use ratatui::Frame;
use ratatui::style::Stylize;
use ratatui::widgets::{Widget, Block, Borders, Padding, Paragraph};
use ratatui::layout::{self, Constraint, Direction, Layout, Rect};

use crate::{App, utils::get_file_names,};

pub fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Min(13), // Main Application
            Constraint::Length(1) // Info
        ]).split(frame.area());
    frame.render_widget(
        Paragraph::new(" Glyph v0.0.0").bold(), 
        layout[0]
    );
    frame.render_widget(
        ExplorerWidget::new(app),
        layout[1]
    );
    frame.render_widget(
        Paragraph::new("Quit (q)").right_aligned(),
        layout[2]
    );
}

// Widget itself must not own any recources, and never outlive the AppState it references to.
struct ExplorerWidget<'a> {
    app: &'a App
}
impl<'a> ExplorerWidget<'a> {
    fn new(app: &'a App) -> Self {
        ExplorerWidget { 
            app: app
        }
    }
}
impl<'a> Widget for ExplorerWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        let mut paragraph: String = String::from(self.app.current_path.to_str().unwrap_or_default().to_owned()+"\n");
        for file_name in get_file_names(&self.app.current_path).unwrap() {
            paragraph = paragraph + file_name.as_ref() + "\n";
        }
        Paragraph::new(paragraph).block(
                Block::default().borders(Borders::ALL).padding(Padding::left(1)))
        .render(area, buf);
    }
}


/**
 * Create a centered Rectangular Area
 */
fn centered_rect(percent_x: u16, percent_y: u16, r:Rect) -> Rect {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100-percent_y)/2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100-percent_y)/2),
            ]
        ).split(r);
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100-percent_x)/2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100-percent_x)/2),
            ]
        ).split(vertical_layout[1])[1]
}