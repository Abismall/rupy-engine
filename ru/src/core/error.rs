use glyphon::{PrepareError, RenderError};
use image::ImageError;
use log::SetLoggerError;
use std::io;
use thiserror::Error;
use wgpu::{CreateSurfaceError, SurfaceError};
use winit::error::{EventLoopError, OsError};
use winit::raw_window_handle::HandleError;

use crate::events::RupyAppEvent;
use crate::prelude::worker::RupyWorkerTask;

/// Application-wide errors that encompass various subsystems such as IO, rendering, and GPU initialization.
#[derive(Debug, Error)]
pub enum AppError {
    // ====== General Errors ======
    #[error("Failed to acquire lock: {0}")]
    LockError(String),
    #[error("Failed to execute task.")]
    TaskJoinError(#[from] tokio::task::JoinError),
    #[error("Channel error: {0}")]
    ChannelError(String),

    #[error("File not found: {0}")]
    FileNotFoundError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Logger setup error: {0}")]
    LoggerSetupError(#[from] SetLoggerError),

    // ====== Texture and Resources Errors ======
    #[error("No texture provided.")]
    MissingTexture,

    #[error("No sampler provided.")]
    MissingSampler,

    #[error("Unknown shader resource source type: {0}")]
    UnknownShaderResourceTypeError(String),

    #[error("PathBuf conversion error: {0}")]
    PathBufConversionError(String),

    #[error("Shader source file error: {0}")]
    ShaderSourceFileError(String),

    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),

    // ====== GPU Errors ======
    #[error("GPU initialization error.")]
    GpuInitializationError,

    #[error("Failed to initialize the GPU instance.")]
    InstanceInitializationError,

    #[error("Failed to find an appropriate adapter.")]
    AdapterRequestError,

    #[error("Failed to create a device and queue: {0}")]
    DeviceCreationError(String),

    #[error("Failed to acquire lock for GPU resources.")]
    LockAcquisitionError,

    #[error("Failed to send command into queue.")]
    TaskQueueSendError(#[from] crossbeam::channel::SendError<RupyWorkerTask>),

    #[error("Failed to send command into queue.")]
    EventSendError(#[from] crossbeam::channel::SendError<RupyAppEvent>),

    #[error("GPU adapter not found.")]
    GpuAdapterNotFound,

    #[error("GPU device creation failed: {0}")]
    GpuDeviceCreationError(#[from] wgpu::RequestDeviceError),

    #[error("Surface creation error: {0}")]
    CreateSurfaceError(#[from] CreateSurfaceError),

    #[error("Surface error: {0}")]
    SurfaceError(#[from] SurfaceError),

    #[error("Surface configuration error.")]
    SurfaceConfigurationError,

    #[error("Bind group entry error: Bind group entries cannot be empty.")]
    NoBindGroupEntryError,

    // ====== Window and OS Errors ======
    #[error("No active window.")]
    NoActiveWindowError,
    #[error("Window creation error: {0}")]
    WindowCreationError(String),

    #[error("Window registry error: {0}")]
    WindowRegistryError(String),

    #[error("Event loop error: {0}")]
    EventLoopError(#[from] EventLoopError),

    #[error("OS error: {0}")]
    OsError(#[from] OsError),

    #[error("Raw window handle error: {0}")]
    RawWindowHandleError(#[from] HandleError),

    #[error("Thread scope error: {0}")]
    ThreadScopeError(String),

    // ====== Rendering and Pipeline Errors ======
    #[error("Pipeline cache error: {0}")]
    PipelineCacheError(String),

    #[error("Pipeline layout cache error: {0}")]
    PipelineLayoutCacheError(String),

    #[error("Bind group cache error: {0}")]
    BindGroupCacheError(String),

    #[error("No adapter found: {0}")]
    NoAdapterError(String),

    #[error("No device found: {0}")]
    NoDeviceError(String),

    #[error("No queue available: {0}")]
    NoQueueError(String),

    #[error("No surface available: {0}")]
    NoSurfaceError(String),

    #[error("Render error: {0}")]
    RenderError(#[from] RenderError),

    #[error("GlyphonManager prepare error: {0}")]
    GlyphonPrepareError(#[from] PrepareError),
}
