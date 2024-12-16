pub mod proxy;
use std::sync::Arc;

use winit::event::{Modifiers, MouseButton};

#[derive(Debug, Clone)]
pub enum RupyAppEvent {
    Shutdown,
    TaskCompleted,
    ToggleConsole,
    ToggleDebugMode,
    ToggleLaunchMenu,
    CreateSurface,
    RenderStart(Arc<winit::window::Window>),
    CreateWindow,
    WindowCreated(Arc<winit::window::Window>),
    ToggleAudio,
    AudioMuteOn,
    AudioMuteOff,
    VolumeUp,
    VolumeDown,

    KeyPressed {
        key_code: u32,
        modifiers: Modifiers, // Control, Shift, Alt, etc.
    },
    KeyReleased {
        key_code: u32,
        modifiers: Modifiers,
    },
    MouseMoved {
        position: (f32, f32), // (x, y)
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
        id: u64,
        position: (f32, f32),
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

impl RupyAppEvent {
    pub fn name(&self) -> &str {
        match self {
            RupyAppEvent::Shutdown => "Shutdown",
            RupyAppEvent::TaskCompleted => "TaskCompleted",
            RupyAppEvent::ToggleConsole => "ToggleConsole",
            RupyAppEvent::ToggleDebugMode => "ToggleDebugMode",
            RupyAppEvent::ToggleLaunchMenu => "ToggleLaunchMenu",
            RupyAppEvent::CreateSurface => "CreateSurface",
            RupyAppEvent::RenderStart(_) => "RenderStart",
            RupyAppEvent::CreateWindow => "CreateWindow",
            RupyAppEvent::WindowCreated(_) => "WindowCreated",
            RupyAppEvent::ToggleAudio => "ToggleAudio",
            RupyAppEvent::AudioMuteOn => "AudioMuteOn",
            RupyAppEvent::AudioMuteOff => "AudioMuteOff",
            RupyAppEvent::VolumeUp => "VolumeUp",
            RupyAppEvent::VolumeDown => "VolumeDown",
            RupyAppEvent::KeyPressed { .. } => "KeyPressed",
            RupyAppEvent::KeyReleased { .. } => "KeyReleased",
            RupyAppEvent::MouseMoved { .. } => "MouseMoved",
            RupyAppEvent::MouseButtonPressed { .. } => "MouseButtonPressed",
            RupyAppEvent::MouseButtonReleased { .. } => "MouseButtonReleased",
            RupyAppEvent::MouseWheel { .. } => "MouseWheel",
            RupyAppEvent::TouchStart { .. } => "TouchStart",
            RupyAppEvent::TouchMove { .. } => "TouchMove",
            RupyAppEvent::TouchEnd { .. } => "TouchEnd",
            RupyAppEvent::WindowResized { .. } => "WindowResized",
            RupyAppEvent::WindowClosed => "WindowClosed",
            RupyAppEvent::WindowFocused { .. } => "WindowFocused",
            RupyAppEvent::WindowMoved { .. } => "WindowMoved",
            RupyAppEvent::WindowMinimized => "WindowMinimized",
            RupyAppEvent::WindowRestored => "WindowRestored",
            RupyAppEvent::AppStarted => "AppStarted",
            RupyAppEvent::AppSuspended => "AppSuspended",
            RupyAppEvent::AppResumed => "AppResumed",
            RupyAppEvent::AppClosed => "AppClosed",
            RupyAppEvent::SceneLoaded { .. } => "SceneLoaded",
            RupyAppEvent::SceneUnloaded { .. } => "SceneUnloaded",
            RupyAppEvent::ObjectSpawned { .. } => "ObjectSpawned",
            RupyAppEvent::ObjectDestroyed { .. } => "ObjectDestroyed",
            RupyAppEvent::InputCommand { .. } => "InputCommand",
            RupyAppEvent::FrameRendered { .. } => "FrameRendered",
            RupyAppEvent::RenderError { .. } => "RenderError",
            RupyAppEvent::AudioStarted { .. } => "AudioStarted",
            RupyAppEvent::AudioStopped { .. } => "AudioStopped",
            RupyAppEvent::AudioError { .. } => "AudioError",
        }
    }
}
