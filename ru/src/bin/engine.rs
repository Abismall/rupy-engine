use std::sync::Arc;

use crossbeam::channel::{self, Receiver, Sender};

use rupy::{
    camera::Camera,
    core::{error::AppError, time::Time},
    events::{
        proxy::{EventBusProxy, EventProxy},
        RupyAppEvent,
    },
    input::manager::InputManager,
    math::mat4_id,
    prelude::{
        perspective::{CameraPerspective, CameraPreset},
        rupy::Rupy,
        state::ApplicationStateFlags,
        window::WindowWrapper,
        worker::{RupyTaskWorker, RupyWorkerTask},
    },
    rupyLogger::factory::LogFactory,
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
        rupy::camera::ProjectionType::Perspective,
    );
    let input = InputManager::new(input_tx, camera);
    let window = WindowWrapper::new();

    let time = Time::new();
    let mut rupy = Rupy {
        state: ApplicationStateFlags::empty(),
        #[cfg(feature = "logging")]
        logger: LogFactory::default(),
        debug_mode: rupy::prelude::DebugMode::None,
        camera: Camera::new(None, rupy::camera::ProjectionType::Orthographic),
        camera_perspective: CameraPerspective::from_preset(CameraPreset::Standard),
        event_proxy,
        input,
        event_tx: arc_tx,
        task_tx,
        window,
        time,
        model_matrix: mat4_id(),
        view_matrix: mat4_id(),
        projection_matrix: mat4_id(),
        shaded_material: None,
        textured_material: None,
        sampler: None,

        model_uniform: None,
        global_uniform: None,
        menu: None,
        adapter: None,
        device: None,
        queue: None,
        glyphon: None,
    };

    tokio::spawn(async move {
        event_bus.start().await;
    });

    let _ = event_loop.run_app(&mut rupy);
    Ok(())
}
