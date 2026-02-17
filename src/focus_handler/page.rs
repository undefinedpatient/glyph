use crate::app::page::{GCreatePage, EntrancePage, GSectionNavBar, GEditView, GLayoutEditView, GLayoutOverview, GLayoutView, GNavBar, GPage, GReadView, GViewer, GOpenPage};
use crate::app::Container;
use crate::event_handler::Focusable;

impl Focusable for EntrancePage {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}
impl Focusable for GCreatePage {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(&**container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        for (index, container) in self.containers.iter().enumerate() {
            if container.is_focused() {
                return Some(index);
            }
        }
        None
    }
}
impl Focusable for GOpenPage {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(&**container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        for (index, container) in self.containers.iter().enumerate() {
            if container.is_focused() {
                return Some(index);
            }
        }
        None
    }
}
impl Focusable for GPage {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(&**container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        for (index, container) in self.containers.iter().enumerate() {
            if container.is_focused() {
                return Some(index);
            }
        }
        None
    }
}

/*
    Navigation Bar (Subpage)
 */
impl Focusable for GNavBar {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}
/*
    Glyph Viewers
 */
impl Focusable for GViewer {
    fn is_focused(&self) -> bool {
        self.state.is_focused.borrow().clone()
    }
    fn set_focus(&mut self, value: bool) -> () {
        let mut focus = self.state.is_focused.borrow_mut();
        *focus = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}
impl Focusable for GReadView {
    fn is_focused(&self) -> bool {
        self.state.is_focused.borrow().clone()
    }
    fn set_focus(&mut self, value: bool) -> () {
        let mut focus = self.state.is_focused.borrow_mut();
        *focus = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }

}
impl Focusable for GEditView {

    fn is_focused(&self) -> bool {
        self.state.shared_focus.borrow().clone()
    }
    fn set_focus(&mut self, value: bool) -> () {
        let mut focus = self.state.shared_focus.borrow_mut();
        *focus = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        if self.state.is_editing {
            Some(self.containers[1].as_ref())
        } else {
            Some(self.containers[0].as_ref())
        }
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        if self.state.is_editing {
            Some(self.containers[1].as_mut())
        } else {
            Some(self.containers[0].as_mut())
        }
    }
    fn focused_child_index(&self) -> Option<usize> {
        if self.state.is_editing {
            Some(1)
        } else {
            Some(0)
        }
    }
}

impl Focusable for GSectionNavBar {
    fn is_focused(&self) -> bool {
        false
    }
    fn set_focus(&mut self, value: bool) -> () {}
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}

impl Focusable for GLayoutView {
    fn is_focused(&self) -> bool {
        self.state.shared_focus.borrow().clone()
    }
    fn set_focus(&mut self, value: bool) -> () {
        let mut focus = self.state.shared_focus.borrow_mut();
        *focus = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        if self.state.is_editing {
            Some(self.containers[1].as_ref())
        } else {
            Some(self.containers[0].as_ref())
        }
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        if self.state.is_editing {
            Some(self.containers[1].as_mut())
        } else {
            Some(self.containers[0].as_mut())
        }
    }
    fn focused_child_index(&self) -> Option<usize> {
        if self.state.is_editing {
            Some(1)
        } else {
            Some(0)
        }
    }

}

impl Focusable for GLayoutOverview {
    fn is_focused(&self) -> bool {
        false
    }
    fn set_focus(&mut self, value: bool) -> () {
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}

impl Focusable for GLayoutEditView {
    fn is_focused(&self) -> bool {
        false
    }
    fn set_focus(&mut self, value: bool) -> () {
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(&**container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        for (index, container) in self.containers.iter().enumerate() {
            if container.is_focused() {
                return Some(index);
            }
        }
        None
    }
}