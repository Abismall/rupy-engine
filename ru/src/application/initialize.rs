use super::rupy::Rupy;
use crate::core::error::AppError;
use crate::events::RupyAppEvent;
use crate::graphics::glyphon::GlyphonRender;
use crate::graphics::gpu::{get_adapter, get_device, get_queue, initialize_gpu_resources_cache};
use crate::log_error;
use crate::math::{mat4_id, mat4_mul};
use pollster::block_on;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::event_loop::ActiveEventLoop;

impl Rupy {
    pub async fn initialize(&mut self) -> Result<(), AppError> {
        block_on(self.init_gpu())?;
        self.init_input();
        let device = get_device()?;
        self.initialize_uniform_buffers(&device)?;

        // resourcer.async_load_shaders().await?;

        // if let Err(e) = self.send_task(RupyWorkerTask::LoadTextures(
        //     TEXTURE_DIR.into(),
        //     "png".into(),
        // )) {
        //     log_error!("Failed to send LoadTextures task: {:?}", e);
        // }
        // if let Err(e) = self.send_task(RupyWorkerTask::ListShaderFiles) {
        //     log_error!("Failed to send ListShaderFiles task: {:?}", e);
        // }
        self.init_text_render(glyphon::Metrics {
            font_size: 150.0,
            line_height: 155.0,
        })?;
        if let Err(e) = self.send_event(RupyAppEvent::CreateWindow) {
            log_error!("Failed to send initialized event: {:?}", e);
            return Err(e);
        };

        Ok(())
    }
    pub fn initialize_uniform_buffers(&mut self, device: &wgpu::Device) -> Result<(), AppError> {
        self.model_matrix = mat4_id();
        self.projection_matrix = self.camera.projection_matrix(&self.camera_perspective);
        self.view_matrix = self.camera.view_matrix();

        let model_view_projection = mat4_mul(
            mat4_mul(self.projection_matrix, self.view_matrix),
            self.model_matrix,
        );

        if self.global_uniform.is_none() {
            let global_uniform_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Global Uniform Buffer"),
                    contents: bytemuck::cast_slice(&self.view_matrix),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
            self.global_uniform = Some(Arc::new(global_uniform_buffer));
        }

        if self.model_uniform.is_none() {
            let model_uniform_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Model Uniform Buffer"),
                    contents: bytemuck::cast_slice(&model_view_projection),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
            self.model_uniform = Some(Arc::new(model_uniform_buffer));
        }

        Ok(())
    }
    pub fn init_input(&mut self) {
        self.input.init_bindings();
    }
    pub fn init_text_render(&mut self, metrics: glyphon::Metrics) -> Result<(), AppError> {
        let device = get_device()?;
        let queue = get_queue()?;
        let format = match &self.window.target {
            Some(surface) => surface.config.format,
            None => wgpu::TextureFormat::Rgba8UnormSrgb,
        };

        let glyphon = GlyphonRender::new(&device, &queue, format, metrics, None);
        self.glyphon = Some(glyphon);

        Ok(())
    }
    pub async fn init_gpu(&mut self) -> Result<(), AppError> {
        if let Err(e) = initialize_gpu_resources_cache().await {
            log_error!("Failed to initialize graphics: {:?}", e);
            return Err(e);
        };
        let device = get_device()?;
        self.device = Some(device);
        Ok(())
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) -> Result<(), AppError> {
        if let Err(e) = self.window.set_window(event_loop) {
            log_error!("Failed to send create window event: {:?}", e);
            return Err(e);
        } else {
            self.window.set_device(
                self.device
                    .as_ref()
                    .expect("Device is required for creating windows")
                    .clone(),
            );
            self.window.create_surface()?;
            self.window.set_visible();
        }

        Ok(())
    }

    pub fn set_device(&mut self) -> Result<(), AppError> {
        let device = get_device()?;
        self.device = Some(device.clone());
        Ok(())
    }
    pub fn set_adapter(&mut self) -> Result<(), AppError> {
        let adapter = get_adapter()?;
        self.adapter = Some(adapter.clone());
        Ok(())
    }
    pub fn set_queue(&mut self) -> Result<(), AppError> {
        let queue = get_queue()?;
        self.queue = Some(queue.clone());
        Ok(())
    }
}
