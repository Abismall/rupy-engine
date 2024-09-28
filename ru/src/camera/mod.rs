use nalgebra::{Matrix4, Point3, Vector3};
use winit::{
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, RawKeyEvent},
    keyboard::PhysicalKey,
};

use crate::input::InputListener;

#[derive(Debug, Clone, Copy)]
pub enum CameraMode {
    FirstPerson,
    ThirdPerson,
    Orbit,
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
    pub yaw: f32,   // In degrees
    pub pitch: f32, // In degrees
    pub is_rotating: bool,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub front: Vector3<f32>,
    pub right: Vector3<f32>,
    pub accumulated_mouse_delta: (f32, f32),
    pub last_mouse_position: Option<(f64, f64)>,
    pressed_keys: std::collections::HashSet<winit::keyboard::PhysicalKey>,
}

impl Camera {
    pub fn new(position: Point3<f32>, aspect_ratio: f32) -> Self {
        let front = Vector3::new(0.0, 0.0, -1.0);

        let up = Vector3::new(0.0, 1.0, 0.0);

        let right = front.cross(&up).normalize();
        Camera {
            position,
            target: position + Vector3::new(0.0, 0.0, -1.0),
            up,
            aspect_ratio,
            fov_y: 45.0,
            near: 0.1,
            far: 100.0,
            yaw: -90.0,
            pitch: 0.0,
            is_rotating: false,
            movement_speed: 10.0,
            mouse_sensitivity: 0.1,
            zoom_sensitivity: 1.0,
            front,
            right,
            accumulated_mouse_delta: (0.0, 0.0),
            last_mouse_position: None,
            pressed_keys: Default::default(),
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;

        if self
            .pressed_keys
            .contains(&PhysicalKey::Code(winit::keyboard::KeyCode::KeyW))
        {
            self.position += self.front * velocity;
        }
        if self
            .pressed_keys
            .contains(&PhysicalKey::Code(winit::keyboard::KeyCode::KeyS))
        {
            self.position -= self.front * velocity;
        }
        if self
            .pressed_keys
            .contains(&PhysicalKey::Code(winit::keyboard::KeyCode::KeyA))
        {
            self.position -= self.right * velocity;
        }
        if self
            .pressed_keys
            .contains(&PhysicalKey::Code(winit::keyboard::KeyCode::KeyD))
        {
            self.position += self.right * velocity;
        }

        let (delta_x, delta_y) = self.accumulated_mouse_delta;
        self.process_mouse_movement(delta_x, delta_y);

        self.accumulated_mouse_delta = (0.0, 0.0);

        self.update_camera_vectors();
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(
            self.aspect_ratio,
            self.fov_y.to_radians(),
            self.near,
            self.far,
        )
    }

    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    pub fn update_camera_vectors(&mut self) {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        let front = Vector3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        )
        .normalize();

        self.front = front;
        self.target = self.position + front;

        let world_up = Vector3::new(0.0, 1.0, 0.0);
        let right = front.cross(&world_up).normalize();
        let up = right.cross(&front).normalize();

        self.right = right;
        self.up = up;
    }
    pub fn look_at(&mut self, target: Point3<f32>) {
        self.target = target;

        let direction = (self.position - self.target).normalize();

        self.yaw = direction.z.atan2(direction.x).to_degrees();
        self.pitch = direction.y.asin().to_degrees();

        self.update_camera_vectors();
    }

    pub fn look_at_object(&mut self, object_position: Point3<f32>) {
        self.look_at(object_position);
    }

    pub fn process_mouse_movement(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * self.mouse_sensitivity;
        self.pitch += delta_y * self.mouse_sensitivity;

        self.yaw = self.yaw % 360.0;
        if self.yaw < 0.0 {
            self.yaw += 360.0;
        }

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.update_camera_vectors();
    }
    pub fn front(&self) -> Vector3<f32> {
        self.front
    }

    pub fn right(&self) -> Vector3<f32> {
        self.right
    }

    pub fn camera_up(&self) -> Vector3<f32> {
        self.up
    }
    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        self.fov_y -= yoffset * self.zoom_sensitivity;
        self.fov_y = self.fov_y.clamp(1.0, 45.0);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(Point3::new(0.0, 0.0, 5.0), 16.0 / 9.0)
    }
}

impl InputListener for Camera {
    fn on_key_event(&mut self, event: &KeyEvent) {
        match event.state {
            ElementState::Pressed => {
                self.pressed_keys.insert(event.physical_key);
            }
            ElementState::Released => {
                self.pressed_keys.remove(&event.physical_key);
            }
        }
    }

    fn on_mouse_motion(&mut self, position: (f64, f64)) {
        if let Some((last_x, last_y)) = self.last_mouse_position {
            let delta_x = (position.0 - last_x) as f32;
            let delta_y = (last_y - position.1) as f32;

            self.accumulated_mouse_delta.0 += delta_x;
            self.accumulated_mouse_delta.1 += delta_y;
        }
        self.last_mouse_position = Some(position);
    }

    fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left {
            self.is_rotating = state == ElementState::Pressed;
        }
    }

    fn on_scroll(&mut self, delta: MouseScrollDelta) {
        let yoffset = match delta {
            MouseScrollDelta::LineDelta(_, scroll_y) => scroll_y,
            MouseScrollDelta::PixelDelta(delta) => delta.y as f32,
        };
        self.process_mouse_scroll(yoffset);
    }

    fn on_raw_key_event(&mut self, _event: &RawKeyEvent) {}
}
