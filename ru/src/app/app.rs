use crate::{
    core::{error::AppError, worker::WorkerTask},
    events::{proxy::EventProxyTrait, RupyAppEvent},
    graphics::global::initialize_instance,
    log_error, log_info,
    prelude::helpers::read_window_attributes_from_env,
};
use std::sync::Arc;

use super::{flags::BitFlags, state::State, DebugMode};
use crossbeam::channel::Sender;

use winit::{event_loop::ActiveEventLoop, window::WindowAttributes};

#[cfg(feature = "logging")]
use crate::rupyLogger;

pub struct Rupy<'a> {
    pub event_proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>,
    pub event_tx: Arc<Sender<RupyAppEvent>>,
    pub task_tx: Sender<WorkerTask>,
    #[cfg(feature = "logging")]
    pub logger: rupyLogger::factory::LogFactory,
    pub debug: DebugMode,
    pub bit_flags: BitFlags,

    pub state: Option<State<'a>>,
}

impl<'a> Rupy<'a> {
    pub fn send_event(&self, event: RupyAppEvent) -> std::result::Result<(), AppError> {
        self.event_tx.send(event).map_err(AppError::EventSendError)
    }
    pub fn send_task(&self, task: WorkerTask) -> Result<(), AppError> {
        self.task_tx
            .send(task)
            .map_err(AppError::TaskQueueSendError)
    }
    pub fn update(&mut self) {
        if self.bit_flags.is_running() {
            if let Some(state) = &mut self.state {
                state.renderer.ctx.update();
            };
        }
    }
}

impl<'a> Rupy<'a> {
    pub fn shutdown(&mut self, event_loop: &ActiveEventLoop) {
        if event_loop.exiting() {
            return;
        } else {
            log_info!("Exit");
            event_loop.exit();
        };
    }

    pub async fn initialize(&mut self) -> Result<(), AppError> {
        if let Err(e) = initialize_instance().await {
            log_error!("Failed to setup gpu resources: {:?}", e);
            return Err(e.into());
        }

        if let Err(e) = self.send_event(RupyAppEvent::CreateWindow) {
            log_error!("Failed to send initialized event: {:?}", e);
            return Err(e);
        }

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
}
