pub mod application;
pub mod core;
pub mod events;
pub mod graphics;
pub mod math;
pub mod model;
pub mod scene;
pub mod system;
pub mod traits;
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
    pub use crate::core::*;
    pub use crate::graphics::texture::texture_cache::TextureCache;
    pub use crate::math::*;
    pub use crate::model::*;
    pub use crate::system::camera::frustum::*;
    pub use crate::system::camera::perspective::*;
    pub use crate::system::*;
    pub use crate::traits::*;
}
