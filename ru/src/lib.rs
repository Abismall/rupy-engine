pub(crate) mod application;
pub(crate) mod camera;
pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod geometry;
pub(crate) mod gpu;
pub(crate) mod input;
pub(crate) mod material;
pub(crate) mod math;
pub(crate) mod menu;
pub(crate) mod render;
pub(crate) mod scene;
pub(crate) mod shader;
pub(crate) mod text;
pub(crate) mod utilities;

#[cfg(feature = "logging")]
pub use crate::utilities::logger as rupyLogger;
//
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

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    };
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {};
}

pub use application::handler::Rupy;
pub use error::AppError;
