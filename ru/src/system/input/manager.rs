use crossbeam::channel::Sender;
use std::{collections::HashMap, sync::Arc};
use winit::{event::MouseButton, keyboard::PhysicalKey};

use crate::{events::RupyAppEvent, system::camera::Camera};

use super::{
    action::{Action, ActionDispatcher},
    binding::InputBindings,
    InputContext, InputEvent, KeyInputEventType, MouseInputEventType,
};

pub struct InputManager {
    key_states: HashMap<PhysicalKey, bool>,
    mouse_button_states: HashMap<MouseButton, bool>,
    bindings: HashMap<InputContext, InputBindings>,
    dispatcher: ActionDispatcher,
    current_context: InputContext,
    pub last_cursor_position: Option<(f32, f32)>,
    pub camera: Camera,
}

impl InputManager {
    pub fn new(tx: Arc<Sender<RupyAppEvent>>, camera: Camera) -> Self {
        let dispatcher = ActionDispatcher::new(tx.clone());

        let mut bindings = HashMap::new();
        bindings.insert(InputContext::NoSurface, InputBindings::new());
        bindings.insert(InputContext::Window, InputBindings::new());

        InputManager {
            key_states: HashMap::new(),
            mouse_button_states: HashMap::new(),
            bindings,
            dispatcher,
            current_context: InputContext::NoSurface,
            camera,
            last_cursor_position: None,
        }
    }

    pub fn init_bindings(&mut self) {
        self.bind_key(
            PhysicalKey::Code(winit::keyboard::KeyCode::F1),
            Action::ToggleConsole,
        );
        self.bind_key(
            PhysicalKey::Code(winit::keyboard::KeyCode::F2),
            Action::ToggleDebugMode,
        );
        self.bind_key(
            PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
            Action::CloseRequested,
        );

        // Add movement bindings for camera
        self.bind_key(
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyW),
            Action::MoveForward,
        );
        self.bind_key(
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyS),
            Action::MoveBackward,
        );
        self.bind_key(
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyA),
            Action::MoveLeft,
        );
        self.bind_key(
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyD),
            Action::MoveRight,
        );
    }

    pub fn bind_key(&mut self, key: PhysicalKey, action: Action) {
        if let Some(bindings) = self.bindings.get_mut(&self.current_context) {
            bindings.bind_key(key, action);
        }
    }
    /// Set the current input context
    pub fn set_context(&mut self, context: InputContext) {
        self.current_context = context;
    }

    /// Register a callback for a specific action
    pub fn register_callback<F>(&mut self, action: Action, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.dispatcher.register_callback(action, callback);
    }

    pub fn process_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::KeyInput(ref key_event) => self.process_key_event(key_event),
            InputEvent::MouseInput(ref mouse_event) => self.process_mouse_event(mouse_event),
            _ => {}
        }
    }

    fn process_key_event(&mut self, key_event: &KeyInputEventType) {
        match key_event {
            KeyInputEventType::Key(key_evt) => {
                self.key_states.insert(
                    key_evt.physical_key,
                    key_evt.state == winit::event::ElementState::Pressed,
                );
                if key_evt.state == winit::event::ElementState::Pressed {
                    if let Some(action) = self
                        .bindings
                        .get(&self.current_context)
                        .and_then(|b| b.get_action_for_key(&key_evt.physical_key))
                    {
                        self.dispatcher.dispatch(*action);
                        self.handle_camera_movement(*action);
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_camera_movement(&mut self, action: Action) {
        match action {
            Action::MoveForward => self.camera.move_forward(1.0), // Adjust speed as needed
            Action::MoveBackward => self.camera.move_forward(-1.0),
            Action::MoveLeft => self.camera.move_right(-1.0),
            Action::MoveRight => self.camera.move_right(1.0),
            _ => {}
        }
    }

    fn process_mouse_event(&mut self, mouse_event: &MouseInputEventType) {
        match mouse_event {
            MouseInputEventType::Button(button_evt) => {
                self.mouse_button_states.insert(
                    button_evt.button,
                    button_evt.state == winit::event::ElementState::Pressed,
                );
            }
            MouseInputEventType::Motion(delta) => {
                let sensitivity = self.camera.sensitivity;
                let (pos_x, pos_y) = (delta.0 as f32, delta.1 as f32);
                if let Some((last_x, last_y)) = self.last_cursor_position {
                    let delta_x = pos_x - last_x;
                    let delta_y = pos_y - last_y;
                    self.camera
                        .rotate(delta_x * sensitivity, delta_y * sensitivity);
                }
                self.last_cursor_position = Some((pos_x, pos_y));
            }
            MouseInputEventType::CursorLeft => {
                self.last_cursor_position = None;
            }
            _ => {}
        }
    }
}
