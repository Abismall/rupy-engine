use winit::event::{ElementState, RawKeyEvent};

pub trait InputListener {
    fn on_key_event(&mut self, event: &RawKeyEvent);
    fn on_mouse_motion(&mut self, delta: (f64, f64));
    fn on_mouse_button(&mut self, button: u32, state: ElementState);
}

pub struct InputHandler {
    listeners: Vec<Box<dyn InputListener>>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn InputListener>) {
        if !self.listener_exists(&*listener) {
            self.listeners.push(listener);
        }
    }

    pub fn listener_exists(&self, listener: &dyn InputListener) -> bool {
        self.listeners.iter().any(|l| std::ptr::eq(&**l, listener))
    }

    pub fn remove_listener(&mut self, listener: &dyn InputListener) -> bool {
        if let Some(index) = self
            .listeners
            .iter()
            .position(|l| std::ptr::eq(&**l, listener))
        {
            self.listeners.remove(index);
            true
        } else {
            false
        }
    }
    pub fn key(&mut self, event: &RawKeyEvent) {
        for listener in &mut self.listeners {
            listener.on_key_event(event);
        }
    }

    pub fn mousemotion(&mut self, delta: (f64, f64)) {
        for listener in &mut self.listeners {
            listener.on_mouse_motion(delta);
        }
    }

    pub fn mouse_button(&mut self, button: u32, state: winit::event::ElementState) {
        for listener in &mut self.listeners {
            listener.on_mouse_button(button, state);
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
