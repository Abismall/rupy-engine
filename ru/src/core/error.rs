use glyphon::{PrepareError, RenderError};
use image::ImageError;
use log::SetLoggerError;
use std::env::VarError;
use std::io;

use thiserror::Error;
use wgpu::{CreateSurfaceError, SurfaceError};
use winit::error::{EventLoopError, OsError};
use winit::raw_window_handle::HandleError;

use crate::core::worker::WorkerTask;
use crate::events::RupyAppEvent;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Material type error: {0}")]
    MaterialTypeError(String),

    #[error("Toml error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Serde yaml error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Pipeline not found error: {0}")]
    PipelineNotFoundError(String),

    #[error("Failed to parse shader source: {0}")]
    ShaderParseError(#[from] naga::front::wgsl::ParseError),

    #[error("Var error: {0}")]
    VarError(#[from] VarError),

    #[error("Failed to execute task.")]
    TaskJoinError(#[from] tokio::task::JoinError),

    #[error("Channel error: {0}")]
    ChannelError(String),

    #[error("Failed to load texture file: {0}")]
    TextureFileLoadError(String),

    #[error("Surface was lost or has not been initialized")]
    SurfaceInitializationError,

    #[error("File not found: {0}")]
    FileNotFoundError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Logger setup error: {0}")]
    LoggerSetupError(#[from] SetLoggerError),

    #[error("No surface available")]
    NoSurfaceAvailable,

    #[error("No texture provided.")]
    MissingTexture,

    #[error("Image format is not supported: {0}")]
    UnsupportedImageFormat(String),

    #[error("No sampler provided.")]
    MissingSampler,

    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),

    #[error("Failed to initialize the GPU instance.")]
    InstanceInitializationError,

    #[error("Failed to find an appropriate adapter.")]
    AdapterRequestError,

    #[error("Failed to acquire RwLock: {0}")]
    LockAcquisitionFailure(String),

    #[error("Failed to send command into queue.")]
    TaskQueueSendError(#[from] crossbeam::channel::SendError<WorkerTask>),

    #[error("Failed to send command into queue.")]
    EventSendError(#[from] crossbeam::channel::SendError<RupyAppEvent>),

    #[error("GPU device creation failed: {0}")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),

    #[error("Surface creation error: {0}")]
    CreateSurfaceError(#[from] CreateSurfaceError),

    #[error("Surface error: {0}")]
    SurfaceError(#[from] SurfaceError),

    #[error("Surface configuration error.")]
    SurfaceConfigurationError,

    #[error("Bind group entry error: Bind group entries cannot be empty.")]
    NoBindGroupEntryError,

    #[error("No active window.")]
    NoActiveWindowError,

    #[error("Event loop error: {0}")]
    EventLoopError(#[from] EventLoopError),

    #[error("OS error: {0}")]
    OsError(#[from] OsError),

    #[error("Raw window handle error: {0}")]
    RawWindowHandleError(#[from] HandleError),

    #[error("Render error: {0}")]
    RenderError(#[from] RenderError),

    #[error("GlyphonManager prepare error: {0}")]
    GlyphonPrepareError(#[from] PrepareError),
}
