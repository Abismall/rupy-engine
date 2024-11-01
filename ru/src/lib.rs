pub mod application;
pub mod camera;
pub mod core;
pub mod events;
pub mod graphics;
pub mod input;
pub mod math;
pub mod scene;
pub mod traits;
pub mod ui;
pub mod utilities;

#[cfg(feature = "logging")]
pub use core::logging as rupyLogger;

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
    pub use crate::application::*;
    pub use crate::camera::*;
    pub use crate::core::*;
    pub use crate::events::*;
    pub use crate::input::*;
    pub use crate::math::*;
    pub use crate::scene::*;
    pub use crate::traits::*;
    pub use crate::ui::*;
    pub use crate::utilities::*;
}
