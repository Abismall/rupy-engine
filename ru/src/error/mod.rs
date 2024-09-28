use thiserror::Error;
use winit::raw_window_handle::HandleError;

#[derive(Debug, Error)]
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
    #[error("RequestAdapterError: {0}")]
    RequestAdapterError(String),
    #[error("RequestDeviceError: {0}")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
    #[error("CommandBufferSubmissionError: {0}")]
    CommandBufferSubmissionError(String),
}
