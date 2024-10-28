use crate::prelude::Mat4;

/// Enum representing different types of camera perspectives.
pub enum CameraPreset {
    /// Standard perspective with predefined values
    Standard,
    /// Wide perspective with a wider field of view
    Wide,
    /// Narrow perspective with a narrower field of view
    Narrow,
    /// Custom perspective for custom values
    Custom {
        fov: f32,
        near_clip: f32,
        far_clip: f32,
        aspect_ratio: f32,
        render_distance: f32,
    },
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CameraPerspective {
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub aspect_ratio: f32,
    pub render_distance: f32,
}

impl CameraPerspective {
    pub fn from_preset(preset: CameraPreset) -> Self {
        match preset {
            CameraPreset::Standard => Self {
                fov: 60.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: 16.0 / 9.0,
                render_distance: 1000.0,
            },
            CameraPreset::Wide => Self {
                fov: 90.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: 16.0 / 9.0,
                render_distance: 1000.0,
            },
            CameraPreset::Narrow => Self {
                fov: 45.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: 16.0 / 9.0,
                render_distance: 1000.0,
            },
            CameraPreset::Custom {
                fov,
                near_clip,
                far_clip,
                aspect_ratio,
                render_distance,
            } => Self {
                fov,
                near_clip,
                far_clip,
                aspect_ratio,
                render_distance,
            },
        }
    }

    pub fn perspective_projection(&self) -> Mat4 {
        let fov_radians = self.fov.to_radians();
        let f = 1.0 / (fov_radians / 2.0).tan();
        let near = self.near_clip;
        let far = self.far_clip;
        let nf = 1.0 / (near - far);

        [
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (far + near) * nf, -1.0],
            [0.0, 0.0, (2.0 * far * near) * nf, 0.0],
        ]
    }

    pub fn orthographic_projection(&self) -> Mat4 {
        let left = -self.aspect_ratio;
        let right = self.aspect_ratio;
        let bottom = -1.0;
        let top = 1.0;
        let near = self.near_clip;
        let far = self.far_clip;

        [
            [2.0 / (right - left), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, 2.0 / (near - far), 0.0],
            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                1.0,
            ],
        ]
    }
}
