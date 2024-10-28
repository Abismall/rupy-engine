use std::collections::HashMap;
use std::sync::Arc;

use crossbeam::channel::Sender;

use crate::events::RupyAppEvent;
use crate::log_debug;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Action {
    ToggleAudio,
    ToggleDebugMode,
    ToggleConsole,
    CloseRequested,
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    VolumeUp,
    VolumeDown,
    Muted,
    Unmuted,
    Exit,
}

pub type ActionCallback = Arc<dyn Fn() + Send + Sync>;
pub struct ActionDispatcher {
    callbacks: HashMap<Action, Vec<ActionCallback>>,
    tx: Arc<Sender<RupyAppEvent>>,
}

impl ActionDispatcher {
    pub fn new(tx: Arc<Sender<RupyAppEvent>>) -> Self {
        ActionDispatcher {
            callbacks: HashMap::new(),
            tx,
        }
    }

    pub fn register_callback<F>(&mut self, action: Action, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.callbacks
            .entry(action)
            .or_insert_with(Vec::new)
            .push(Arc::new(callback));
    }

    pub fn dispatch(&self, action: Action) {
        if let Some(callbacks) = self.callbacks.get(&action) {
            for callback in callbacks {
                callback();
            }
        }

        match action {
            Action::ToggleConsole => {
                if let Err(e) = self.tx.send(RupyAppEvent::ToggleConsole) {
                    log_debug!("Failed to send ToggleConsole event: {:?}", e);
                }
            }
            Action::ToggleDebugMode => {
                if let Err(e) = self.tx.send(RupyAppEvent::ToggleDebugMode) {
                    log_debug!("Failed to send ToggleConsole event: {:?}", e);
                }
            }

            Action::CloseRequested => {
                if let Err(e) = self.tx.send(RupyAppEvent::ExitRequest) {
                    log_debug!("Failed to send ToggleConsole event: {:?}", e);
                }
            }
            _ => {}
        }
    }
}
