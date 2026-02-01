use crate::app::page::{CreateGlyphPage, EntrancePage, GlyphEditContentView, GlyphNavigationBar, GlyphPage, GlyphOldViewer, OpenGlyphPage, GlyphViewer, GlyphReadView, GlyphLayoutView, GlyphEditView};
use crate::drawer::{get_draw_flag, DrawFlag, Drawable};
use crate::event_handler::Focusable;
use crate::model;
use crate::model::{LayoutOrientation, LocalEntryState, Section};
use crate::state::page::{GlyphEditContentState, GlyphMode, GlyphReadState};
use color_eyre::owo_colors::OwoColorize;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Flex, HorizontalAlignment, Layout, Offset, Rect, Size};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Paragraph, Widget, Wrap};
use ratatui::Frame;
use std::cell::Ref;
use std::collections::HashMap;
use std::rc::Rc;
use serde::de::Unexpected::Option;
use tui_big_text::{BigText, PixelSize};

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
        let plain_entries: Vec<(i64, String)> = ref_entry_state.entries.iter().map(
            |(id,entry)| {
                (id.clone(), entry.entry_name.clone())
            }
        ).collect();
        let mut list_items: Vec<Line> = plain_entries
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

impl Drawable for GlyphOldViewer {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("Content", draw_flag);
        match self.state.mode {
            GlyphMode::Read => {
                widget_frame = widget_frame.title_top(Line::from("[ READ ]").right_aligned());
            }
            GlyphMode::Layout => {
                widget_frame = widget_frame.title_top(Line::from("[ LAYOUT ]").right_aligned());
            }
            GlyphMode::Edit => {
                widget_frame = widget_frame.title_top(Line::from("[ EDIT ]").right_aligned());
            }
            GlyphMode::EditContent => {
                widget_frame = widget_frame.title_top(Line::from("[ EDIT TEXT ]").right_aligned());
            }
        }
        let inner_area = widget_frame.inner(area);
        let inner_areas = Layout::horizontal(
[
                Constraint::Fill(1),
                Constraint::Length(144),
                Constraint::Fill(1)
            ]
        ).split(inner_area);
        let content_area: Rect = inner_areas[1];
        widget_frame.render(area, frame.buffer_mut());


        /*

         */
        if self.state.local_entry_state_ref().unwrap().active_entry_id.is_none() {
            let message: Paragraph = Paragraph::new("No Entry Selected").alignment(Alignment::Center);
            let center_area = content_area.centered(Constraint::Fill(1), Constraint::Length(3));
            message.render(center_area, frame.buffer_mut());
            return;
        }
        match self.state.mode  {
            /*
                READ
             */
            GlyphMode::Read => {
                draw_read_view(self, inner_areas[1], frame.buffer_mut(), draw_flag);
            }
            /*
                LAYOUT
             */
            GlyphMode::Layout => {
                let virtual_area = Size::new(content_area.width, content_area.height*3);
                // let mut scroll_view = ScrollView::new(virtual_area);
                // scroll_view = scroll_view.horizontal_scrollbar_visibility(ScrollbarVisibility::Never);
                draw_layout_view(self, inner_areas[1], frame.buffer_mut(), draw_flag);
                // scroll_view.render(inner_area, frame.buffer_mut(), &mut self.state.scroll_state.borrow_mut());
            }
            /*
               EDIT
             */
            GlyphMode::Edit => {
                draw_edit_view(self, frame, content_area);
            }
            /*
               EDIT CONTENT
             */
            GlyphMode::EditContent => {
                self.mode_views[3].as_ref().unwrap().render(frame, content_area, DrawFlag::DEFAULT);
            }
        }


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

fn draw_layout_view(me: &GlyphOldViewer, layout_area: Rect, buffer: &mut Buffer, draw_flag: DrawFlag) {
    let entry_state: Ref<LocalEntryState> = me.state.local_entry_state_ref().unwrap();
    let entry_layout: &model::Layout = &entry_state.get_active_entry_ref().unwrap().layout.1;
    evaluate_layout(me, layout_area, buffer, entry_layout, 0,0);
}

fn draw_read_view(me: &GlyphOldViewer, layout_area: Rect, buffer: &mut Buffer, draw_flag: DrawFlag) {

}


fn evaluate_layout(me: &GlyphOldViewer, area: Rect, buffer: &mut Buffer, layout: &model::Layout, depth: u16, at: usize) -> Vec<(u16, Rect)>{
    let mut target_section_text: String = "None".to_string();
    if let Some(position_target) = layout.section_index {
        target_section_text = position_target.to_string();
    }

    let focused_coordinate = me.state.layout_selected_coordinate.clone();
    let mut block: Block = Block::bordered().title(layout.label.as_str());
    if depth == focused_coordinate.len() as u16 {
        if let Some(focused_index) = focused_coordinate.last() {
            if *focused_index == at {
                block = block.border_type(BorderType::Thick);
            }
        } else if depth == 0 {
            block = block.border_type(BorderType::Thick);
        }
    } else if let Some(hovered_index) = me.state.layout_hovered_index {
        if depth == focused_coordinate.len() as u16 + 1 && at == hovered_index  {
            block = block.border_type(BorderType::Double);
        }
    }
    if !layout.sub_layouts.is_empty() {
        block = block.title_bottom(Line::from(target_section_text).dim());
    } else {
        block = block.title_bottom(Line::from(target_section_text));

    }

    let recursive_area: Rect = block.inner(area);
    block.render(area, buffer);


    // Process the child
    let constraints: Vec<Constraint> = layout.sub_layouts.iter().enumerate().map(
        |(index, sub)| {
            if (sub.details.flex != 0) {
                Constraint::Fill(sub.details.flex)
            } else {
                Constraint::Length(sub.details.length)
            }
        }
    ).collect();
    let sub_areas =
        match layout.details.orientation {
            LayoutOrientation::Vertical => {
                Layout::vertical(constraints).split(recursive_area)
            }
            LayoutOrientation::Horizontal => {
                Layout::horizontal(constraints).split(recursive_area)
            }
        };

    let mut areas: Vec<(u16, Rect)> = vec![];
    if let Some(section_index) = layout.section_index {
        areas.push((section_index, recursive_area));
    }
    for (i, sub_layout) in layout.sub_layouts.iter().enumerate() {
        areas = [areas, evaluate_layout(
            me,
            sub_areas[i],
            buffer,
            sub_layout,
            depth + 1,
            i,
        )].concat()
    }
    areas
}

fn draw_edit_view(me: &GlyphOldViewer, frame: &mut Frame, section_area: Rect) {
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
            if let Some(index) = me.state.edit_hovered_index {
                if index == i {
                    paragraph_frame = paragraph_frame.border_type(BorderType::Double)
                }
            }
            if let Some(sid) = me.state.edit_selected_sid {
                if sid == **key {
                    paragraph_frame = paragraph_frame.border_type(BorderType::Thick)
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

impl Drawable for GlyphViewer {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("Content", draw_flag);
        match self.state.mode {
            GlyphMode::Read => {
                widget_frame = widget_frame.title_top(Line::from("[ READ ]").right_aligned());
            }
            GlyphMode::Layout => {
                widget_frame = widget_frame.title_top(Line::from("[ LAYOUT ]").right_aligned());
            }
            GlyphMode::Edit => {
                widget_frame = widget_frame.title_top(Line::from("[ EDIT ]").right_aligned());
            }
            GlyphMode::EditContent => {
                widget_frame = widget_frame.title_top(Line::from("[ EDIT TEXT ]").right_aligned());
            }
        }
        let inner_area = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());
        /*
            Body
         */
        if self.state.local_entry_state_ref().unwrap().active_entry_id.is_none() {
            let message: Paragraph = Paragraph::new("No Entry Selected").alignment(Alignment::Center);
            let center_area = inner_area.centered(Constraint::Fill(1), Constraint::Length(3));
            message.render(center_area, frame.buffer_mut());
            return;
        }
        match self.state.mode {
            GlyphMode::Read => {
                self.containers[0].as_ref().render(frame, inner_area, draw_flag);
            }
            GlyphMode::Layout => {
                self.containers[1].as_ref().render(frame, inner_area, draw_flag);
            }
            GlyphMode::Edit => {
                self.containers[2].as_ref().render(frame, inner_area, draw_flag);
            }
            GlyphMode::EditContent => {
                self.containers[3].as_ref().render(frame, inner_area, draw_flag);
            }
        }
    }
}
impl Drawable for GlyphReadView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let entry_state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
        let entry_layout: &model::Layout = &entry_state.get_active_entry_ref().unwrap().layout.1;
        let areas: Vec<(u16, Rect)> = evaluate_read_areas(self, area, entry_layout, 0,0);
        let ref_sections: &HashMap<i64, Section> = &entry_state.get_active_entry_ref().unwrap().sections;
        for (sid, section) in ref_sections {
            if let Some((position, area)) = areas.iter().find(
                |(position, area)|{
                    *position as i64 == section.position
                }
            ) {
                Paragraph::new(section.content.clone()).wrap(Wrap { trim: true }).render(*area, frame.buffer_mut());
            }
        }

    }
}
fn evaluate_read_areas(me: &GlyphReadView, area: Rect, layout: &model::Layout, depth: u16, at: usize) -> Vec<(u16, Rect)> {
    let mut target_section_text: String = "None".to_string();
    if let Some(position_target) = layout.section_index {
        target_section_text = position_target.to_string();
    }

    let mut block: Block = Block::bordered().title(layout.label.as_str()).title_bottom(target_section_text);
    let recursive_area: Rect = block.inner(area);
    // Process the child
    let constraints: Vec<Constraint> = layout.sub_layouts.iter().enumerate().map(
        |(index, sub)| {
            if (sub.details.flex != 0) {
                Constraint::Fill(sub.details.flex)
            } else {
                Constraint::Length(sub.details.length)
            }
        }
    ).collect();
    let sub_areas =
        match layout.details.orientation {
            LayoutOrientation::Vertical => {
                Layout::vertical(constraints).split(recursive_area)
            }
            LayoutOrientation::Horizontal => {
                Layout::horizontal(constraints).split(recursive_area)
            }
        };

    let mut areas: Vec<(u16, Rect)> = vec![];
    if let Some(section_index) = layout.section_index {
        if layout.sub_layouts.is_empty() {
            areas.push((section_index, area));
        }
    }
    for (i, sub_layout) in layout.sub_layouts.iter().enumerate() {
        areas = [areas,evaluate_read_areas(
            me,
            sub_areas[i],
            sub_layout,
            depth + 1,
            i,
        )].concat()
    }
    areas

}
impl Drawable for GlyphLayoutView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {

    }

}
impl Drawable for GlyphEditView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {

    }
}


impl Drawable for GlyphEditContentView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag) {
        let lr_areas = Layout::horizontal( [Constraint::Length(42), Constraint::Fill(1)] ).split(area);
        let title_button_areas = Layout::vertical([Constraint::Length(5), Constraint::Fill(3)]).flex(Flex::SpaceBetween).split(lr_areas[0]);
        let button_areas = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(title_button_areas[1]);

        let hover_index = self.state.hovered_index;
        self.containers[0].render(frame, title_button_areas[0],
                                  get_draw_flag(hover_index, 0, Some(self.containers[0].is_focused()))
        );
        self.containers[1].render(frame, lr_areas[1],
                                  get_draw_flag(hover_index, 1, Some(self.containers[1].is_focused()))
        );
        self.components[0].render(frame, button_areas[0],
                                  get_draw_flag(hover_index, 2, Some(self.containers[0].is_focused()))
        );
        self.components[1].render(frame, button_areas[1],
                                  get_draw_flag(hover_index, 3, Some(self.containers[1].is_focused()))
        );
    }
}