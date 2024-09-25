use nalgebra::{Matrix4, Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub is_rotating: bool,
    pub move_speed: f32,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        aspect_ratio: f32,
        fov_y: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let direction = (position - target).normalize();
        let distance = (position - target).magnitude();

        let yaw = direction.z.atan2(direction.x);
        let pitch = direction.y.asin();

        Camera {
            position,
            target,
            up,
            aspect_ratio,
            fov_y,
            near,
            far,
            yaw,
            pitch,
            distance,
            is_rotating: false,
            move_speed: 100.0, // Set initial move speed (can be adjusted)
        }
    }
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect_ratio, self.fov_y, self.near, self.far)
    }

    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
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
        self.pitch = (self.pitch + delta_pitch).clamp(-1.5, 1.5); // Limit pitch to avoid flipping
        self.update_camera_position();
    }

    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance - amount).clamp(0.5, 100.0); // Limit zoom distance
        self.update_camera_position();
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = Vector3::new(self.yaw.sin(), 0.0, -self.yaw.cos()).normalize();
        let up = Vector3::new(0.0, 1.0, 0.0);

        self.target += right * delta_x + up * delta_y;
        self.update_camera_position();
    }

    /// Move forward in the direction the camera is looking
    pub fn move_forward(&mut self, delta_time: f32) {
        let forward = Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
        self.target += forward * self.move_speed * delta_time;
        self.update_camera_position();
    }

    /// Move right, perpendicular to the forward direction
    pub fn move_right(&mut self, delta_time: f32) {
        let right = Vector3::new(self.yaw.sin(), 0.0, -self.yaw.cos()).normalize();
        self.target += right * self.move_speed * delta_time;
        self.update_camera_position();
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Point3::new(0.0, 0.0, 5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: *Vector3::y_axis(),
            aspect_ratio: 16.0 / 9.0,
            fov_y: std::f32::consts::FRAC_PI_4,
            near: 0.1,
            far: 100.0,
            yaw: 0.0,
            pitch: 0.0,
            distance: 5.0,
            is_rotating: false,
            move_speed: 100.0, // Increased move speed for smoother control
        }
    }
}

use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::KeyCode;

use crate::input::InputListener;

impl InputListener for Camera {
    fn on_key_event(&mut self, event: &KeyEvent) {
        let delta_time = 0.016; // Assume 60 FPS (for smooth movement); calculate delta_time based on frame timing in a real app
        match event.physical_key {
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyW) => self.move_forward(-delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyS) => self.move_forward(delta_time),

            winit::keyboard::PhysicalKey::Code(KeyCode::KeyA) => self.move_right(-delta_time),
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyD) => self.move_right(delta_time),

            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowUp) => self.pan(0.0, 0.1),
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowDown) => self.pan(0.0, -0.1),

            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowLeft) => self.pan(-0.1, 0.0),
            winit::keyboard::PhysicalKey::Code(KeyCode::ArrowRight) => self.pan(0.1, 0.0),

            _ => (),
        }
    }

    fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        if self.is_rotating {
            self.rotate(delta.0 as f32 * 0.01, delta.1 as f32 * 0.01);
        }
    }

    fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left {
            self.is_rotating = state == ElementState::Pressed;
        }
    }

    fn on_scroll(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, scroll_y) => self.zoom(scroll_y),
            MouseScrollDelta::PixelDelta(delta) => self.zoom(delta.y as f32 * 0.1),
        }
    }
}
