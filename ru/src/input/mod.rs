pub(crate) mod handler;
pub(crate) mod traits;

use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, RawKeyEvent};
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
