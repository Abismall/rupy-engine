use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, RawKeyEvent};

pub(crate) mod handler;
pub trait InputListener {
    fn on_key_event(&mut self, event: &KeyEvent);
    fn on_mouse_motion(&mut self, delta: (f64, f64));
    fn on_mouse_button(&mut self, button: MouseButton, state: ElementState);
    fn on_scroll(&mut self, delta: MouseScrollDelta);
}
