pub mod proxy;
use winit::event::{Modifiers, MouseButton};

use crate::texture::{config::TextureConfig, file::TextureFile};

#[derive(Debug)]
pub enum RupyAppEvent {
    Shutdown,
    TaskCompleted(crate::prelude::WorkerTaskCompletion),
    Initialized,
    ToggleConsole,
    ToggleDebugMode,
    ToggleLaunchMenu,
    CreateSurface,
    CreateWindow,
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
#[derive(Debug)]
pub enum WorkerTaskCompletion {
    LoadTextureFiles(Vec<TextureConfig>),
    LoadShaderFiles(Vec<String>),
}
