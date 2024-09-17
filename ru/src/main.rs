extern crate rupy;

#[cfg(feature = "logging")]
use rupy::LogFactory;
use rupy::{AppConfig, AppError, AppState};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), AppError> {
    #[cfg(feature = "logging")]
    {
        LogFactory::default().init()?;
    }
    winit::event_loop::EventLoop::new()?.run_app(&mut AppState::Uninitialized {
        config: AppConfig::default(),
    })?;

    Ok(())
}
