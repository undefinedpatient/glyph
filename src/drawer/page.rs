use crate::app::page::{CreateGlyphPage, EntrancePage, GlyphEditContentView, GlyphEditOrderView, GlyphEditView, GlyphLayoutEditView, GlyphLayoutOverview, GlyphLayoutView, GlyphNavigationBar, GlyphPage, GlyphReadView, GlyphViewer, OpenGlyphPage};
use crate::drawer::{get_draw_flag, DrawFlag, Drawable};
use crate::event_handler::Focusable;
use crate::model;
use crate::model::{LayoutOrientation, LocalEntryState, Section};
use crate::state::page::GlyphMode;
use crate::theme::Theme;
use color_eyre::owo_colors::OwoColorize;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Flex, HorizontalAlignment, Layout, Offset, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Padding, Paragraph, Widget, Wrap};
use ratatui::Frame;
use std::cell::Ref;
use std::collections::HashMap;
use std::rc::Rc;
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
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
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
                    button_interactable.render(frame, button_rects[i], DrawFlag::HIGHLIGHTING, theme);
                } else {
                    button_interactable.render(frame, button_rects[i], DrawFlag::DEFAULT, theme);
                }
            }
            button_interactable.render(frame, button_rects[i], DrawFlag::DEFAULT, theme);
        }
    }
}
impl Drawable for CreateGlyphPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
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
            theme
        );
        self.components[0].render(
            frame,
            button_areas[0],
            get_draw_flag(self.state.hovered_index, 1, None),
            theme
        );
        self.components[1].render(
            frame,
            button_areas[1],
            get_draw_flag(self.state.hovered_index, 2, None),
            theme
        );
        /*
            Dialog
         */
        if !self.dialogs.is_empty() {
            for (i, dialog) in self.dialogs.iter().enumerate() {
                if i == self.dialogs.len() - 1 {
                    dialog.render(frame, area, DrawFlag::FOCUSED, theme);
                } else {
                    dialog.render(frame, area, DrawFlag::DEFAULT, theme);
                }
            }
        }
    }
}
impl Drawable for OpenGlyphPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
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
            theme
        );
        self.components[0].render(
            frame,
            button_areas[0],
            get_draw_flag(self.state.hovered_index, 1, None),
            theme
        );
        self.components[1].render(
            frame,
            button_areas[1],
            get_draw_flag(self.state.hovered_index, 2, None),
            theme
        );
    }
}

impl Drawable for GlyphPage {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
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
        self.containers[0].render(frame, content_areas[0], get_draw_flag(self.state.hovered_index, 0, Some(self.containers[0].is_focused())), theme);
        self.containers[1].render(frame, content_areas[1], get_draw_flag(self.state.hovered_index, 1, Some(self.containers[1].is_focused())), theme);


        /*
            Dialog
         */
        if !self.dialogs.is_empty() {
            for (i, dialog) in self.dialogs.iter().enumerate() {
                if i == self.dialogs.len() - 1 {
                    dialog.render(frame, area, DrawFlag::FOCUSED, theme);
                } else {
                    dialog.render(frame, area, DrawFlag::DEFAULT, theme);
                }
            }
        }
    }
}
/*
    Navigation Bar (Subpage)
 */

impl Drawable for GlyphNavigationBar {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
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





impl Drawable for GlyphViewer {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("Content", draw_flag);
        match self.state.mode {
            GlyphMode::Read => {
                widget_frame = widget_frame.title_top(Line::from("[ READ ]").right_aligned());
            }
            GlyphMode::Edit => {
                widget_frame = widget_frame.title_top(Line::from("[ EDIT ]").right_aligned());
            }
            GlyphMode::Layout => {
                widget_frame = widget_frame.title_top(Line::from("[ LAYOUT ]").right_aligned());
            }
        }
        let inner_area = widget_frame.inner(area.centered_horizontally(Constraint::Percentage(90)));
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
                self.containers[0].as_ref().render(frame, inner_area, draw_flag, theme);
            }
            GlyphMode::Edit => {
                self.containers[1].as_ref().render(frame, inner_area, draw_flag, theme);
            }
            GlyphMode::Layout => {
                self.containers[2].as_ref().render(frame, inner_area, draw_flag, theme);
            }
        }
    }
}
impl Drawable for GlyphReadView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let entry_state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
        let eid: i64 = entry_state.active_entry_id.unwrap();
        let layout = entry_state.get_entry_layout_ref(&eid).unwrap();
        let areas: Vec<(u16, Rect)> = evaluate_read_areas(self, area, layout, 0,0);
        let ref_sections: &Vec<(i64, Section)> = &entry_state.get_sections_ref(&eid);
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

impl Drawable for GlyphEditView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let focused_panel_index = *self.state.focused_panel_index.borrow();
        let edit_areas = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).split(area);
        self.containers[0].render(frame, edit_areas[0],
                                  if !self.is_focused() {
                                      DrawFlag::DEFAULT
                                  }
                                  else if focused_panel_index == 0 {
                                      DrawFlag::FOCUSED
                                  } else {
                                      DrawFlag::DEFAULT
                                  },
                                  theme
        );
        self.containers[1].render(frame, edit_areas[1],
                                  if !self.is_focused() {
                                      DrawFlag::DEFAULT
                                  }
                                  else if focused_panel_index == 1 {
                                      DrawFlag::FOCUSED
                                  } else {
                                      DrawFlag::DEFAULT
                                  },
                                  theme
                                  ,
        );

    }
}




impl Drawable for GlyphEditOrderView{
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("", draw_flag).title(Line::from("(q)").right_aligned());
        let inner_area = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());



        let state = self.state.entry_state.borrow();
        let eid = state.active_entry_id.unwrap();
        let section_list: &Vec<(i64, Section)> = state.get_sections_ref(&eid);
        let edit_area = inner_area.centered_horizontally(Constraint::Percentage(90));
        let draw_section_list: Vec<((u32, u32), Paragraph)> = section_list.iter().enumerate().map(
            |(i, (key, value)): (usize, &(i64, Section))| {
                let text = Text::from(value.content.clone());
                let mut section_dimension: (u32, u32) = (Text::width(&text) as u32 + 2, Text::height(&text) as u32 + 3);

                let mut paragraph_frame: Block = Block::bordered();
                if let Some(index) = self.state.hovered_index {
                    if index == i {
                        paragraph_frame = paragraph_frame.border_type(BorderType::Double)
                    }
                }
                if let Ok(o_sid) = self.state.editing_sid.try_borrow() {
                    if let Some(sid) = *o_sid {
                        if sid == *key {
                            paragraph_frame = paragraph_frame.border_type(BorderType::Thick)
                        }
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
                if stack_height>area.height {
                    return false;
                }
                stack_height += *h as u16;
                true
            }).map(
            |((w,h),_)| {
                Constraint::Length(*h as u16)
            }
        ).collect();
        let section_areas = Layout::vertical(section_constraints).split(edit_area);

        for (index, (_ ,paragraph)) in draw_section_list.iter().enumerate() {
            if index >= section_areas.len() {
                break;
            }
            paragraph.render(section_areas[index], frame.buffer_mut());
        }
    }
}

impl Drawable for GlyphEditContentView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("", draw_flag).title_top("(e)");


        // When has no editing sid
        if self.state.editing_sid.borrow().is_none() {
            widget_frame = widget_frame.dim();
        }


        let inner_area = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());
        let lr_areas = Layout::horizontal( [Constraint::Max(24), Constraint::Fill(1)] ).split(inner_area);
        let title_button_areas = Layout::vertical([Constraint::Length(5), Constraint::Fill(3)]).flex(Flex::SpaceBetween).split(lr_areas[0]);
        let button_areas = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(title_button_areas[1]);

        let hover_index = self.state.hovered_index;
        self.containers[0].render(frame, title_button_areas[0],

                                  get_draw_flag(hover_index, 0, Some(self.containers[0].is_focused())),
                                  theme
        );
        self.containers[1].render(frame, lr_areas[1],
                                  get_draw_flag(hover_index, 1, Some(self.containers[1].is_focused())),
                                  theme
        );
        self.components[0].render(frame, button_areas[0],
                                  get_draw_flag(hover_index, 2, Some(self.containers[0].is_focused())),
                                  theme
        );
        self.components[1].render(frame, button_areas[1],
                                  get_draw_flag(hover_index, 3, Some(self.containers[1].is_focused())),
                                  theme
        );
    }
}

impl Drawable for GlyphLayoutView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let focused_panel_index = *self.state.focused_panel_index.borrow();
        let edit_areas = Layout::horizontal([Constraint::Percentage(100), Constraint::Length(32)]).split(area);
        self.containers[0].render(frame, edit_areas[0],
                                  if !self.is_focused() {
                                      DrawFlag::DEFAULT
                                  }
                                  else if focused_panel_index == 0 {
                                      DrawFlag::FOCUSED
                                  } else {
                                      DrawFlag::DEFAULT
                                  },
                                  theme
        );
        self.containers[1].render(frame, edit_areas[1],
                                  if !self.is_focused() {
                                      DrawFlag::DEFAULT
                                  }
                                  else if focused_panel_index == 1 {
                                      DrawFlag::FOCUSED
                                  } else {
                                      DrawFlag::DEFAULT
                                  },
                                  theme
                                  ,
        );

    }

}

impl Drawable for GlyphLayoutOverview {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("", draw_flag).title(Line::from("(q)").right_aligned());
        let inner_area = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());




        let entry_state: Ref<LocalEntryState> = self.state.local_entry_state_ref().unwrap();
        let eid: i64 = entry_state.active_entry_id.unwrap();
        let layout = entry_state.get_entry_layout_ref(&eid).unwrap();
        evaluate_layout(self, inner_area, frame.buffer_mut(), layout, 0, Vec::new());
    }
}

impl Drawable for GlyphLayoutEditView {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        /*
           Container Frame
        */
        let mut widget_frame: Block = block!("", draw_flag).title(Line::from("(e)").left_aligned());



        let inner_area = widget_frame.inner(area);
        widget_frame.render(area, frame.buffer_mut());
    }
}
fn evaluate_layout(me: &GlyphLayoutOverview, area: Rect, buffer: &mut Buffer, layout: &model::Layout, depth: u16, at: Vec<usize>) -> Vec<(u16, Rect)>{
    let mut target_section_text: String = "None".to_string();
    if let Some(position_target) = layout.section_index {
        target_section_text = position_target.to_string();
    }

    let focused_coordinate = &me.state.selected_coordinate.borrow().clone();

    let mut block: Block = Block::bordered().title(layout.label.as_str()).padding(Padding {left: 1, right: 1, top: 1, bottom: 1});
    if at == *focused_coordinate {
        block = block.border_type(BorderType::Thick).title_style(Style::new().bold());

    } else if let Some(hovered_index) = me.state.hovered_index {
        let mut hover_coordinate = me.state.selected_coordinate.borrow().clone();
        hover_coordinate.push(hovered_index);
        if hover_coordinate == at {
            block = block.border_type(BorderType::Double);
        }
    }
    if !layout.sub_layouts.is_empty() {
        block = block.title_bottom(Line::from("(Disabled)").dim());
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
        let mut sub_at = at.clone();
        sub_at.push(i);
        areas = [areas, evaluate_layout(
            me,
            sub_areas[i],
            buffer,
            sub_layout,
            depth + 1,
            sub_at
        )].concat()
    }
    areas
}
