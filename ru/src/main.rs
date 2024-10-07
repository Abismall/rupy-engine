extern crate rupy;

use crossbeam::channel::{self, Receiver, Sender};
use rupy::{
    application::event::EventProxyTrait,
    prelude::{
        rupy::Rupy, AppError, Audio, Console, EventBusProxy, EventProxy, InputManager, RupyAppEvent,
    },
};
use std::sync::{Arc, Mutex};
use winit::event_loop::EventLoop;

#[cfg(feature = "logging")]
use rupy::rupyLogger::LogFactory;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), AppError> {
    #[cfg(feature = "logging")]
    {
        let logger = LogFactory::default();
        let _ = logger.init();
    }

    let (sys_tx, sys_rx): (Sender<RupyAppEvent>, Receiver<RupyAppEvent>) = channel::unbounded();

    let el = EventLoop::<RupyAppEvent>::with_user_event().build()?;
    let create_proxy = Arc::new(el.create_proxy());

    let proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync> =
        Arc::new(EventProxy::new(create_proxy));

    let message_bus_tx = sys_rx.clone();
    let bus_proxy = Arc::clone(&proxy);

    let mut message_bus = EventBusProxy::new(message_bus_tx, bus_proxy);
    let mut inputs = InputManager::new();
    inputs.set_event_proxy(Arc::clone(&proxy));
    inputs.bind_global_actions(Arc::new(sys_tx.clone()));
    let app_tx = sys_tx.clone();
    let app_proxy = Arc::clone(&proxy);

    let audio = Arc::new(Mutex::new(Audio::new()));
    let console = Arc::new(Mutex::new(Console::new()));
    let mut app = Rupy::new(Arc::new(app_tx), inputs, app_proxy);

    Console::subscribe_to_events(Arc::clone(&console), &mut message_bus);
    Audio::subscribe_to_events(Arc::clone(&audio), &mut message_bus);

    tokio::spawn(async move {
        message_bus.process_events().await;
    });

    let _ = el.run_app(&mut app).unwrap();
    drop(audio);
    drop(proxy);
    drop(app);
    Ok(())
}
