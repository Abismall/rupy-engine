use image::ImageError;
use log::SetLoggerError;
use std::io;
use thiserror::Error;
use wgpu::CreateSurfaceError;
use winit::error::{EventLoopError, OsError};
use winit::raw_window_handle::HandleError;

#[derive(Debug, Error)]
pub enum AppError {
    // General application errors
    #[error("Channel error: {0}")]
    ChannelError(String),

    #[error("Window registry error: {0}")]
    WindowRegistryError(String),

    #[error("Thread scope error: {0}")]
    ThreadScopeError(String),

    #[error("State rehydration error: {0}")]
    StateRehydrationError(String),

    #[error("File not found: {0}")]
    FileNotFoundError(String),

    #[error("Event loop error: {0}")]
    EventLoopError(#[from] EventLoopError),

    #[error("OS error: {0}")]
    OsError(#[from] OsError),

    #[error("Raw window handle error: {0}")]
    RawWindowHandleError(#[from] HandleError),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Logger setup error: {0}")]
    LoggerSetupError(#[from] SetLoggerError),

    #[error("Shader source file error: {0}")]
    ShaderSourceFileError(String),

    #[error("Image processing error: {0}")]
    ImageProcessingError(#[from] ImageError),

    // GPU-related errors
    #[error("GPU Initialization error")]
    GpuInitializationError,

    #[error("GPU Adapter not found")]
    GpuAdapterNotFound,

    #[error("GPU Device creation failed: {0}")]
    GpuDeviceCreationError(#[from] wgpu::RequestDeviceError),

    #[error("Surface creation error: {0}")]
    SurfaceCreationError(#[from] CreateSurfaceError),

    #[error("Surface configuration error")]
    SurfaceConfigurationError,

    #[error("Command buffer submission error: {0}")]
    CommandBufferSubmissionError(String),

    // Window-related errors
    #[error("Window creation error: {0}")]
    WindowCreationError(String),

    #[error("Bind group entry error: Bind group entries cannot be empty.")]
    NoBindGroupEntryError,
}
