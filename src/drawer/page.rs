use crate::app::page::{CreateGlyphPage, EntrancePage};
use crate::app::widget::SimpleButton;
use crate::drawer::Drawable;
use crate::event_handler::Focusable;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Widget};
use std::rc::Rc;
use tui_big_text::{BigText, PixelSize};

impl Drawable for EntrancePage {
    fn render(&self, area: Rect, buf: &mut Buffer)
    {
        /*
            Outer Frame
         */
        let block: Block;
        if self.is_focused() {
            block = Block::default().borders(Borders::ALL).border_type(BorderType::Double);
        }else{
            block = Block::default().borders(Borders::ALL);
        }
        /*
           Title
         */
        let title = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec![
                "Glyph".magenta().into(),
            ])
            .alignment(HorizontalAlignment::Center)
            .build();
        /*

         */
        let area_inner: Rect = block.inner(area);
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let rects: Rc<[Rect]> = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(3)
        ])
            .flex(Flex::Center)
            .split(rect);
        let button_rects = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1)
        ]).split(rects[1]);
        // Render Section
        block.render(area, buf);
        title.render(rects[0], buf);
        for (i, button_interactable) in (&self.interactables).iter().enumerate() {
            if let Some(simple_button) = (button_interactable).as_any().downcast_ref::<SimpleButton>(){
                if let Some(ci) = self.hover_index {
                    if i == ci {
                        simple_button.render_highlighted(button_rects[i], buf);
                    } else {
                        simple_button.render(button_rects[i], buf);
                    }
                }else{
                    simple_button.render(button_rects[i], buf);
                }
            }


        }
    }
}
impl Drawable for CreateGlyphPage {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        /*
            Outer Frame
         */
        let block: Block;
        if self.is_focused() {
            block = Block::default().borders(Borders::ALL).border_type(BorderType::Double).title("Create glyph page");
        }else{
            block = Block::default().borders(Borders::ALL).title("Create glyph page");
        }
        /*

         */
        let inner_area: Rect = block.inner(area);
        let file_explorer_area = inner_area
            .centered(Constraint::Max(42), Constraint::Percentage(50));

        block.render(area, buf);
        /*
            Directory Widget
         */
        // let list_items: Vec<ListItem> = get_dir_names(&self.)
        //     .unwrap_or(Vec::new())
        //     .iter()
        //     .enumerate()
        //     .map(
        //         |(i, item)| {
        //             return ListItem::new(item.clone());
        //         }
        //     )
        //     .collect();
        //
        //
        // let current_path: String = (&self.current_path).clone().to_str().unwrap_or("Invalid Path").to_string();
        //
        // let list_widget = List::new(list_items)
        //     .block(
        //         Block::bordered().title(current_path)
        //     )
        //     .highlight_style(Style::new().bold());
        //
        // list_widget.render(file_explorer_area, buf);

    }
}
