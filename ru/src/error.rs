use thiserror::Error;
use winit::raw_window_handle::HandleError;
// Define custom error types
#[derive(Debug, Error)]
#[error("Failed to create pipeline")]
pub struct PipelineCreationError;
#[derive(Debug, Error)]
#[error("Failed to create window")]
pub struct WindowCreationError;
#[derive(Debug, Error)]
#[error("Failed to load shader")]
pub struct ShaderLoadError;
#[derive(Debug, Error)]
#[error("Shader not initialized")]
pub struct DeviceNotInitializedError;
#[derive(Debug, Error)]
#[error("Device not initialized")]
pub struct DeviceRequestError;
#[derive(Debug, Error)]
#[error("Device request failed")]

pub enum AppError {
    #[error("WinitEventLoopError: {0}")]
    WinitEventLoopError(#[from] winit::error::EventLoopError),

    #[error("RawWindowHandleError: {0}")]
    RawWindowHandleError(#[from] HandleError),

    #[error("WinitOsError: {0}")]
    OsError(#[from] winit::error::OsError),

    #[error("CreateSurfaceError: {0}")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),

    #[error("SetLoggerError: {0}")]
    SetLoggerError(#[from] log::SetLoggerError),

    #[error("Device not initialized: {0}")]
    DeviceNotInitialized(#[from] DeviceNotInitializedError),

    #[error("Pipeline creation error: {0}")]
    PipelineCreation(#[from] PipelineCreationError),

    #[error("Shader load error: {0}")]
    ShaderLoadError(#[from] std::io::Error),
}
