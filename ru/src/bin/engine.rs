use std::sync::Arc;

use crossbeam::channel::{self, Receiver, Sender};

use rupy::{
    core::{error::AppError, time::Time},
    events::{
        proxy::{EventBusProxy, EventProxy},
        RupyAppEvent,
    },
    log_error,
    model::window::WindowWrapper,
    prelude::{
        resources::ResourceManager,
        rupy::Rupy,
        state::AppState,
        worker::{RupyTaskWorker, RupyWorkerTask},
    },
    rupyLogger::factory::LogFactory,
    system::{camera::Camera, input::manager::InputManager},
    traits::bus::EventProxyTrait,
};
use winit::event_loop::EventLoop;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    #[cfg(feature = "logging")]
    {
        let logger = LogFactory::default();
        let _ = logger.init();
    }

    let (tx, rx): (Sender<RupyAppEvent>, Receiver<RupyAppEvent>) = channel::unbounded();
    let (task_tx, task_rx): (Sender<RupyWorkerTask>, Receiver<RupyWorkerTask>) =
        crossbeam::channel::unbounded();

    RupyTaskWorker::spawn(task_rx, tx.clone());

    let arc_tx = Arc::new(tx);

    let event_loop = EventLoop::<RupyAppEvent>::with_user_event().build()?;
    let event_loop_proxy = Arc::new(event_loop.create_proxy());
    let event_proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync> =
        Arc::new(EventProxy::new(event_loop_proxy));

    let event_bus_rx = rx.clone();
    let event_bus_proxy = event_proxy.clone();
    let event_bus = EventBusProxy::new(event_bus_rx, event_bus_proxy);

    let input_tx = arc_tx.clone();

    let camera = Camera::new(
        Some([0.0, 0.0, 5.0]),
        rupy::system::camera::ProjectionType::Perspective,
    );
    let input = InputManager::new(input_tx, camera);
    let window = WindowWrapper::new();

    let time = Time::new();
    let mut rupy = Rupy {
        state: AppState::empty(),
        resources: ResourceManager::new(),
        #[cfg(feature = "logging")]
        logger: LogFactory::default(),
        event_proxy,
        input,
        event_tx: arc_tx,
        task_tx,
        window,
        time,
        device: None,
        queue: None,
        renderer: None,
        glyphon: None,
        surface: None,
    };

    tokio::spawn(async move {
        event_bus.start().await;
    });

    let _ = event_loop.run_app(&mut rupy);
    Ok(())
}
