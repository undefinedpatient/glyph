use std::path::PathBuf;
use std::string::String;

pub enum Screen {
    ExploreView,
}

// The State Object hold all the data in Navi
pub struct App {
    pub current_path: PathBuf,
    pub screen: Screen,

    info_message: String,
    should_quit: bool
}

impl App {
    pub fn new(path: &PathBuf) -> App {
        App {
            current_path: path.clone(),
            screen: Screen::ExploreView,
            info_message: String::new(),
            should_quit: false,
        }
    }
    pub fn set_should_quit(&mut self, flag: bool) {
        self.should_quit = flag;
    }
    pub fn should_quit(&mut self) -> bool {
        return self.should_quit;
    }
}