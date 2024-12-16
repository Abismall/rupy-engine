use cgmath::{Matrix4, SquareMatrix};

use crate::camera::{projection::Projection, Camera};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
    inv_proj: [[f32; 4]; 4],
    inv_view: [[f32; 4]; 4],
}

impl CameraUniform {
    pub const LABEL: &str = "camera_uniform";

    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view: Matrix4::identity().into(),
            view_proj: Matrix4::identity().into(),
            inv_proj: Matrix4::identity().into(),
            inv_view: Matrix4::identity().into(),
        }
    }

    pub fn compute(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();

        let proj = projection.calc_matrix();
        let view = camera.calc_view_matrix();
        let view_proj = proj * view;

        self.view = view.into();
        self.view_proj = view_proj.into();
        self.inv_proj = proj.invert().unwrap().into();
        self.inv_view = view.invert().unwrap().into();
    }
}
