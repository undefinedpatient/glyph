use std::rc::Rc;
use crate::app::Focusable;

pub struct FocusHandler {
    index: Option<usize>,
    is_focused: bool,
    focuses: Vec<Rc<dyn Focusable>>,
}
