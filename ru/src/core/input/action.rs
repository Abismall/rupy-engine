use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Action {
    ToggleRenderMode,
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
    Run,
    Exit,
}

pub type ActionCallback = Arc<dyn Fn() + Send + Sync>;
pub struct ActionDispatcher {
    callbacks: HashMap<Action, Vec<ActionCallback>>,
}

impl ActionDispatcher {
    pub fn new() -> Self {
        ActionDispatcher {
            callbacks: HashMap::new(),
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
    }
}
