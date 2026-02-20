use color_eyre::eyre::Result;
use color_eyre::Report;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Layout {
    pub label: String,
    pub section_index: Option<u16>,
    pub sub_layouts: Vec<Layout>,
    pub details: LayoutDetails,
}
impl Layout {
    pub fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
            section_index: None,
            sub_layouts: Vec::new(),
            details: LayoutDetails::new(),
        }
    }
    pub fn get_layout_at_ref(&self, coordinates: &Vec<usize>) -> Option<&Layout> {
        let mut coor = coordinates.clone();
        coor.reverse();
        if coor.len() == 0 {
            return Some(self);
        }
        let mut temp_layout: &Layout = self;
        while let Some(index) =  coor.pop() {
            temp_layout = &(*temp_layout).sub_layouts[index];
        }
        if coor.is_empty() {
            Some(temp_layout)
        } else {
            None
        }
    }
    pub fn get_layout_at_mut(&mut self, coordinates: &Vec<usize>) -> Option<&mut Layout> {
        let mut coor = coordinates.clone();
        coor.reverse();
        if coor.len() == 0 {
            return Some(self);
        }
        let mut temp_layout: &mut Layout = self;
        while let Some(index) =  coor.pop() {
            temp_layout = &mut (*temp_layout).sub_layouts[index];
        }
        if coor.is_empty() {
            Some(temp_layout)
        } else {
            None
        }
    }
    pub fn update_layout_at(&mut self, layout: &Layout, coordinates: &Vec<usize>) {
        if let Some(target) = self.get_layout_at_mut(coordinates){
            target.label = layout.label.clone();
            target.details = layout.details.clone();
        }
    }
    pub fn insert_sublayout_under(&mut self, layout: Layout, coordinates: &Vec<usize>) {
        if coordinates.is_empty() {
            self.sub_layouts.push(layout);
        } else {
            let coor = coordinates.clone();
            if let Some(target) = self.get_layout_at_mut(&coor) {
                target.sub_layouts.push(layout);
            }
        }
    }

    pub fn remove_sublayout(&mut self, coordinates: &Vec<usize>) -> Result<()> {
        let mut coor = coordinates.clone();
        let index = coor.pop().unwrap();
        if let Some(target) = self.get_layout_at_mut(&coor) {
            target.sub_layouts.remove(index);
            Ok(())
        } else {
            Err(Report::msg(format!("Tried to remove a layout that does not exist. At {:?}", coor)))
        }

    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LayoutOrientation {
    Horizontal,
    Vertical,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SizeMode {
    Length,
    Flex,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum BorderMode {
    None,
    Plain,
    Dashed,
    Rounded
}
#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutDetails {
    pub size_mode: SizeMode,
    pub border_mode: BorderMode,
    pub length: u16,
    pub flex: u16,
    pub padding: u16,
    pub margin: u16,

    pub orientation: LayoutOrientation, // Describing orientation main axis for the children
}



impl LayoutDetails {
    pub fn new() -> Self {
        Self {
            size_mode: SizeMode::Flex,
            border_mode: BorderMode::None,
            length: 42,
            flex: 1,
            padding: 0,
            margin: 0,

            orientation: LayoutOrientation::Vertical,
        }
    }
}
