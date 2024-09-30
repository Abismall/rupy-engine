use std::sync::{Arc, Mutex};

use crate::camera::entity::Camera;
use crate::config::screen::ScreenConfig;
use crate::config::settings::Settings;
use crate::gpu::GPUGlobal;
use crate::object::buffer::BufferManager;
use crate::pipeline::manager::PipelineManager;
use crate::render::surface::TargetSurface;
use crate::scene::manager::SceneManager;
use crate::utilities::debug::DebugMetrics;
use crate::AppError;
use winit::{event_loop::ActiveEventLoop, window::WindowId};

fn on_exit(window_id: WindowId, camera: &mut Camera, el: &ActiveEventLoop) {
    camera
        .id
        .is_some_and(|locked| locked.eq(&window_id))
        .then(|| camera.unlock());
    el.exit();
}

pub struct ApplicationState {
    scene_manager: Arc<SceneManager>,
    buffer_manager: Arc<BufferManager>,
    pipeline_manager: Arc<PipelineManager>,
    camera: Camera,
    screen_config: ScreenConfig,
    settings: Settings,
    debug_metrics: DebugMetrics,
    gpu: Arc<Mutex<GPUGlobal>>,
    target: TargetSurface,
}

impl ApplicationState {
    pub fn camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub async fn build(
        target: TargetSurface,
        instance_desc: Option<wgpu::InstanceDescriptor>,
    ) -> Result<ApplicationState, AppError> {
        let gpu = Arc::new(Mutex::new(GPUGlobal::initialize(instance_desc).await?));
        let (
            scene_manager,
            buffer_manager,
            pipeline_manager,
            camera,
            screen_config,
            settings,
            debug_metrics,
        ) = build();

        Ok(ApplicationState {
            scene_manager,
            buffer_manager,
            pipeline_manager,
            camera,
            screen_config,
            settings,
            debug_metrics,
            gpu,
            target,
        })
    }
}

fn build() -> (
    Arc<SceneManager>,
    Arc<BufferManager>,
    Arc<PipelineManager>,
    Camera,
    ScreenConfig,
    Settings,
    DebugMetrics,
) {
    let camera = Camera::new([0.0, 0.0, 0.0]);
    let screen_config = ScreenConfig::new();
    let settings = Settings::default();
    let debug = DebugMetrics::new();
    let scenes = Arc::new(SceneManager::new());
    let buffers = Arc::new(BufferManager::new());
    let pipelines = Arc::new(PipelineManager::new());

    (
        scenes,
        buffers,
        pipelines,
        camera,
        screen_config,
        settings,
        debug,
    )
}
