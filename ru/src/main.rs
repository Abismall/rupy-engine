use std::sync::Arc;

use crossbeam::channel::{self, Receiver, Sender};

use rupy::{
    app::state::AppState,
    core::{error::AppError, worker::RupyWorker},
    events::{
        proxy::{EventBusProxy, EventProxy, EventProxyTrait},
        RupyAppEvent,
    },
    prelude::{rupy::Rupy, worker::WorkerTask},
    rupyLogger::factory::LogFactory,
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
    let (task_tx, task_rx): (Sender<WorkerTask>, Receiver<WorkerTask>) =
        crossbeam::channel::unbounded();

    RupyWorker::spawn(task_rx, tx.clone());

    let arc_tx = Arc::new(tx);

    let event_loop = EventLoop::<RupyAppEvent>::with_user_event().build()?;
    let event_loop_proxy = Arc::new(event_loop.create_proxy());
    let event_proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync> =
        Arc::new(EventProxy::new(event_loop_proxy));

    let event_bus_rx = rx.clone();
    let event_bus_proxy = event_proxy.clone();
    let event_bus = EventBusProxy::new(event_bus_rx, event_bus_proxy);
    let app_state = AppState::empty();
    let mut rupy = Rupy {
        #[cfg(feature = "logging")]
        logger: LogFactory::default(),
        event_proxy,
        event_tx: arc_tx,
        task_tx,
        state: app_state,
        debug: rupy::app::DebugMode::None,
        render_context: None,
    };

    tokio::spawn(async move {
        event_bus.start().await;
    });

    let _ = event_loop.run_app(&mut rupy);
    Ok(())
}
