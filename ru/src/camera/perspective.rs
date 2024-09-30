use vecmath::traits::{Cast, One, Radians, Zero};

use crate::math::Mat4;

use super::entity::Camera;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CameraPerspective {
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub aspect_ratio: f32,
}
impl CameraPerspective {
    pub fn projection(&self) -> Mat4 {
        let _0: f32 = Zero::zero();
        let _1: f32 = One::one();
        let _2: f32 = _1 + _1;
        let pi: f32 = Radians::_180();
        let _360: f32 = Cast::cast(360.0f64);
        let f = _1 / (self.fov * (pi / _360)).tan();
        let (far, near) = (self.far_clip, self.near_clip);
        [
            [f / self.aspect_ratio, _0, _0, _0],
            [_0, f, _0, _0],
            [_0, _0, (far + near) / (near - far), -_1],
            [_0, _0, (_2 * far * near) / (near - far), _0],
        ]
    }
}
impl Camera {
    pub fn view_matrix(&self) -> Mat4 {
        self.orthogonal()
    }

    pub fn projection_matrix(&self, perspective: &CameraPerspective) -> Mat4 {
        perspective.projection()
    }
}
