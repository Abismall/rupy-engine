pub(crate) mod core;

pub(crate) mod graphics;
pub(crate) mod input;

pub(crate) mod math;

pub(crate) mod object;
pub(crate) mod shader;

#[cfg(feature = "logging")]
pub use crate::core::logger as rupyLogger;
use core::application;

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

pub use application::Rupy;
pub use core::error::AppError;
