use crate::theme::{Iceberg, Theme};

pub mod dialog;
pub mod page;
pub mod widget;
// Global State of the Application
pub struct AppState {
    pub theme: Iceberg,
    pub should_quit: bool,
}