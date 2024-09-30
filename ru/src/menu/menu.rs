use std::fmt::Debug;
use winit::{
    event::{ElementState, KeyEvent, MouseButton, RawKeyEvent, WindowEvent},
    keyboard::KeyCode,
};

use crate::log_debug;

use super::item::MenuItem;

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
