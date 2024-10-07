use std::sync::Arc;

use log::Level;
use winit::event_loop::EventLoopProxy;

#[derive(Clone, Debug)]
pub enum RupyAppEvent {
    OpenConsole,
    CloseConsole,
    ToggleConsole,

    ToggleAudio,
    AudioMuteOn,
    AudioMuteOff,
    VolumeUp,
    VolumeDown,

    CloseRequested,
    KeyPressed {
        key_code: u32,
        modifiers: Modifiers, // Control, Shift, Alt, etc.
    },
    KeyReleased {
        key_code: u32,
        modifiers: Modifiers,
    },
    MouseMoved {
        position: (f32, f32), // (x, y) coordinates of the cursor
    },
    MouseButtonPressed {
        button: MouseButton, // Left, Right, Middle, etc.
        position: (f32, f32),
    },
    MouseButtonReleased {
        button: MouseButton,
        position: (f32, f32),
    },
    MouseWheel {
        delta: f32, // Scroll amount
        position: (f32, f32),
    },
    TouchStart {
        id: u64,              // Identifier for the touch point
        position: (f32, f32), // Start position of the touch
    },
    TouchMove {
        id: u64,
        position: (f32, f32),
    },
    TouchEnd {
        id: u64,
        position: (f32, f32),
    },

    WindowResized {
        width: u32,
        height: u32,
    },
    WindowClosed,
    WindowFocused {
        focused: bool,
    },
    WindowMoved {
        position: (i32, i32),
    },
    WindowMinimized,
    WindowRestored,

    AppStarted,
    AppSuspended,
    AppResumed,
    AppClosed,

    SceneLoaded {
        scene_name: String,
    },
    SceneUnloaded {
        scene_name: String,
    },
    ObjectSpawned {
        object_id: u64,
        object_type: String,
    },
    ObjectDestroyed {
        object_id: u64,
        object_type: String,
    },

    InputCommand {
        command: String,
    },

    FrameRendered {
        frame_time: f64,
    },
    RenderError {
        message: String,
    },

    AudioStarted {
        sound_id: u64,
    },
    AudioStopped {
        sound_id: u64,
    },
    AudioError {
        sound_id: u64,
        message: String,
    },
}
#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8), // (e.g., forward/backward buttons on a mouse)
}

#[derive(Debug, Clone)]
pub enum Modifiers {
    Shift,
    Control,
    Alt,
    Meta, // MacOS "Command" key
    None,
}

pub trait EventProxyTrait<T: 'static + std::fmt::Debug> {
    fn send_event(&self, event: T) -> Result<(), winit::event_loop::EventLoopClosed<T>>;
}

pub struct EventProxy<T: 'static + std::fmt::Debug> {
    event_loop_proxy: Arc<EventLoopProxy<T>>,
}

impl<T: 'static + std::fmt::Debug> EventProxy<T> {
    pub fn new(event_loop_proxy: Arc<EventLoopProxy<T>>) -> Self {
        Self { event_loop_proxy }
    }
}

impl<T: 'static + std::fmt::Debug> EventProxyTrait<T> for EventProxy<T> {
    fn send_event(&self, event: T) -> Result<(), winit::event_loop::EventLoopClosed<T>> {
        self.event_loop_proxy.send_event(event)
    }
}
