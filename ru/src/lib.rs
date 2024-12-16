pub mod app;
pub mod camera;
pub mod core;
pub mod ecs;
pub mod graphics;
pub mod math;
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
    pub use crate::app::*;
    pub use crate::camera::*;
    pub use crate::core::*;
    pub use crate::math::*;
    pub use crate::utilities::*;
}
