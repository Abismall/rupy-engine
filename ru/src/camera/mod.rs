pub mod frustum;
pub mod perspective;
use perspective::CameraPerspective;
use quaternion::Quaternion;
use vecmath::traits::{One, Zero};
use winit::window::WindowId;

use crate::{
    math::{vector::rotate_vector, Mat4, Vec3},
    prelude::{cross_vec3, dot_vec3, normalize_vec3, subtract_vec3},
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ProjectionType {
    Perspective,
    Orthographic,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera {
    pub projection_type: ProjectionType,
    pub position: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub forward: Vec3,
    pub yaw: f32,
    pub sensitivity: f32,
    pub speed: f32,
    pub pitch: f32,
    pub id: Option<WindowId>,
}

impl Camera {
    const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

    pub fn new(position: Option<Vec3>, projection_type: ProjectionType) -> Camera {
        let yaw = 0.0;
        let pitch = 0.0;
        let camera = Camera {
            position: position.unwrap_or_else(|| [0.0, 0.0, -5.0]),
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            forward: [0.0, 0.0, -1.0],
            sensitivity: 0.1,
            speed: 0.08,
            yaw,
            pitch,
            id: None,
            projection_type,
        };

        camera
    }

    pub fn look_at(&mut self, point: Vec3) {
        self.forward = normalize_vec3(subtract_vec3(point, self.position));
        self.update_right();
    }

    fn update_right(&mut self) {
        self.right = normalize_vec3(cross_vec3(self.up, self.forward));
    }
    pub fn move_forward(&mut self, delta: f32) {
        // Move forward along the forward vector
        let forward_normalized = normalize_vec3(self.forward);
        for i in 0..3 {
            self.position[i] += forward_normalized[i] * delta;
        }
    }

    pub fn move_right(&mut self, delta: f32) {
        // Move right along the right vector
        let right_normalized = normalize_vec3(self.right);
        for i in 0..3 {
            self.position[i] += right_normalized[i] * delta;
        }
    }

    pub fn move_up(&mut self, delta: f32) {
        // Move up along the up vector
        let up_normalized = normalize_vec3(self.up);
        for i in 0..3 {
            self.position[i] += up_normalized[i] * delta;
        }
    }

    pub fn rotate(&mut self, yaw_offset: f32, pitch_offset: f32) {
        self.yaw += yaw_offset * self.sensitivity;
        self.pitch += pitch_offset * self.sensitivity;

        // Clamp the pitch to avoid flipping upside down
        self.pitch = self.pitch.clamp(-Self::MAX_PITCH, Self::MAX_PITCH);

        // Update the forward, right, and up vectors after changing yaw/pitch

        self.update_vectors();
    }

    fn update_vectors(&mut self) {
        // Calculate the forward vector using yaw and pitch
        self.forward = normalize_vec3([
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        ]);

        // Recalculate right and up vectors based on new forward vector
        let world_up = [0.0, 1.0, 0.0];
        self.right = normalize_vec3(cross_vec3(world_up, self.forward));
        self.up = cross_vec3(self.forward, self.right);
    }
    pub fn set_yaw_pitch(&mut self, yaw_offset: f32, pitch_offset: f32) {
        // Update yaw and pitch with provided offsets
        self.yaw += yaw_offset * self.sensitivity;
        self.pitch += pitch_offset * self.sensitivity;

        // Clamp the pitch to avoid flipping upside down
        self.pitch = self.pitch.clamp(-Self::MAX_PITCH, Self::MAX_PITCH);

        // Update the forward, right, and up vectors after changing yaw/pitch
        self.update_vectors();
    }

    pub fn rotate_y(angle: f32) -> Mat4 {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        [
            [cos_angle, 0.0, -sin_angle, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sin_angle, 0.0, cos_angle, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    pub fn view_matrix(&self) -> Mat4 {
        let forward = normalize_vec3(self.forward);
        let right = normalize_vec3(self.right);
        let up = normalize_vec3(self.up);
        let pos = self.position;

        // Translation part
        let translation = [
            -dot_vec3(right, pos),
            -dot_vec3(up, pos),
            -dot_vec3(forward, pos),
        ];

        // Create the look-at matrix (aka the view matrix)
        [
            [right[0], up[0], forward[0], 0.0],
            [right[1], up[1], forward[1], 0.0],
            [right[2], up[2], forward[2], 0.0],
            [translation[0], translation[1], translation[2], 1.0],
        ]
    }

    // Update the projection_matrix method
    pub fn projection_matrix(&self, perspective: &CameraPerspective) -> Mat4 {
        match self.projection_type {
            ProjectionType::Perspective => perspective.perspective_projection(),
            ProjectionType::Orthographic => perspective.orthographic_projection(),
        }
    }
    pub fn orthogonal(&self) -> Mat4 {
        let forward = normalize_vec3(self.forward);
        let right = normalize_vec3(self.right);
        let up = normalize_vec3(self.up);
        let pos = self.position;

        // Translation part
        let translation = [
            -dot_vec3(right, pos),
            -dot_vec3(up, pos),
            -dot_vec3(forward, pos),
        ];

        // Create the look-at matrix (aka the view matrix)
        [
            [right[0], up[0], forward[0], 0.0],
            [right[1], up[1], forward[1], 0.0],
            [right[2], up[2], forward[2], 0.0],
            [translation[0], translation[1], translation[2], 1.0],
        ]
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
}
