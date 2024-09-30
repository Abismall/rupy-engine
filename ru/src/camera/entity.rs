use quaternion::Quaternion;
use vecmath::traits::{One, Zero};
use winit::window::WindowId;

use crate::{
    log_debug,
    math::{
        cross_vec3, dot_vec3, normalize_vec3, subtract_vec3, vector::rotate_vector, Mat4, Vec3,
    },
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera {
    pub position: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub forward: Vec3,
    pub id: Option<WindowId>,
}

impl Camera {
    pub fn new(position: Vec3) -> Camera {
        let _0 = Zero::zero();
        let _1 = One::one();
        Camera {
            position,
            right: [_1, _0, _0],
            up: [_0, _1, _0],
            forward: [_0, _0, _1],
            id: None,
        }
    }

    pub fn lock(&mut self, id: WindowId) {
        if self.id.is_none() {
            self.id = Some(id);
            log_debug!("[LOCK] {:?}", id);
        }
    }
    pub fn unlock(&mut self) {
        if let Some(id) = self.id {
            self.id = None;
            log_debug!("[UNLOCK] {:?}", id);
        }
    }
    pub fn orthogonal(&self) -> Mat4 {
        let p = self.position;
        let r = self.right;
        let u = self.up;
        let f = self.forward;
        let _0 = Zero::zero();
        [
            [r[0], u[0], f[0], _0],
            [r[1], u[1], f[1], _0],
            [r[2], u[2], f[2], _0],
            [
                -dot_vec3(r, p),
                -dot_vec3(u, p),
                -dot_vec3(f, p),
                One::one(),
            ],
        ]
    }

    pub fn look_at(&mut self, point: Vec3) {
        self.forward = normalize_vec3(subtract_vec3(point, self.position));
        self.update_right();
    }

    pub fn set_yaw_pitch(&mut self, yaw: f32, pitch: f32) {
        let (y_s, y_c, p_s, p_c) = (yaw.sin(), yaw.cos(), pitch.sin(), pitch.cos());
        self.forward = [y_s * p_c, p_s, y_c * p_c];
        self.up = [y_s * -p_s, p_c, y_c * -p_s];
        self.update_right();
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        let _0: f32 = Zero::zero();
        let _1: f32 = One::one();
        let forward: Vec3 = [_0, _0, _1];
        let up: Vec3 = [_0, _1, _0];
        self.forward = rotate_vector(rotation, forward);
        self.up = rotate_vector(rotation, up);
        self.update_right();
    }

    fn update_right(&mut self) {
        self.right = cross_vec3(self.up, self.forward);
    }
}
