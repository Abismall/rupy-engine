use nalgebra::Matrix4;

use crate::prelude::constant::ZERO_F32;

#[derive(Debug, Clone, Copy)]
pub enum ProjectionMode {
    Orthographic(f32, f32, f32, f32, f32, f32),
    Perspective(f32, f32, f32, f32),
}

#[derive(Debug, Clone)]
pub struct CameraProjection {
    pub mode: ProjectionMode,
    pub aspect_ratio: f32,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraProjection {
    pub fn new_perspective(aspect_ratio: f32, fov_y: f32, near: f32, far: f32) -> Self {
        Self {
            mode: ProjectionMode::Perspective(fov_y, aspect_ratio, near, far),
            aspect_ratio,
            fov_y,
            near,
            far,
        }
    }

    pub fn new_orthographic(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            mode: ProjectionMode::Orthographic(left, right, bottom, top, near, far),
            aspect_ratio: (right - left) / (top - bottom),
            fov_y: std::f32::consts::FRAC_PI_4,
            near,
            far,
        }
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        match self.mode {
            ProjectionMode::Orthographic(left, right, bottom, top, near, far) => {
                Matrix4::new_orthographic(left, right, bottom, top, near, far)
            }
            ProjectionMode::Perspective(fov_y, aspect_ratio, near, far) => {
                Matrix4::new_perspective(aspect_ratio, fov_y, near, far)
            }
        }
    }

    pub fn set_projection_mode(&mut self, mode: ProjectionMode) {
        self.mode = mode;
        match mode {
            ProjectionMode::Orthographic(_, _, _, _, near, far) => {
                self.near = near;
                self.far = far;
            }
            ProjectionMode::Perspective(_, aspect_ratio, near, far) => {
                self.aspect_ratio = aspect_ratio;
                self.near = near;
                self.far = far;
            }
        }
    }
}
