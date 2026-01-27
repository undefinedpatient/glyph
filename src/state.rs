pub mod dialog;
pub mod page;
mod widget;

// Global State of the Application
pub struct GlobalState {
    pub should_quit: bool,
    pub db_connection: Option<rusqlite::Connection>,
}
