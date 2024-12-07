pub use winit::event;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
pub mod action;

pub fn process_input_events<F>(event: &WindowEvent, process_fn: F)
where
    F: FnOnce(),
{
    match event {
        WindowEvent::CursorMoved { .. }
        | WindowEvent::KeyboardInput { .. }
        | WindowEvent::MouseWheel {
            delta: MouseScrollDelta::PixelDelta(_) | MouseScrollDelta::LineDelta(_, _),
            ..
        } => {
            process_fn();
        }
        _ => {}
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum InputContext {
    NoSurface,
    Window,
}
pub const MOUSE_SCROLL_ZERO: (f64, f64) = (0.0, 0.0);
pub const MOUSE_POSITION_ZERO: (f64, f64) = (0.0, 0.0);

pub type InputElementState = winit::event::ElementState;
pub type MouseMotionDelta = (f64, f64);

#[derive(Debug, Clone)]
pub struct MouseButtonElementState {
    pub button: winit::event::MouseButton,
    pub state: InputElementState,
}
#[derive(Debug, Clone)]
pub enum MouseInputEventType {
    Motion(MouseMotionDelta),
    CursorLeft,
    Button(MouseButtonElementState),
    Scroll(winit::event::MouseScrollDelta),
}
#[derive(Debug, Clone)]
pub enum KeyInputEventType {
    Key(winit::event::KeyEvent),
    RawKey(winit::event::RawKeyEvent),
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    MouseInput(MouseInputEventType),
    KeyInput(KeyInputEventType),
}

pub trait InputListener {
    fn on_key_event(&mut self, event: &KeyEvent, delta_time: f32);
    fn on_mouse_motion(&mut self, delta: (f64, f64));
    fn on_mouse_button(&mut self, button: MouseButton, state: ElementState);
    fn on_scroll(&mut self, delta: MouseScrollDelta);
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
        self.listeners.push(listener);
    }

    pub fn handle_input(&mut self, event: &KeyEvent, delta: f32) {
        for listener in self.listeners.iter_mut() {
            listener.on_key_event(event, delta);
        }
    }

    pub fn handle_mouse_motion(&mut self, delta: (f64, f64)) {
        for listener in self.listeners.iter_mut() {
            listener.on_mouse_motion(delta);
        }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        for listener in self.listeners.iter_mut() {
            listener.on_mouse_button(button, state);
        }
    }
}

#[derive(Debug)]
pub struct MouseMovementDetails {
    pub direction: String,
    pub speed: f64,
    pub delta: (f64, f64),
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
