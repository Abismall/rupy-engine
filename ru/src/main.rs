extern crate rupy;

#[cfg(feature = "logging")]
use rupy::rupyLogger::LogFactory;

use rupy::{AppError, Rupy};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), AppError> {
    #[cfg(feature = "logging")]
    {
        let _ = LogFactory::default().init();
    }

    let evt_loop = winit::event_loop::EventLoop::new()?;
    let mut app = Rupy::new();

    let _ = evt_loop.run_app(&mut app);
    Ok(())
}
