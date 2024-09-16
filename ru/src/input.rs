use crate::log_debug;
use winit::event::RawKeyEvent;
#[derive(Default)]
pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }
    pub fn mousemotion(&mut self, delta: (f64, f64)) {
        let movement = MouseMovementDetails::new(delta);
    }
    pub fn key(&mut self, event: &RawKeyEvent) {
        match event.physical_key {
            winit::keyboard::PhysicalKey::Code(key_code) => {
                log_debug!("Received input: {:?}", key_code);
            }
            winit::keyboard::PhysicalKey::Unidentified(native_key_code) => {
                log_debug!("Received input: {:?}", native_key_code);
            }
        }
    }
}

#[derive(Debug)]
struct MouseMovementDetails {
    direction: String,
    speed: f64,
    delta: (f64, f64),
}
impl MouseMovementDetails {
    pub fn new(delta: (f64, f64)) -> Self {
        let (dx, dy) = delta;

        let direction = match (dx, dy) {
            (0.0, 0.0) => "Stationary".to_string(),
            (dx, 0.0) if dx > 0.0 => "Right".to_string(),
            (dx, 0.0) if dx < 0.0 => "Left".to_string(),
            (0.0, dy) if dy > 0.0 => "Down".to_string(),
            (0.0, dy) if dy < 0.0 => "Up".to_string(),
            (dx, dy) if dx > 0.0 && dy > 0.0 => "Down-Right".to_string(),
            (dx, dy) if dx > 0.0 && dy < 0.0 => "Up-Right".to_string(),
            (dx, dy) if dx < 0.0 && dy > 0.0 => "Down-Left".to_string(),
            (dx, dy) if dx < 0.0 && dy < 0.0 => "Up-Left".to_string(),
            _ => "Unknown".to_string(),
        };

        let speed = ((dx * dx) + (dy * dy)).sqrt();

        Self {
            direction,
            speed,
            delta,
        }
    }
}
