use std::collections::HashMap;
use std::sync::Arc;
use winit::{
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, RawKeyEvent},
    keyboard::PhysicalKey,
};

use super::InputEvent;

pub type ActionCallback = Arc<dyn Fn() + Send + Sync>;

#[derive(Debug)]
pub struct InputListener {
    active_keys: HashMap<PhysicalKey, ElementState>, // Tracks currently active (pressed) keys
    active_mouse_buttons: HashMap<MouseButton, ElementState>, // Tracks currently pressed mouse buttons
    mouse_position: (f64, f64),                               // Current position of the mouse
    scroll_delta: (f32, f32), // Cumulative scroll delta (tracks scroll wheel movement)
}

impl InputListener {
    pub fn new() -> Self {
        InputListener {
            active_keys: HashMap::new(),
            active_mouse_buttons: HashMap::new(),
            mouse_position: (0.0, 0.0), // Mouse starts at (0, 0)
            scroll_delta: (0.0, 0.0),   // No scroll delta initially
        }
    }

    pub fn handle_event(&mut self, event: &InputEvent) {
        match event {
            InputEvent::Key(key_event) => self.key_event(key_event),
            InputEvent::RawKey(raw_key_event) => self.raw_key_event(raw_key_event),
            InputEvent::MouseMotion { delta } => self.handle_mousemotion(delta),
            InputEvent::MouseButton { button, state } => self.handle_mousebutton(button, state),
            InputEvent::Scroll(mouse_scroll_delta) => self.handle_scroll(mouse_scroll_delta),
        }
    }

    fn key_event(&mut self, key_event: &KeyEvent) {
        self.update_active_keys(key_event.physical_key, key_event.state);
    }

    fn raw_key_event(&mut self, raw_key_event: &RawKeyEvent) {
        self.update_active_keys(raw_key_event.physical_key, raw_key_event.state);
    }

    fn update_active_keys(&mut self, key: PhysicalKey, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.active_keys.insert(key, state);
            }
            ElementState::Released => {
                self.active_keys.remove(&key);
            }
        }
    }

    fn handle_mousemotion(&mut self, delta: &(f64, f64)) {
        self.mouse_position.0 += delta.0;
        self.mouse_position.1 += delta.1;
    }

    fn handle_mousebutton(&mut self, button: &MouseButton, state: &ElementState) {
        match state {
            ElementState::Pressed => {
                self.active_mouse_buttons.insert(*button, *state);
            }
            ElementState::Released => {
                self.active_mouse_buttons.remove(&button);
            }
        }
    }

    fn handle_scroll(&mut self, mouse_scroll_delta: &MouseScrollDelta) {
        match mouse_scroll_delta {
            MouseScrollDelta::LineDelta(x, y) => {
                self.scroll_delta.0 += x;
                self.scroll_delta.1 += y;
            }
            MouseScrollDelta::PixelDelta(pos) => {
                self.scroll_delta.0 += pos.x as f32;
                self.scroll_delta.1 += pos.y as f32;
            }
        }
    }

    pub fn is_key_active(&self, key: &PhysicalKey) -> bool {
        self.active_keys.get(key) == Some(&ElementState::Pressed)
    }

    pub fn is_mouse_button_active(&self, button: &MouseButton) -> bool {
        self.active_mouse_buttons.get(button) == Some(&ElementState::Pressed)
    }
}
