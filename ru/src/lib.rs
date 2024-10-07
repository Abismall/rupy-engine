pub mod application;
pub mod audio;
pub mod camera;
pub mod console;
pub mod core;
pub mod ecs;
pub mod gpu;
pub mod input;
pub mod pipeline;
pub mod render;
pub mod scene;
pub mod shader;
pub mod ui;
pub mod utilities;

#[cfg(feature = "logging")]
pub use core::log as rupyLogger;

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*);
    };
}
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    };
}
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    };
}
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        log::warn!($($arg)*);
    };
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {};
}

pub mod prelude {
    pub use crate::application::bus::EventBusProxy;
    pub use crate::application::event::EventProxy;
    pub use crate::application::event::RupyAppEvent;
    pub use crate::application::rupy;
    pub use crate::audio::Audio;
    pub use crate::camera::perspective::CameraPerspective;
    pub use crate::camera::Camera;
    pub use crate::console::Console;
    pub use crate::core::error::AppError;
    pub use crate::core::math::spatial::*;
    pub use crate::core::math::trigonometry::*;
    pub use crate::core::math::vector::*;
    pub use crate::input::binding::InputBindings;
    pub use crate::input::manager::InputManager;
    pub use crate::scene::object;
    pub use crate::scene::scene;
    pub use crate::scene::texture;
    pub use crate::shader::reflection;
    pub use crate::shader::source;
}
