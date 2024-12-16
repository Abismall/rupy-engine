use cgmath::EuclideanSpace;
use winit::dpi::{PhysicalSize, Pixel};

use crate::{
    core::cache::CacheKey,
    ecs::{components::ResourceContext, systems::render::BufferFactory},
    graphics::uniform::Uniforms,
    log_warning,
};

use super::{controller::CameraController, projection::Projection, Camera};

pub struct CameraHandler {
    pub view: Camera,
    pub projection: Projection,
    pub controller: CameraController,
}

impl CameraHandler {
    pub fn new(
        position: (f32, f32, f32),
        yaw: cgmath::Deg<f32>,
        pitch: cgmath::Deg<f32>,
        size: PhysicalSize<u32>,
    ) -> Self {
        let view: Camera = Camera::new(position, yaw, pitch);
        let aspect_ratio = (size.width / size.height).cast::<f64>();
        let projection = Projection::new(aspect_ratio, cgmath::Deg(45.0), 1.0, 100.0);
        let controller = CameraController::default();

        CameraHandler {
            view,
            projection,
            controller,
        }
    }
    pub fn position(&self) -> cgmath::Vector3<f32> {
        self.view.position.to_vec()
    }
    pub fn view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        self.projection.calc_matrix() * self.view.calc_view_matrix()
    }
    pub fn forward(&self) -> cgmath::Vector3<f32> {
        self.view.calculate_vectors().0
    }
    pub fn set_aspect_ratio_from_size<P: winit::dpi::Pixel>(&mut self, size: PhysicalSize<P>) {
        let width: f32 = size.width.cast();
        let height: f32 = size.height.cast();
        self.projection.set_aspect_ratio(width, height);
    }

    pub fn update_buffer(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        uniforms: &mut Uniforms,
        resources: &mut ResourceContext,
    ) {
        let cache_id = CacheKey::from("bg:camera");

        if let Ok(buffer) = resources.buffer_manager.get_or_create_buffer(cache_id, || {
            Ok(BufferFactory::create_camera_uniform_buffer(
                device,
                uniforms.camera,
            ))
        }) {
            uniforms.camera.compute(&self.view, &self.projection);
            let uniform_data = &[uniforms.camera];
            queue.write_buffer(&buffer, 0, bytemuck::cast_slice(uniform_data));
        } else {
            log_warning!("Failed to acquire camera uniform buffer!");
        };
    }
}

pub fn create_camera_handler(size: PhysicalSize<u32>, position: (f32, f32, f32)) -> CameraHandler {
    let yaw = cgmath::Deg(0.0);
    let pitch = cgmath::Deg(0.0);
    CameraHandler::new(position, yaw, pitch, size)
}
