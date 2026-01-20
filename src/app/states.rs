use std::path::{Path, PathBuf};


// This represent the Application State 
pub struct ApplicationState{
    current_path: PathBuf,
    info_message: Option<String>,
    warning_message: Option<String>,
    error_message: Option<String>,
    should_quit: bool,
}
impl ApplicationState{
    pub fn new() -> Self {
        let path_buf: PathBuf = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
        ApplicationState { 
            current_path: path_buf,
            info_message: None, 
            warning_message: None, 
            error_message: None, 
            should_quit: false
        }
    }
    pub fn set_should_quit(&mut self, flag: bool) -> () {
        self.should_quit = flag;
    }
    pub fn get_should_quit(&mut self) -> &bool {
        &self.should_quit
    }
    pub fn get_current_path(&self) -> &PathBuf {
        &self.current_path
    }
    pub fn set_current_path(&mut self, path_buf: &PathBuf) -> () {
        self.current_path = path_buf.clone();
    }
    // INFO / WARNING / ERROR Messages
    pub fn set_info_message(&mut self, message: &str) -> () {
        self.info_message = Some(String::from(message));
    }
    pub fn set_warning_message(&mut self, message: &str) -> () {
        self.warning_message = Some(String::from(message));
    }
    pub fn set_error_message(&mut self, message: &str) -> () {
        self.error_message = Some(String::from(message));
    }
    pub fn reset_info_message(&mut self) -> () {
        self.info_message = None;
    }
    pub fn reset_warning_message(&mut self) -> () {
        self.warning_message = None;
    }
    pub fn reset_error_message(&mut self) -> () {
        self.error_message = None;
    }
    pub fn info_message(&self) -> Option<String> {
        self.info_message.clone()
    }
    pub fn warning_message(&self) -> Option<String> {
        self.warning_message.clone()
    }
    pub fn error_message(&self) -> Option<String> {
        self.error_message.clone()
    }
}
// This represent the Widget State
pub struct WidgetState{}