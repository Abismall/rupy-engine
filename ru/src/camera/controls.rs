use cgmath::{InnerSpace, Rad, Vector3};
use winit::event::{ElementState, MouseScrollDelta, WindowEvent};

use crate::log_info;

use super::Camera;

pub struct CameraController {
    speed: f32,
    sensitivity: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
    yaw: f32,
    pitch: f32,
    last_mouse_pos: Option<(f64, f64)>,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
            yaw: -90.0,
            pitch: 0.0,
            last_mouse_pos: None,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        state,
                        physical_key,
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match physical_key {
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW) => {
                        self.is_forward_pressed = is_pressed;
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS) => {
                        self.is_backward_pressed = is_pressed
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => {
                        self.is_left_pressed = is_pressed
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => {
                        self.is_right_pressed = is_pressed
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Space) => {
                        self.is_up_pressed = is_pressed
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ShiftLeft) => {
                        self.is_down_pressed = is_pressed
                    }
                    _ => return false,
                }
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some((last_x, last_y)) = self.last_mouse_pos {
                    let delta_x = position.x - last_x;
                    let delta_y = position.y - last_y;

                    self.yaw += delta_x as f32 * self.sensitivity;
                    self.pitch -= delta_y as f32 * self.sensitivity;

                    self.pitch = self.pitch.clamp(-90.0, 90.0);
                }
                self.last_mouse_pos = Some((position.x, position.y));
                true
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, scroll_y) => {
                    self.speed = (self.speed - scroll_y).clamp(0.1, 10.0);
                    true
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    self.speed = (self.speed - delta.y as f32 * 0.1).clamp(0.1, 10.0);
                    true
                }
            },
            _ => false,
        }
    }

    pub fn update(&mut self, camera: &mut Camera, dt: f32) {
        let forward = Vector3::new(
            self.yaw.to_radians().cos(),
            0.0,
            self.yaw.to_radians().sin(),
        )
        .normalize();
        let right = forward.cross(Vector3::unit_y()).normalize();
        let up = Vector3::unit_y();

        let movement = forward
            * (self.is_forward_pressed as u8 as f32 - self.is_backward_pressed as u8 as f32)
            + right * (self.is_right_pressed as u8 as f32 - self.is_left_pressed as u8 as f32)
            + up * (self.is_up_pressed as u8 as f32 - self.is_down_pressed as u8 as f32);

        if movement.magnitude() > 0.0 {
            camera.position += movement.normalize() * self.speed * dt;
        }

        camera.yaw = Rad(self.yaw.to_radians());
        camera.pitch = Rad(self.pitch.to_radians());
        log_info!("camera updated! {:?}", camera);
    }
}
