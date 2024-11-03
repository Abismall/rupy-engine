use std::sync::Arc;

use crossbeam::channel::Sender;

use winit::{event_loop::ActiveEventLoop, window::WindowAttributes};

#[cfg(feature = "logging")]
use crate::rupyLogger;
use crate::{
    core::{error::AppError, worker::WorkerTask},
    events::RupyAppEvent,
    graphics::{global::initialize_instance, RenderMode},
    log_debug, log_error, log_info,
    traits::bus::EventProxyTrait,
};

use super::{context::RenderContext, state::AppState, DebugMode};

pub struct Rupy {
    pub event_proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>,
    pub event_tx: Arc<Sender<RupyAppEvent>>,
    pub task_tx: Sender<WorkerTask>,
    #[cfg(feature = "logging")]
    pub logger: rupyLogger::factory::LogFactory,
    pub debug: DebugMode,
    pub state: AppState,
    pub render_context: Option<RenderContext>,
}

impl Rupy {
    pub fn send_event(&self, event: RupyAppEvent) -> std::result::Result<(), AppError> {
        self.event_tx.send(event).map_err(AppError::EventSendError)
    }
    pub fn send_task(&self, task: WorkerTask) -> Result<(), AppError> {
        self.task_tx
            .send(task)
            .map_err(AppError::TaskQueueSendError)
    }
}

impl Rupy {
    pub fn shutdown(&mut self, event_loop: &ActiveEventLoop) {
        if event_loop.exiting() {
            return;
        } else {
            log_info!("Exit");
            event_loop.exit();
        };
    }
    pub async fn initialize(&mut self) -> Result<(), AppError> {
        if let Err(e) = self.setup_gpu_resources_cache().await {
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
        match event_loop.create_window(WindowAttributes::default()) {
            Ok(win) => Ok(win),
            Err(e) => Err(AppError::from(e)),
        }
    }
    pub async fn setup_gpu_resources_cache(&mut self) -> Result<(), AppError> {
        if let Err(e) = initialize_instance().await {
            log_error!("Failed to initialize graphics: {:?}", e);
            return Err(e);
        };
        Ok(())
    }
    pub fn toggle_debug_mode(&mut self) {
        if let Some(ctx) = &mut self.render_context {
            self.debug.toggle();
            ctx.debug = self.debug;
            log_info!("{:?}", ctx.debug);
        };
    }

    pub fn toggle_render_mode(&mut self) {
        if let Some(ctx) = &mut self.render_context {
            ctx.mode = match ctx.mode {
                RenderMode::Depth => RenderMode::WireWithDepth,
                RenderMode::WireWithDepth => RenderMode::Flat,
                RenderMode::Flat => RenderMode::WireNoDepth,
                RenderMode::WireNoDepth => RenderMode::Depth,
            };
            log_debug!("{:?}", ctx.mode);
            match ctx.mode {
                RenderMode::Depth | RenderMode::WireWithDepth => {
                    ctx.enable_depth_stencil();
                }
                RenderMode::Flat | RenderMode::WireNoDepth => {
                    ctx.disable_depth_stencil();
                }
            }
        }
    }
}
