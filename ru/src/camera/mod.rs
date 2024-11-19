use crate::ecs::geometry::plane::Plane3D;
use crate::input::InputListener;
use crate::prelude::constant::{PERSPECTIVE_FAR, PERSPECTIVE_NEAR, ZERO_F32};
pub mod frustum;
pub mod projection;
use nalgebra::{Matrix4, Point3, Vector3};
use projection::{CameraProjection, ProjectionMode};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::KeyCode;

#[repr(C)]
#[derive(Default, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Plane {
    pub normal: [f32; 3],
    pub distance: f32,
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub projection: CameraProjection,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub speed: f32,
    pub is_panning: bool,
    pub plane: Option<Plane3D>,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        projection: CameraProjection,
        speed: f32,
        plane: Option<Plane3D>,
    ) -> Self {
        let direction = (position - target).normalize();
        let distance = (position - target).magnitude();
        let yaw = direction.z.atan2(direction.x);
        let pitch = direction.y.asin();
        Camera {
            position,
            target,
            up,
            projection,
            yaw,
            pitch,
            distance,
            speed,
            is_panning: false,
            plane,
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn view_projection_matrix(&self) -> [[f32; 4]; 4] {
        (self.projection.projection_matrix() * self.view_matrix()).into()
    }

    pub fn set_projection_mode(&mut self, mode: ProjectionMode) {
        self.projection.set_projection_mode(mode);
    }
}
impl Camera {
    pub fn set_speed(&mut self, new_speed: f32) {
        self.speed = new_speed;
    }

    pub fn increment_speed(&mut self) {
        self.speed *= 1.5;
    }

    pub fn decrement_speed(&mut self) {
        self.speed /= 1.5;
    }

    pub fn position(&self) -> [f32; 3] {
        self.position.into()
    }

    pub fn update_camera_position(&mut self) {
        let x = self.distance * self.pitch.cos() * self.yaw.cos();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.sin();
        let offset = Vector3::new(x, y, z);
        self.position = self.target + offset;
    }

    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(-1.5, 1.5);
        self.update_camera_position();
    }
    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance - amount).clamp(0.5, 100.0);
        self.update_camera_position();
    }
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = Vector3::new(self.yaw.sin(), 0.0, -self.yaw.cos()).normalize();
        let up = self.up;

        self.target += right * delta_x - up * delta_y;

        self.update_camera_position();
    }
    pub fn move_forward(&mut self, delta_time: f32) {
        let forward = Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
        self.target += forward * self.speed * delta_time;
        self.update_camera_position();
    }
    pub fn move_right(&mut self, delta_time: f32) {
        let right = Vector3::new(self.yaw.sin(), 0.0, -self.yaw.cos()).normalize();
        self.target += right * self.speed * delta_time;
        self.update_camera_position();
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 0.0),
            *Vector3::y_axis(),
            CameraProjection::new_perspective(
                16.0 / 9.0,
                std::f32::consts::FRAC_PI_4,
                PERSPECTIVE_NEAR,
                PERSPECTIVE_FAR,
            ),
            1.0,
            None,
        )
    }
}

impl InputListener for Camera {
    fn on_key_event(&mut self, event: &KeyEvent, delta_time: f32) {
        match event.physical_key {
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyW) => self.move_forward(-delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyS) => self.move_forward(delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyA) => self.move_right(-delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyD) => self.move_right(delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowUp) => self.pan(ZERO_F32, delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.pan(ZERO_F32, -delta_time)
            }
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.pan(-delta_time, ZERO_F32)
            }
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowRight) => {
                self.pan(delta_time, ZERO_F32)
            }
            winit::keyboard::PhysicalKey::Code(KeyCode::NumpadSubtract) => {
                self.decrement_speed();
            }
            winit::keyboard::PhysicalKey::Code(KeyCode::NumpadAdd) => {
                self.increment_speed();
            }
            _ => (),
        }
    }
    fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        if !self.is_panning {
            self.rotate(delta.0 as f32 * 0.01, delta.1 as f32 * 0.01);
        } else {
            let delta_x = delta.0 as f32 * 0.05;
            let delta_y = delta.1 as f32 * 0.05;
            self.pan(delta_x, delta_y);
        }
    }
    fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left {
            self.is_panning = state == ElementState::Pressed;
        }
    }
    fn on_scroll(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, scroll_y) => self.zoom(scroll_y),
            MouseScrollDelta::PixelDelta(delta) => self.zoom(delta.y as f32 * 0.1),
        }
    }
}
