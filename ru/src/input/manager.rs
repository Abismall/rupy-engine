use crossbeam::channel::Sender;
use std::collections::HashMap;
use std::sync::Arc;
use winit::{
    event::{ElementState, MouseButton},
    keyboard::PhysicalKey,
};

use crate::{
    application::event::EventProxyTrait,
    input::{binding::InputBindings, InputEvent},
    prelude::{EventBusProxy, RupyAppEvent},
};

use super::handler::InputListener;

pub struct InputManager {
    listener: InputListener,
    bindings: InputBindings,
    event_bindings: HashMap<PhysicalKey, RupyAppEvent>,
    event_proxy: Option<Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            listener: InputListener::new(),
            bindings: InputBindings::new(),
            event_bindings: HashMap::new(),
            event_proxy: None,
        }
    }

    pub fn subscribe_to_events(&self, event_bus: &mut EventBusProxy<RupyAppEvent>) {

        // event_bus.subscribe("example", move |event| {
        //     if let RupyAppEvent::Example = event {
        //         log_debug!("RupyAppEvent::Example");
        //     }
        // });
    }

    pub fn bind_global_actions(&mut self, tx: Arc<Sender<RupyAppEvent>>) {
        self.bind_key(
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ),
            {
                let tx = tx.clone();
                move || {
                    let _ = tx.send(RupyAppEvent::ToggleAudio);
                }
            },
        );

        self.bind_key(
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F1),
            {
                let tx = tx.clone();
                move || {
                    let _ = tx.send(RupyAppEvent::ToggleConsole);
                }
            },
        );

        self.bind_key(
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
            {
                let tx = tx.clone();
                move || {
                    let _ = tx.send(RupyAppEvent::CloseRequested);
                }
            },
        );
        self.bind_key(
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::NumpadAdd),
            {
                let tx = tx.clone();
                move || {
                    let _ = tx.send(RupyAppEvent::VolumeUp);
                }
            },
        );
        self.bind_key(
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::NumpadSubtract),
            {
                let tx = tx.clone();
                move || {
                    let _ = tx.send(RupyAppEvent::VolumeDown);
                }
            },
        );
    }

    pub fn set_event_proxy(&mut self, proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>) {
        self.event_proxy = Some(proxy);
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        self.listener.handle_event(&event);

        if let InputEvent::Key(key_event) = &event {
            if key_event.state == ElementState::Pressed {
                self.bindings.trigger_key(key_event.physical_key);

                if let Some(event_binding) = self.event_bindings.get(&key_event.physical_key) {
                    if let Some(proxy) = &self.event_proxy {
                        let event_clone = event_binding.clone();
                        proxy.send_event(event_clone).ok();
                    }
                }
            }
        }

        if let InputEvent::MouseButton { button, state } = event {
            if state == ElementState::Pressed {
                self.bindings.trigger_mouse_button(button);
            }
        }
    }

    pub fn bind_mouse_button<F>(&mut self, button: MouseButton, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.bindings.bind_mouse_button(button, action);
    }

    pub fn bind_key<F>(&mut self, key: PhysicalKey, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.bindings.bind_key(key, action);
    }

    pub fn bind_event_to_key(&mut self, key: PhysicalKey, event: RupyAppEvent) {
        self.event_bindings.insert(key, event);
    }

    pub fn is_key_active(&self, key: &PhysicalKey) -> bool {
        self.listener.is_key_active(key)
    }

    pub fn is_mouse_button_active(&self, button: &MouseButton) -> bool {
        self.listener.is_mouse_button_active(button)
    }
}
