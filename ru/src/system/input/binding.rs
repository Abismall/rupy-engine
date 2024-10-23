use std::collections::HashMap;

use winit::{event::MouseButton, keyboard::PhysicalKey};

use super::action::Action;

pub struct InputBindings {
    mouse_bindings: HashMap<MouseButton, Action>,
    key_bindings: HashMap<PhysicalKey, Action>,
}

impl InputBindings {
    pub fn new() -> Self {
        InputBindings {
            mouse_bindings: HashMap::new(),
            key_bindings: HashMap::new(),
        }
    }
    pub fn bind_mouse_button(&mut self, button: MouseButton, action: Action) {
        self.mouse_bindings.insert(button, action);
    }

    pub fn bind_key(&mut self, key: PhysicalKey, action: Action) {
        self.key_bindings.insert(key, action);
    }

    pub fn get_action_for_mouse_button(&self, button: &MouseButton) -> Option<&Action> {
        self.mouse_bindings.get(button)
    }

    pub fn get_action_for_key(&self, key: &PhysicalKey) -> Option<&Action> {
        self.key_bindings.get(key)
    }

    pub fn unbind_mouse_button(&mut self, button: MouseButton) {
        self.mouse_bindings.remove(&button);
    }

    pub fn unbind_key(&mut self, key: PhysicalKey) {
        self.key_bindings.remove(&key);
    }
}
