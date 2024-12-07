pub mod controls;
pub mod frustum;
pub mod projection;

use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};
use controls::CameraController;
use projection::Projection;
use winit::dpi::PhysicalSize;

use crate::graphics::model::CameraUniform;
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        )
    }
}
pub struct CameraHandler {
    pub camera: Camera,
    pub projection: Projection,
    pub controller: CameraController,
    pub uniform: CameraUniform,
}

impl CameraHandler {
    pub fn new(
        position: (f32, f32, f32),
        yaw: cgmath::Deg<f32>,
        pitch: cgmath::Deg<f32>,
        size: PhysicalSize<u32>,
    ) -> Self {
        let camera = Camera::new(position, yaw, pitch);
        let projection = Projection::new(size.width, size.height, cgmath::Deg(45.0), 0.1, 100.0);
        let controller = CameraController::new(4.0, 0.8);
        let uniform = CameraUniform::new();

        CameraHandler {
            camera,
            projection,
            controller,
            uniform,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.projection.resize(size.width, size.height);
    }
    pub fn update_view_projection(&mut self) {
        self.uniform.view_proj = (self.projection.calc_matrix() * self.camera.view_matrix()).into();
    }
    pub fn update_view_position(&mut self) {
        self.uniform.view_position = self.camera.position.to_homogeneous().into();
    }
    pub fn update_movement(&mut self, delta_time: f32) {
        self.controller.update(&mut self.camera, delta_time);
    }
    pub fn update(&mut self) {
        self.update_view_position();
        self.update_view_projection();
    }
}
