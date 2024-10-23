pub mod binding;

pub mod manager;
pub use winit::event;
pub mod action;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum InputContext {
    NoSurface,
    Window,
}
const MOUSE_SCROLL_ZERO: (f64, f64) = (0.0, 0.0);
const MOUSE_POSITION_ZERO: (f64, f64) = (0.0, 0.0);
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
