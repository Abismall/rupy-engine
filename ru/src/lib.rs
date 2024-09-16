pub(crate) mod application;
pub(crate) mod constants;
pub(crate) mod error;
pub(crate) mod input;
pub(crate) mod rendering;
pub(crate) mod spatial;
pub(crate) mod window;

#[cfg(feature = "logging")]
pub(crate) mod logger;

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

#[cfg(feature = "logging")]
pub use logger::LogFactory;

pub use application::{AppConfig, AppState};
pub use error::AppError;
