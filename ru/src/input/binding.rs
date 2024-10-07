use std::{collections::HashMap, sync::Arc};

use winit::{event::MouseButton, keyboard::PhysicalKey};

use super::handler::ActionCallback;

pub struct InputBindings {
    mouse_bindings: HashMap<MouseButton, ActionCallback>,
    key_bindings: HashMap<PhysicalKey, ActionCallback>,
}

impl InputBindings {
    pub fn new() -> Self {
        InputBindings {
            mouse_bindings: HashMap::new(),
            key_bindings: HashMap::new(),
        }
    }

    pub fn bind_mouse_button<F>(&mut self, button: MouseButton, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.mouse_bindings.insert(button, Arc::new(action));
    }

    pub fn bind_key<F>(&mut self, key: PhysicalKey, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.key_bindings.insert(key, Arc::new(action));
    }

    pub fn trigger_mouse_button(&self, button: MouseButton) {
        if let Some(action) = self.mouse_bindings.get(&button) {
            action();
        }
    }

    pub fn trigger_key(&self, key: PhysicalKey) {
        if let Some(action) = self.key_bindings.get(&key) {
            action();
        }
    }

    pub fn unbind_mouse_button(&mut self, button: MouseButton) {
        self.mouse_bindings.remove(&button);
    }

    pub fn unbind_key(&mut self, key: PhysicalKey) {
        self.key_bindings.remove(&key);
    }
}
