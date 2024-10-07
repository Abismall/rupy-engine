use crate::{
    log_debug, log_info,
    prelude::{EventBusProxy, RupyAppEvent},
};
use std::sync::{Arc, Mutex};

pub struct Console {
    is_visible: bool,
}

impl Console {
    pub fn new() -> Self {
        Self { is_visible: false }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        log_info!("Hide");
    }

    pub fn show(&mut self) {
        self.is_visible = true;
        log_info!("Show");
    }

    pub fn toggle(&mut self) {
        if self.is_visible {
            self.hide();
        } else {
            self.show();
        }
    }

    pub fn subscribe_to_events(
        console: Arc<Mutex<Console>>,
        event_bus: &mut EventBusProxy<RupyAppEvent>,
    ) {
        event_bus.subscribe("toggle_console", move |event| {
            if let RupyAppEvent::ToggleConsole = event {
                if let Ok(mut console) = console.lock() {
                    console.toggle();
                } else {
                    log_debug!("Failed to lock console instance.");
                }
            }
        });
    }
}
