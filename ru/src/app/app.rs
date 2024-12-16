use crate::{
    core::{
        error::AppError,
        events::{proxy::EventProxyTrait, RupyAppEvent},
        worker::WorkerTask,
    },
    graphics::global::initialize_instance,
    log_error,
    prelude::helpers::read_window_attributes_from_env,
};
use std::sync::Arc;

use super::state::State;
use crossbeam::channel::Sender;

use winit::{event_loop::ActiveEventLoop, window::WindowAttributes};

#[cfg(feature = "logging")]
use crate::rupyLogger;

pub struct Rupy {
    pub event_proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>,
    pub event_tx: Arc<Sender<RupyAppEvent>>,
    pub task_tx: Sender<WorkerTask>,
    #[cfg(feature = "logging")]
    pub logger: rupyLogger::factory::LogFactory,

    pub state: Option<State>,
}

impl Rupy {
    pub fn send_event(&self, event: RupyAppEvent) {
        let event_name = event.name().to_string();
        if let Err(e) = self.event_tx.send(event) {
            log_error!("Rupy::send_event: {:?} {:?}", event_name, e);
        }
    }
    pub fn send_task(&self, task: WorkerTask) -> std::result::Result<(), AppError> {
        Ok(self.task_tx.send(task)?)
    }
}

impl Rupy {
    pub async fn initialize(&mut self) -> Result<(), AppError> {
        if let Err(e) = initialize_instance().await {
            log_error!("Failed to setup gpu resources: {:?}", e);
            return Err(e.into());
        }

        self.send_event(RupyAppEvent::CreateWindow);
        Ok(())
    }
    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) -> Result<winit::window::Window, AppError> {
        let (width, height, x, y) = read_window_attributes_from_env();
        match event_loop.create_window(
            WindowAttributes::default()
                .with_title("RupyEngine")
                .with_decorations(true)
                .with_visible(true)
                .with_theme(Some(winit::window::Theme::Dark))
                .with_inner_size(winit::dpi::LogicalSize::new(width, height))
                .with_position(winit::dpi::LogicalPosition::new(x, y)),
        ) {
            Ok(win) => Ok(win),
            Err(e) => Err(AppError::from(e)),
        }
    }

    pub async fn setup_instance(&mut self) -> Result<(), AppError> {
        if let Err(e) = initialize_instance().await {
            log_error!("Failed to initialize graphics: {:?}", e);
            return Err(e);
        };
        Ok(())
    }
    pub async fn initialize_state(
        &mut self,
        window: std::sync::Arc<winit::window::Window>,
    ) -> bool {
        let gpu = pollster::block_on(crate::graphics::context::GpuResourceCache::new());
        let bit_flags = super::flags::BitFlags::empty();
        self.state = match State::new(gpu, bit_flags, window).await {
            Ok(state) => Some(state),
            Err(e) => {
                log_error!("{:?}", e);
                None
            }
        };
        self.state.is_some()
    }
}
