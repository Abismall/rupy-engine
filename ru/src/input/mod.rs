pub(crate) mod binding;
pub(crate) mod handler;
pub(crate) mod manager;
pub use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, RawKeyEvent};
#[derive(Debug, Clone)]
pub enum InputEvent {
    Key(KeyEvent),
    RawKey(RawKeyEvent),
    MouseMotion {
        delta: (f64, f64),
    },
    MouseButton {
        button: MouseButton,
        state: ElementState,
    },
    Scroll(MouseScrollDelta),
}
