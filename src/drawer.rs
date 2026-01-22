mod entrance;

use ratatui::style::Stylize;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::Frame;
use crate::app::{Application};

pub enum DrawType {
    Full,
    Partial
}
pub trait Drawable {
    fn draw_type(&self) -> DrawType;
    fn draw(&self, frame: &mut Frame);
}
pub fn draw(frame: &mut Frame, app: &mut Application) {
    const MAX_FULLSCREEN: u8 = 1;
    let mut fullscreen_count: u8 = 0;
    for view in (*app.views).iter_mut() {
        match view.as_drawable().draw_type() {
            DrawType::Full => {
                if fullscreen_count > MAX_FULLSCREEN {
                    continue;
                }
                view.as_drawable().draw(frame);
                fullscreen_count += 1;
            }
            DrawType::Partial => {
                view.as_drawable().draw(frame);
                
            }
        }
    }
    
}

