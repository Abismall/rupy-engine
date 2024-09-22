pub(crate) mod app;
pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod geometry;
pub(crate) mod graphics;
pub(crate) mod input;
pub(crate) mod math;
pub(crate) mod menu;
pub(crate) mod shader;
pub(crate) mod utilities;
use app::state;
//
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

pub use app::rupy::Rupy;
pub use error::AppError;
