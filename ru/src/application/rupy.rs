use crossbeam::channel::Sender;
use glyphon::{Resolution, TextBounds};
use wgpu::{util::DeviceExt, IndexFormat, RenderPass};

use super::{
    state::ApplicationStateFlags, surface::SurfaceWrapper, window::WindowWrapper,
    worker::RupyWorkerTask, DebugMode,
};
#[cfg(feature = "logging")]
use crate::rupyLogger;
use crate::{
    camera::{perspective::CameraPerspective, Camera},
    core::{error::AppError, time::Time},
    events::RupyAppEvent,
    graphics::glyphon::{get_text_bounds, GlyphonRender},
    input::manager::InputManager,
    log_error, log_info,
    scene::{
        components::{mesh::Mesh, uniform::CameraUniforms, vertex::Vertex},
        material::{shaded::ShadedMaterial, textured::TexturedMaterial, Material},
    },
    traits::{bus::EventProxyTrait, rendering::Renderable},
    ui::menu::Menu,
};
use std::sync::Arc;

pub struct Rupy {
    pub state: ApplicationStateFlags,
    pub event_proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>,
    pub event_tx: Arc<Sender<RupyAppEvent>>,
    pub task_tx: Sender<RupyWorkerTask>,
    pub input: InputManager,
    #[cfg(feature = "logging")]
    pub logger: rupyLogger::factory::LogFactory,
    pub debug_mode: DebugMode,
    pub camera: Camera,
    pub camera_perspective: CameraPerspective,
    pub glyphon: Option<GlyphonRender>,
    pub shaded_material: Option<ShadedMaterial>,
    pub textured_material: Option<TexturedMaterial>,
    pub sampler: Option<wgpu::Sampler>,

    pub model_uniform: Option<Arc<wgpu::Buffer>>,
    pub global_uniform: Option<Arc<wgpu::Buffer>>,
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub model_matrix: [[f32; 4]; 4],
    pub time: Time,
    pub menu: Option<Menu>,
    pub adapter: Option<Arc<wgpu::Adapter>>,
    pub device: Option<Arc<wgpu::Device>>,
    pub queue: Option<Arc<wgpu::Queue>>,
    pub window: WindowWrapper,
}

impl Rupy {
    pub fn exit_process(&mut self, grace_period_secs: u32) {
        if !self.state.contains(ApplicationStateFlags::SHUTTING_DOWN) {
            self.state.set_shutting_down();
        };

        std::thread::spawn(move || {
            if grace_period_secs > 0 {
                log_info!("Shutdown in {}", grace_period_secs);
            }
            std::thread::sleep(std::time::Duration::from_secs(grace_period_secs.into()));
            std::process::exit(0);
        });
    }

    pub fn send_event(&self, event: RupyAppEvent) -> std::result::Result<(), AppError> {
        self.event_tx.send(event).map_err(AppError::EventSendError)
    }
    pub fn send_task(&self, task: RupyWorkerTask) -> Result<(), AppError> {
        self.task_tx
            .send(task)
            .map_err(AppError::TaskQueueSendError)
    }
    pub fn ensure_device(&mut self) -> Result<(), AppError> {
        if self.device.is_none() {
            if let Err(e) = self.set_device() {
                panic!("Could not ensure device: {:?}", e);
            }
        }
        Ok(())
    }
    pub fn ensure_queue(&mut self) -> Result<(), AppError> {
        if self.queue.is_none() {
            if let Err(e) = self.set_queue() {
                panic!("Could not ensure queue: {:?}", e);
            }
        }
        Ok(())
    }
    pub fn try_get_surface(&self) -> Result<&SurfaceWrapper, AppError> {
        self.window
            .target
            .as_ref()
            .ok_or_else(|| AppError::SurfaceInitializationError)
    }
    pub fn get_view(
        frame: &wgpu::SurfaceTexture,
        descriptor: Option<wgpu::TextureViewDescriptor>,
    ) -> Result<wgpu::TextureView, AppError> {
        Ok(frame
            .texture
            .create_view(&descriptor.unwrap_or(wgpu::TextureViewDescriptor::default())))
    }

    pub fn try_get_frame(surface: &SurfaceWrapper) -> Result<wgpu::SurfaceTexture, AppError> {
        if let Ok(frame) = surface.current.get_current_texture() {
            Ok(frame)
        } else {
            Err(AppError::FrameAcquisitionError)
        }
    }
}

impl Rupy {
    pub fn update(&mut self) -> Result<(), AppError> {
        self.time.update();

        if self.debug_mode != DebugMode::None {
            if let (Some(glyphon), Some(size)) = (&mut self.glyphon, self.window.size()) {
                let resolution = Resolution {
                    width: size.width,
                    height: size.height,
                };
                let bounds = get_text_bounds(resolution, Some(150), Some(440));
                Self::prepare_text(
                    self.device.as_ref().ok_or(AppError::NoDeviceError(
                        "Failed to prepare text, no device set!".into(),
                    ))?,
                    self.queue.as_ref().ok_or(AppError::NoQueueError(
                        "Failed to prepare text, no queue set!".into(),
                    ))?,
                    ("PRISMA").to_string(),
                    resolution,
                    bounds,
                    glyphon,
                    self.window.scale_factor(),
                );
            };
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), AppError> {
        self.ensure_device()?;
        self.ensure_queue()?;

        let device = self.device.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();
        let surface = self.try_get_surface()?;
        let frame = Self::try_get_frame(surface)?;
        let view = Self::get_view(&frame, Some(wgpu::TextureViewDescriptor::default()))?;
        let scale = 0.8;

        let magenta_triangle_vertices = [
            Vertex {
                position: [0.0 * scale, 0.5 * scale, 0.0],
                color: [1.0, 0.0, 0.5, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5 * scale, 0.0 * scale, 0.0],
                color: [1.0, 0.0, 0.5, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5 * scale, 0.0 * scale, 0.0],
                color: [1.0, 0.0, 0.5, 1.0],
                uv: [0.0, 0.0],
            },
        ];

        let orange_triangle_vertices = [
            Vertex {
                position: [-0.5 * scale, 0.0 * scale, 0.0],
                color: [1.0, 0.5, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0 * scale, -0.5 * scale, 0.0],
                color: [1.0, 0.5, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.0 * scale, -0.5 * scale, 0.0],
                color: [1.0, 0.5, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
        ];

        let yellow_triangle_vertices = [
            Vertex {
                position: [0.5 * scale, 0.0 * scale, 0.0],
                color: [1.0, 1.0, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.0 * scale, -0.5 * scale, 0.0],
                color: [1.0, 1.0, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [1.0 * scale, -0.5 * scale, 0.0],
                color: [1.0, 1.0, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
        ];
        let magenta_mesh = Mesh::new(device, &magenta_triangle_vertices, &[0, 1, 2]);
        let orange_mesh = Mesh::new(device, &orange_triangle_vertices, &[0, 1, 2]);
        let yellow_mesh = Mesh::new(device, &yellow_triangle_vertices, &[0, 1, 2]);

        let camera_uniforms = CameraUniforms::default();
        let model_uniform_buffer = Arc::new(device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Global Uniform Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniforms]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        ));

        self.shaded_material = Some(ShadedMaterial::new(device, model_uniform_buffer, None)?);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.5,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: Default::default(),
                occlusion_query_set: Default::default(),
            });
            self.render_debug_info(&mut render_pass);
            if let Some(material) = &self.shaded_material {
                render_pass.set_pipeline(&material.pipeline);
                render_pass.set_bind_group(0, material.bind_group(), &[]);

                render_pass.set_vertex_buffer(0, magenta_mesh.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(magenta_mesh.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..magenta_mesh.index_count(), 0, 0..1);

                render_pass.set_vertex_buffer(0, orange_mesh.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(orange_mesh.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..orange_mesh.index_count(), 0, 0..1);

                render_pass.set_vertex_buffer(0, yellow_mesh.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(yellow_mesh.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..yellow_mesh.index_count(), 0, 0..1);
            }
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    fn render_debug_info<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        if self.debug_mode == DebugMode::None {
            return;
        } else if let Some(glyphon) = &self.glyphon {
            if let Err(e) = glyphon.render(render_pass) {
                log_error!("Failed to render debug info: {:?}", e);
            }
        }
    }

    pub(crate) fn prepare_text(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        text: String,
        resolution: Resolution,
        bounds: TextBounds,
        glyphon: &mut GlyphonRender,
        scale_factor: f64,
    ) {
        glyphon.reconfigure(queue, resolution);
        glyphon.set_buffer_size((resolution.width as f32, resolution.height as f32));
        glyphon.set_buffer_text(&text);
        glyphon
            .prepare_text(&device, &queue, bounds, scale_factor as f32)
            .ok();
        glyphon.shape_until_scroll(false);
    }
}
