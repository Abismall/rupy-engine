use winit::event::{
    DeviceEvent, ElementState, KeyEvent, MouseButton, MouseScrollDelta, RawKeyEvent,
};

use super::InputListener;
pub struct InputHandler {
    listeners: Vec<Box<dyn InputListener>>,
}

impl InputHandler {
    fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    fn add_listener(&mut self, listener: Box<dyn InputListener>) {
        self.listeners.push(listener);
    }

    fn handle_input(&mut self, event: &KeyEvent) {
        for listener in self.listeners.iter_mut() {
            listener.on_key_event(event);
        }
    }

    fn handle_raw_input(&mut self, event: &RawKeyEvent) {
        for listener in self.listeners.iter_mut() {
            listener.on_raw_key_event(event);
        }
    }

    fn handle_mouse_motion(&mut self, delta: (f64, f64)) {
        for listener in self.listeners.iter_mut() {
            listener.on_mouse_motion(delta);
        }
    }

    fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        for listener in self.listeners.iter_mut() {
            listener.on_mouse_button(button.into(), state);
        }
    }

    fn handle_mouse_scroll(&mut self, delta: MouseScrollDelta) {
        for listener in self.listeners.iter_mut() {
            listener.on_scroll(delta);
        }
    }

    /// Method to process a generic DeviceEvent, now includes scroll handling
    fn process_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::Key(key_event) => {
                self.handle_raw_input(key_event);
            }
            DeviceEvent::MouseMotion { delta: mouse_delta } => {
                self.handle_mouse_motion(*mouse_delta);
            }
            DeviceEvent::Button { button, state } => {
                if let Some(mouse_button) = map_button_to_mouse(*button) {
                    self.handle_mouse_button(mouse_button, *state);
                }
            }
            DeviceEvent::MouseWheel {
                delta: scroll_delta,
            } => {
                self.handle_mouse_scroll(*scroll_delta);
            }
            _ => {}
        }
    }
}

/// Helper function to map a raw button code to a MouseButton
fn map_button_to_mouse(button: u32) -> Option<MouseButton> {
    match button {
        1 => Some(MouseButton::Left),
        2 => Some(MouseButton::Right),
        3 => Some(MouseButton::Middle),
        _ => None,
    }
}
