use cgmath::{InnerSpace, Vector3, Zero};
use winit::event::{ElementState, MouseScrollDelta, WindowEvent};

use super::Camera;
#[derive(Debug)]
pub struct CameraController {
    speed: f32,
    sensitivity: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
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
            last_mouse_pos: None,
        }
    }
    pub fn process_movement(&mut self, event: &WindowEvent, camera: &mut Camera, dt: f32) {
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
                        self.is_backward_pressed = is_pressed;
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => {
                        self.is_left_pressed = is_pressed;
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => {
                        self.is_right_pressed = is_pressed;
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Space) => {
                        self.is_up_pressed = is_pressed;
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ShiftLeft) => {
                        self.is_down_pressed = is_pressed;
                    }
                    _ => {}
                }

                let (forward, right, up) = camera.calculate_vectors();
                let mut movement = Vector3::zero();

                if self.is_forward_pressed {
                    movement += forward;
                }
                if self.is_backward_pressed {
                    movement -= forward;
                }
                if self.is_right_pressed {
                    movement += right;
                }
                if self.is_left_pressed {
                    movement -= right;
                }
                if self.is_up_pressed {
                    movement += up;
                }
                if self.is_down_pressed {
                    movement -= up;
                }

                if movement.magnitude() > 0.0 {
                    movement = movement.normalize();
                    camera.position += movement * self.speed;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some((last_x, last_y)) = self.last_mouse_pos {
                    let delta_x = position.x - last_x;
                    let delta_y = position.y - last_y;

                    let scaled_delta_x = delta_x as f32 * self.sensitivity * 0.1;
                    let scaled_delta_y = delta_y as f32 * self.sensitivity * 0.1;

                    camera.yaw += cgmath::Rad(scaled_delta_x);

                    camera.pitch = {
                        let pitch = camera.pitch + cgmath::Rad(scaled_delta_y);
                        if pitch < cgmath::Rad(-89.0_f32.to_radians()) {
                            cgmath::Rad(-89.0_f32.to_radians())
                        } else if pitch > cgmath::Rad(89.0_f32.to_radians()) {
                            cgmath::Rad(89.0_f32.to_radians())
                        } else {
                            pitch
                        }
                    };
                }

                self.last_mouse_pos = Some((position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, scroll_y) => {
                    self.speed = (self.speed + scroll_y * 0.1).clamp(0.1, 10.0);
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    self.speed = (self.speed + delta.y as f32 * 0.01).clamp(0.1, 10.0);
                }
            },
            _ => {}
        }
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new(0.1, 0.1)
    }
}
