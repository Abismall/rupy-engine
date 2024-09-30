use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, RawKeyEvent};

use super::traits::*;
pub struct InputHandler {
    key_listeners: Vec<Box<dyn KeyListener>>,
    raw_key_listeners: Vec<Box<dyn RawKeyListener>>,
    mouse_motion_listeners: Vec<Box<dyn MouseMotionListener>>,
    mouse_button_listeners: Vec<Box<dyn MouseButtonListener>>,
    scroll_listeners: Vec<Box<dyn ScrollListener>>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            key_listeners: Vec::new(),
            raw_key_listeners: Vec::new(),
            mouse_motion_listeners: Vec::new(),
            mouse_button_listeners: Vec::new(),
            scroll_listeners: Vec::new(),
        }
    }

    pub fn add_key_listener(&mut self, listener: Box<dyn KeyListener>) {
        self.key_listeners.push(listener);
    }

    pub fn add_raw_key_listener(&mut self, listener: Box<dyn RawKeyListener>) {
        self.raw_key_listeners.push(listener);
    }

    pub fn add_mouse_motion_listener(&mut self, listener: Box<dyn MouseMotionListener>) {
        self.mouse_motion_listeners.push(listener);
    }

    pub fn add_mouse_button_listener(&mut self, listener: Box<dyn MouseButtonListener>) {
        self.mouse_button_listeners.push(listener);
    }

    pub fn add_scroll_listener(&mut self, listener: Box<dyn ScrollListener>) {
        self.scroll_listeners.push(listener);
    }

    pub fn handle_key_event(&mut self, event: &KeyEvent) {
        for listener in &mut self.key_listeners {
            listener.on_key_event(event);
        }
    }

    pub fn handle_raw_key_event(&mut self, event: &RawKeyEvent) {
        for listener in &mut self.raw_key_listeners {
            listener.on_raw_key_event(event);
        }
    }

    pub fn handle_mouse_motion(&mut self, delta: (f64, f64)) {
        for listener in &mut self.mouse_motion_listeners {
            listener.on_mouse_motion(delta);
        }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        for listener in &mut self.mouse_button_listeners {
            listener.on_mouse_button(button, state);
        }
    }

    pub fn handle_scroll(&mut self, delta: MouseScrollDelta) {
        for listener in &mut self.scroll_listeners {
            listener.on_scroll(delta);
        }
    }
}
