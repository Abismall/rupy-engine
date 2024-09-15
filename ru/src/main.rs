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

    let evt_loop = winit::event_loop::EventLoop::new().map_err(|e| AppError::from(e))?;

    let config = AppConfig::default();
    let mut state = AppState::Setup { config };

    evt_loop.run_app(&mut state).unwrap();

    Ok(())
}
