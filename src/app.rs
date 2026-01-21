pub mod states;
pub mod views;
pub mod widgets;
pub use states::*;
pub use views::*;
pub use widgets::*;

use std::collections::HashMap;

// The State Object hold all the data in Navi
pub struct App {
    // UI
    s_pages: Vec<PageView>,
    s_dialogs: Vec<DialogView>,
    s_popup: Vec<PopupView>,

    // Application Level State
    pub state: ApplicationState,
    pub h_page_states: HashMap<PageView, PageState>,
    pub h_dialog_states: HashMap<DialogView, DialogState>,
    // # A popup does not have state.
}

impl App {
    pub fn new() -> App {
        App {
            s_pages: vec![PageView::Entrance],
            s_dialogs: Vec::new(),
            s_popup: Vec::new(),

            state: ApplicationState::new(),
            h_page_states: HashMap::new(),
            h_dialog_states: HashMap::new(),
        }
    }   
    // Views
    pub fn push_popup(&mut self, popup: PopupView) -> () {
        self.s_popup.push(popup);
    }
    pub fn peek_popup_ref(&self) -> Option<&PopupView> {
        self.s_popup.last()
    }
    pub fn pop_popup(&mut self) -> Option<PopupView> {
        self.s_popup.pop()
    }
    pub fn push_dialog(&mut self, dialog: DialogView) -> () {
        self.s_dialogs.push(dialog);
    }
    pub fn peek_dialog_ref(&self) -> Option<&DialogView> {
        self.s_dialogs.last()
    }
    pub fn pop_dialog(&mut self) -> Option<DialogView> {
        self.s_dialogs.pop()
    }
    pub fn push_page(&mut self, view: PageView) -> () {
        self.s_pages.push(view);
    }
    pub fn peek_page_ref(&self) -> Option<&PageView> {
        self.s_pages.last()
    }
    pub fn pop_page(&mut self) -> Option<PageView> {
        self.s_pages.pop()
    }
    pub fn push_message(&mut self, level: MessageLevel) -> () {
        match level {
            MessageLevel::INFO => {
                if let Some(message) = self.state.message(MessageLevel::INFO).clone() {
                    self.push_popup(PopupView::Info(message));
                    self.state.reset_message(MessageLevel::INFO);
                }
            },
            MessageLevel::WARNING => {
                if let Some(message) = self.state.message(MessageLevel::WARNING).clone() {
                    self.push_popup(PopupView::Warning(message));
                    self.state.reset_message(MessageLevel::WARNING);
                }
            },
            MessageLevel::ERROR => {
                if let Some(message) = self.state.message(MessageLevel::ERROR).clone() {
                    self.push_popup(PopupView::Error(message));
                    self.state.reset_message(MessageLevel::ERROR);
                }
            }
        }
    }
    //
}