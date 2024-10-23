use super::rupy::Rupy;
use crate::core::error::AppError;
use crate::events::RupyAppEvent;
use crate::graphics::gpu::{get_device, get_queue, initialize_gpu_resources_cache};
use crate::scene::systems::render::RenderSystem;
use crate::system::glyphon::GlyphonRender;
use crate::{log_error, log_info};

impl Rupy {
    pub async fn initialize(&mut self) -> Result<(), AppError> {
        initialize_gpu_resources_cache().await?;
        self.device = Some(get_device()?);
        self.queue = Some(get_queue()?);

        self.input.init_bindings();
        self.renderer = Some(RenderSystem::new());
        self.initialize_glyphon().await?;
        log_info!("Init completed");

        if let Err(e) = self.send_event(RupyAppEvent::Initialized) {
            log_error!("Failed to send initialized event: {:?}", e);
            return Err(e);
        };
        Ok(())
    }

    pub async fn initialize_glyphon(&mut self) -> Result<(), AppError> {
        let device = &get_device()?;
        let queue = &get_queue()?;
        let glyphon_render = GlyphonRender::new(
            device,
            queue,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            glyphon::Metrics::new(24.0, 72.0),
            None,
        );
        self.glyphon = Some(glyphon_render);
        Ok(())
    }
}
