use vecmath::traits::{Cast, One, Radians, Zero};

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

    /// Generates the projection matrix for the camera perspective
    pub fn projection(&self) -> Mat4 {
        let _0: f32 = Zero::zero();
        let _1: f32 = One::one();
        let _2: f32 = _1 + _1;
        let pi: f32 = Radians::_180();
        let _360: f32 = Cast::cast(360.0f64);

        // Calculate the f value for the projection matrix
        let f = _1 / (self.fov * (pi / _360)).tan();
        let (far, near) = (self.far_clip, self.near_clip);

        // Return the perspective projection matrix
        [
            [f / self.aspect_ratio, _0, _0, _0],
            [_0, f, _0, _0],
            [_0, _0, (far + near) / (near - far), -_1],
            [_0, _0, (_2 * far * near) / (near - far), _0],
        ]
    }
}
