use std::{cell::RefCell, rc::Rc};

use crate::{input::handler::InputListener, log_debug};
use std::fmt::Debug;
use winit::{
    event::{ElementState, RawKeyEvent, WindowEvent},
    keyboard::KeyCode,
};
#[derive(Clone, Copy, Debug)]
pub struct MenuItem<T, L> {
    pub label: L,
    pub action: T,
}

impl<T, L> MenuItem<T, L>
where
    L: Debug,
{
    pub fn new(label: L, action: T) -> Self {
        Self { label, action }
    }
}

#[derive(Debug, Clone)]
pub struct Menu<T, L>
where
    L: Debug,
{
    pub active: bool,
    pub selected: usize,
    pub items: Vec<MenuItem<T, L>>,
}

impl<T, L> Menu<T, L>
where
    T: Debug,
    L: Debug,
{
    pub fn new(items: Vec<MenuItem<T, L>>) -> Self {
        Self {
            active: true,
            selected: 0,
            items,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn render(&self) {
        for (index, item) in self.items.iter().enumerate() {
            if self.selected == index {
                log_debug!("> {:?}. {:?}", index + 1, item.label);
            } else {
                log_debug!("  {}. {:?}", index + 1, item.label);
            }
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> Option<&T> {
        match event {
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                winit::keyboard::PhysicalKey::Code(key_code) => match (event.state, key_code) {
                    (ElementState::Pressed, KeyCode::ArrowUp) => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                    (ElementState::Pressed, KeyCode::ArrowDown) => {
                        if self.selected < self.items.len() - 1 {
                            self.selected += 1;
                        }
                    }
                    (ElementState::Pressed, KeyCode::Enter) => {
                        return Some(&self.items[self.selected].action);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
        None
    }
}

impl<T, L> InputListener for Menu<T, L>
where
    T: Debug,
    L: Debug,
{
    fn on_key_event(&mut self, event: &RawKeyEvent) {
        match event.physical_key {
            winit::keyboard::PhysicalKey::Code(key_code) => {
                log_debug!("Menu received key code: {:?}", key_code);
                match key_code {
                    winit::keyboard::KeyCode::ArrowUp => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                    winit::keyboard::KeyCode::ArrowDown => {
                        if self.selected < self.items.len() - 1 {
                            self.selected += 1;
                        }
                    }
                    winit::keyboard::KeyCode::Enter => {
                        log_debug!("Selected action: {:?}", self.items[self.selected].action);
                    }
                    _ => {}
                }
            }
            winit::keyboard::PhysicalKey::Unidentified(native_key_code) => {
                log_debug!("Menu received native key code: {:?}", native_key_code);
            }
        }
    }

    fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        log_debug!("Menu mouse motion: {:?}", delta);
    }

    fn on_mouse_button(&mut self, button: u32, state: ElementState) {
        log_debug!("Menu mouse button: {} state: {:?}", button, state);
    }
}

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
