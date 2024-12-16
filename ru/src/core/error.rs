use super::{events::RupyAppEvent, worker::WorkerTask};
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // ECS-related errors
    #[error("ComponentDowncastError: {0}")]
    WorldQueryError(String),
    // Resource-related errors
    #[error("Resource creation failed: {0}")]
    ResourceCreationFailed(String),
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    #[error("Resource already exists")]
    DuplicateResource,
    #[error("Dimension mismatch")]
    DimensionMismatch,

    // Scene-related errors
    #[error("Scene error: {0}")]
    CreateSceneError(String),
    #[error("Scene not found: {0}")]
    SceneNotFoundError(String),

    // Material-related errors
    #[error("Material type error: {0}")]
    MaterialTypeError(String),

    // Configuration-related errors
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error("Toml error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Serde yaml error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    // Shader-related errors
    #[error("Failed to parse shader source: {0}")]
    ShaderParseError(#[from] naga::front::wgsl::ParseError),

    // I/O and file-related errors
    #[error("io::Error: {0}")]
    IoError(#[from] io::Error),
    #[error("FileNotFoundError: {0}")]
    FileNotFoundError(String),
    #[error("image::ImageError: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("walkdir::Error: {0}")]
    WalkDirError(#[from] walkdir::Error),
    #[error("tobj::LoadError {0}")]
    TobjLoadError(#[from] tobj::LoadError),

    // Logging and system-related errors
    #[error("Logger setup error: {0}")]
    LoggerSetupError(#[from] log::SetLoggerError),
    #[error("OS error: {0}")]
    OsError(#[from] winit::error::OsError),
    #[error("crossbeam::channel::SendError<RupyAppEvent>: {0}")]
    CrossBeamChannelSendEventError(#[from] crossbeam::channel::SendError<RupyAppEvent>),
    #[error("crossbeam::channel::SendError<WorkerTask>: {0}")]
    CrossBeamChannelSendTaskError(#[from] crossbeam::channel::SendError<WorkerTask>),
    #[error("EventLoopError: {0}")]
    EventLoopError(#[from] winit::error::EventLoopError),
    // GPU and rendering errors
    #[error("wgpu::CreateSurfaceError: {0}")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),
    #[error("wgpu::SurfaceError: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),
    #[error("wgpu::RequestDeviceError: {0}")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
    #[error("GPUResourceError: {0}")]
    GPUResourceError(String),

    #[error("RenderError {0}")]
    RenderError(#[from] glyphon::RenderError),

    // Task-related errors
    #[error("TaskJoinError: {0}")]
    TaskJoinError(#[from] tokio::task::JoinError),

    // Misc
    #[error("Failed to acquire RwLock: {0}")]
    LockAcquisitionFailure(String),
    #[error("Var error: {0}")]
    VarError(#[from] std::env::VarError),
}
