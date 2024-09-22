use winit::event::{ElementState, RawKeyEvent};

use crate::input::handler::InputListener;

use super::menu::Menu;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub struct MenuWrapper<T, L>
where
    L: Debug,
{
    pub menu: Rc<RefCell<Menu<T, L>>>,
}

impl<T, L> MenuWrapper<T, L>
where
    L: Debug,
{
    pub fn new(menu: Rc<RefCell<Menu<T, L>>>) -> Self {
        MenuWrapper { menu }
    }
}
impl<T, L> InputListener for MenuWrapper<T, L>
where
    T: Debug,
    L: Debug,
{
    fn on_key_event(&mut self, event: &RawKeyEvent) {
        let mut menu = self.menu.borrow_mut();
        menu.on_key_event(event);
    }

    fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        let mut menu = self.menu.borrow_mut();
        menu.on_mouse_motion(delta);
    }

    fn on_mouse_button(&mut self, button: u32, state: ElementState) {
        let mut menu = self.menu.borrow_mut();
        menu.on_mouse_button(button, state);
    }
}
