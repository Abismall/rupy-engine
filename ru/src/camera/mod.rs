pub(crate) mod frustum;
pub(crate) mod perspective;
use perspective::CameraPerspective;
use quaternion::Quaternion;
use vecmath::traits::{One, Zero};
use winit::window::WindowId;

use crate::{
    core::math::clamp,
    prelude::{cross_vec3, dot_vec3, normalize_vec3, rotate_vector, subtract_vec3},
};

use naga::Handle;

use crate::{core::math::Vec3, prelude::Mat4};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera {
    pub projection_matrix: Mat4,
    pub view_matrix: Mat4,
    // pub target_framebuffer: Option<Handle<Framebuffer>>,
    pub position: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub forward: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub id: Option<WindowId>,
}

impl Camera {
    const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
    pub fn new(position: Option<Vec3>) -> Camera {
        let yaw = 0.0;
        let pitch = 0.0;
        let mut camera = Camera {
            position: position.unwrap_or_else(|| [0.0, 0.0, 10.0]),
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            forward: [0.0, 0.0, -1.0],
            yaw,
            pitch,
            id: None,
            projection_matrix: Default::default(),
            view_matrix: Default::default(),
            // target_framebuffer: None,
        };
        camera.update_vectors();
        camera
    }
    pub fn view_matrix(&self) -> Mat4 {
        self.orthogonal()
    }

    pub fn projection_matrix(&self, perspective: &CameraPerspective) -> Mat4 {
        perspective.projection()
    }
    fn update_vectors(&mut self) {
        self.forward = normalize_vec3([
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        ]);

        let world_up = [0.0, 1.0, 0.0];
        self.right = normalize_vec3(cross_vec3(world_up, self.forward));
        self.up = cross_vec3(self.forward, self.right);
    }

    pub fn set_yaw_pitch(&mut self, yaw_offset: f32, pitch_offset: f32) {
        self.yaw += yaw_offset;

        self.pitch = clamp(pitch_offset, -Self::MAX_PITCH, Self::MAX_PITCH);
        self.update_vectors();
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
