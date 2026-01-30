use std::cell::Ref;
use crate::app::page::{CreateGlyphPage, EntrancePage, GlyphNavigationBar, GlyphPage, GlyphViewer, OpenGlyphPage};
use crate::drawer::{get_draw_flag, DrawFlag, Drawable};
use crate::event_handler::Focusable;
use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Layout, Offset, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap};
use ratatui::Frame;
use std::rc::Rc;
use tui_big_text::{BigText, PixelSize};
use crate::model::{Entry, LocalEntryState, Section};
use crate::state::page::{GlyphMode, GlyphViewerState};

macro_rules! block {
    ($title: expr, $flag: expr) => {
         match $flag {
            DrawFlag::DEFAULT => {
                Block::bordered().title($title)
            }
            DrawFlag::HIGHLIGHTING => {
                Block::bordered().title(Line::from($title).bold()).border_type(BorderType::Double)
            }
            DrawFlag::FOCUSED => {
                Block::bordered().title(Line::from($title).bold()).border_type(BorderType::Thick)
            }
        }
    };
}


impl Drawable for EntrancePage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Container Frame
        */
        let block: Block = block!("Glyph", draw_flag);
        /*
          Title
        */
        let title = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec!["Glyph".magenta().into()])
            .alignment(HorizontalAlignment::Center)
            .build();

        /*

        */
        let area_inner: Rect = block.inner(area);
        let rect: Rect = area_inner.centered(Constraint::Fill(1), Constraint::Ratio(1, 2));
        let rects: Rc<[Rect]> = Layout::vertical([Constraint::Length(8), Constraint::Length(3)])
            .flex(Flex::Center)
            .split(rect);
        let button_rects = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(rects[1]);
        // Render Section
        block.render(area, frame.buffer_mut());
        title.render(rects[0], frame.buffer_mut());
        for (i, button_interactable) in (&self.components).iter().enumerate() {
            if let Some(ci) = self.state.hovered_index {
                if i == ci {
                    button_interactable.render(frame, button_rects[i], DrawFlag::HIGHLIGHTING);
                } else {
                    button_interactable.render(frame, button_rects[i], DrawFlag::DEFAULT);
                }
            }
            button_interactable.render(frame, button_rects[i], DrawFlag::DEFAULT);
        }
    }
}
impl Drawable for CreateGlyphPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Outer Frame
        */
        let page_frame: Block = block!("Create Glyph", draw_flag);
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
        );
        self.components[0].render(
            frame,
            button_areas[0],
            get_draw_flag(self.state.hovered_index, 1, None),
        );
        self.components[1].render(
            frame,
            button_areas[1],
            get_draw_flag(self.state.hovered_index, 2, None),
        );
        /*
            Dialog
         */
        if !self.dialogs.is_empty() {
            for (i, dialog) in self.dialogs.iter().enumerate() {
                if i == self.dialogs.len() - 1 {
                    dialog.render(frame, area, DrawFlag::FOCUSED);
                } else {
                    dialog.render(frame, area, DrawFlag::DEFAULT);
                }
            }
        }
    }
}
impl Drawable for OpenGlyphPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Outer Frame
        */
        let page_frame: Block = block!("Open Glyph", draw_flag);
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
        );
        self.components[0].render(
            frame,
            button_areas[0],
            get_draw_flag(self.state.hovered_index, 1, None),
        );
        self.components[1].render(
            frame,
            button_areas[1],
            get_draw_flag(self.state.hovered_index, 2, None),
        );
    }
}

impl Drawable for GlyphPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Outer Frame
        */
        let page_frame: Block = block!("Glyph", draw_flag);


        let chunks= Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(2)
        ]).split(area);

        let page_area: Rect = page_frame.inner(chunks[0]);
        let content_areas = Layout::horizontal([Constraint::Length(24), Constraint::Min(24)]).split(page_area);


        page_frame.render(chunks[0], frame.buffer_mut());
        self.containers[0].render(frame, content_areas[0], get_draw_flag(self.state.hovered_index, 0, Some(self.containers[0].is_focused())));
        self.containers[1].render(frame, content_areas[1], get_draw_flag(self.state.hovered_index, 1, Some(self.containers[1].is_focused())));


        /*
            Dialog
         */
        if !self.dialogs.is_empty() {
            for (i, dialog) in self.dialogs.iter().enumerate() {
                if i == self.dialogs.len() - 1 {
                    dialog.render(frame, area, DrawFlag::FOCUSED);
                } else {
                    dialog.render(frame, area, DrawFlag::DEFAULT);
                }
            }
        }
    }
}
/*
    Navigation Bar (Subpage)
 */

impl Drawable for GlyphNavigationBar {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Container Frame
        */
        let widget_frame: Block = block!("Entries", draw_flag);

        /*
            List Items (Entry)
         */
        let ref_entry_state = self.state.entry_state.borrow();
        let mut list: Vec<(i64, String)> = ref_entry_state.entries
            .iter()
            .map(|(id, entry): (&i64, &Entry)| (id.clone() , entry.entry_name.clone())).collect();
        let mut list_items: Vec<Line> = list
            .iter()
            .enumerate()
            .map(|(i, (id, name)): (usize, &(i64, String))| {
                let mut line: Line;
                // // If Selected => " >"
                if let Some(selected_id) = ref_entry_state.active_entry_id {
                    if selected_id == *id {
                        line = Line::from(String::from(" > ") + &*name.clone());
                    } else {
                        line = Line::from(String::from("   ") + &*name.clone());
                    }
                } else {
                    line = Line::from(String::from("   ") + &*name.clone());
                }

                // If Hovered => Bold
                if let Some(hovered_index) = self.state.hovered_index {
                    if hovered_index == i {
                        line = line.bold()
                    }
                }


                line
            }).collect();


        let inner_area: Rect = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());
        for (i, line) in list_items[self.state.offset..].iter().enumerate() {
            if i * self.state.line_height >= inner_area.height as usize {
                break;
            }
            (line as &Line).render(
                inner_area.offset(Offset {
                    x: 0,
                    y: (i * &self.state.line_height) as i32,
                }),
                frame.buffer_mut(),
            );
        }
    }
}

impl Drawable for GlyphViewer {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("Content", draw_flag);
        match self.state.mode {
            GlyphMode::READ => {
                widget_frame = widget_frame.title_top(Line::from("(READ)").right_aligned());
            }
            GlyphMode::LAYOUT => {
                widget_frame = widget_frame.title_top(Line::from("(LAYOUT)").right_aligned());
            }
            GlyphMode::EDIT => {
                widget_frame = widget_frame.title_top(Line::from("(EDIT)").right_aligned());
            }
        }
        let inner_areas = Layout::horizontal(
[
                Constraint::Fill(1),
                Constraint::Length(72),
                Constraint::Fill(1)
            ]
        ).split(widget_frame.inner(area));
        let content_area: Rect = inner_areas[1];
        widget_frame.render(area, frame.buffer_mut());


        /*

         */
        match self.state.mode  {
            /*
                READ
             */
            GlyphMode::READ => {
            }
            /*
                LAYOUT
             */
            GlyphMode::LAYOUT => {
            }
            /*
                EDIT
             */
            GlyphMode::EDIT => {
                draw_edit_view(self, frame, content_area, draw_flag);
            }
        }
    }
}

fn draw_layout_view(me: &GlyphViewer, frame: &mut Frame, layout_area: Rect, draw_flag: DrawFlag) {
    
}
fn draw_edit_view(me: &GlyphViewer, frame: &mut Frame, section_area: Rect, draw_flag: DrawFlag) {
    let entry_state:Ref<LocalEntryState> = me.state.entry_state.borrow();
    let active_entry_id: i64 = entry_state.active_entry_id.unwrap();
    let entry= entry_state.entries.get(&active_entry_id).unwrap();
    let mut section_list: Vec<(&i64, &Section)> = entry.sections.iter().map(
        |(key, value): (&i64, &Section)| {
            (key, value)
        }
    ).collect::<Vec<(&i64, &Section)>>();
    section_list.sort_by_key(|item|{*((*item).0)});

    let draw_section_list: Vec<((u32, u32), Paragraph)> = section_list.iter().enumerate().map(
        |(i, (key, value)): (usize, &(&i64, &Section))| {
            let text = Text::from(value.content.clone());
            let mut section_dimension: (u32, u32) = (Text::width(&text) as u32 + 2, Text::height(&text) as u32 + 3);

            let mut paragraph_frame: Block = Block::bordered();
            if let Some(index) = me.state.section_hover_index {
                if index == i {
                    paragraph_frame = paragraph_frame.border_type(BorderType::Double)
                }
            }
            let paragraph = Paragraph::new(text)
                .block(paragraph_frame.title(value.title.clone()).title_bottom(value.position.to_string()));

            return (section_dimension, paragraph);
        }
    ).collect();


    let mut stack_height: u16 = 0u16;
    let section_constraints: Vec<Constraint> = draw_section_list.iter().take_while(
        |((w,h), paragraph): &&((u32, u32), Paragraph)|{
            if stack_height>section_area.height {
                return false;
            }
            stack_height += *h as u16;
            true
        }).map(
        |((w,h),_)| {
            Constraint::Length(*h as u16)
        }
    ).collect();
    let section_areas = Layout::vertical(section_constraints).split(section_area);

    for (index, (_ ,paragraph)) in draw_section_list.iter().enumerate() {
        if index >= section_areas.len() {
            break;
        }
        paragraph.render(section_areas[index], frame.buffer_mut());
    }

}