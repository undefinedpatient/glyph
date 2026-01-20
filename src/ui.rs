use std::rc::Rc;

use ratatui::symbols::block;
use ratatui::{Frame, text};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap};
use ratatui::layout::{self, Alignment, Constraint, Direction, Flex, HorizontalAlignment, Layout, Rect};
use tui_big_text::{BigText, PixelSize};

use crate::app::{App, Popup, PopupConfirmType, View};
use crate::utils::get_dir_names;
use crate::{utils::get_file_names};

pub fn ui(frame: &mut Frame, app: &mut App) {
    if let Some(view) = app.peek_view_ref() {
        match view {
            View::Entrance => {
                frame.render_widget(
                    EntranceView::new(app),
                    frame.area()
                );
            },
            View::CreateGlyph => {
                frame.render_widget(
                    CreateGlyphView::new(app),
                    frame.area()
                );
            }
            _ => {}
        }

    }
    if app.state.error_message().is_some() {
        app.push_error_message();
    } else {
        if app.state.warning_message().is_some() {
            app.push_warning_message();
        } else {
            if app.state.info_message().is_some() {
                app.push_info_message();
            }
        }
    }
    if let Some(popup) = app.peek_popup_ref() {
        frame.render_widget(    PopupWidget::new(popup), frame.area());
    }
}

// Widget itself must not own any recources, and never outlive the AppState it references to.
struct EntranceView<'a>{
    ref_app: &'a App
}
impl<'a> EntranceView<'a>{
    fn new(app: &'a mut App) -> Self {
        EntranceView { 
            ref_app: app
        }
    }
}
impl<'a> Widget for EntranceView<'a>{
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

struct CreateGlyphView<'a>{
    mut_ref_app: &'a mut App
}
impl<'a> CreateGlyphView<'a>{
    fn new(app: &'a mut App) -> Self {
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

struct PopupWidget<'a> {
    ref_popup: &'a Popup
}
impl<'a> PopupWidget<'a> {
    fn new(popup: &'a Popup) -> Self {
        PopupWidget {
            ref_popup: popup
        }
    }
}
impl<'a> Widget for PopupWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        match self.ref_popup {
            Popup::Confirm(popup_t) => {
                match popup_t {
                    PopupConfirmType::Exit => {
                        let area: Rect = area.centered(Constraint::Length(42), Constraint::Length(6));
                        let paragraph_message: Paragraph = Paragraph::new("Confirm Exit?")
                            .wrap(Wrap {trim:true})
                            .alignment(Alignment::Center)
                            .block(
                                Block::bordered()
                                .padding(Padding::uniform(1))
                                .title_top(Line::from("Confirmation").right_aligned())
                                .title_bottom(Line::from("Confirm (Y) Cancel (n)").right_aligned())
                            );

                        Clear.render(area, buf);
                        paragraph_message.render(area, buf);
                    }
                    _ => {}
                }
            }
            Popup::Info(message) => {
                let area: Rect = area.centered(Constraint::Length(42), Constraint::Length(6));
                let paragraph_message: Paragraph = Paragraph::new(message.as_str())
                    .wrap(Wrap {trim:true})
                    .alignment(Alignment::Center)
                    .block(
                        Block::bordered()
                        .padding(Padding::uniform(1))
                        .title_top(Line::from("Info").centered())
                        .title_bottom(Line::from("Understood").centered())
                    );

                Clear.render(area, buf);
                paragraph_message.render(area, buf);
            }
            Popup::Warning(message) => {
                let area: Rect = area.centered(Constraint::Length(48), Constraint::Length(7));
                let paragraph_message: Paragraph = Paragraph::new(message.as_str())
                    .wrap(Wrap {trim:true})
                    .alignment(Alignment::Center)
                    .block(
                        Block::bordered()
                        .style(Style::new().yellow())
                        .padding(Padding::uniform(1))
                        .title_top(Line::from("Warning").yellow().centered())
                        .title_bottom(Line::from("Understood").centered())
                    );

                Clear.render(area, buf);
                paragraph_message.render(area, buf);
            }
            Popup::Error(message) => {
                let area: Rect = area.centered(Constraint::Length(64), Constraint::Length(8));
                let paragraph_message: Paragraph = Paragraph::new(message.as_str())
                    .wrap(Wrap {trim:true})
                    .alignment(Alignment::Center)
                    .block(
                        Block::bordered()
                        .style(Style::new().red())
                        .padding(Padding::uniform(1))
                        .title_top(Line::from("Error").red().centered())
                        .title_bottom(Line::from("Understood").centered())
                    );

                Clear.render(area, buf);
                paragraph_message.render(area, buf);
            }
            _ => {}
        }
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

