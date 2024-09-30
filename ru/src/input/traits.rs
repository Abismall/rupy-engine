use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, RawKeyEvent};

// Key events
pub trait KeyListener {
    fn on_key_event(&mut self, event: &KeyEvent);
}

// Raw key events
pub trait RawKeyListener {
    fn on_raw_key_event(&mut self, event: &RawKeyEvent);
}

// Mouse motion events
pub trait MouseMotionListener {
    fn on_mouse_motion(&mut self, delta: (f64, f64));
}

// Mouse button events
pub trait MouseButtonListener {
    fn on_mouse_button(&mut self, button: MouseButton, state: ElementState);
}

// Scroll events
pub trait ScrollListener {
    fn on_scroll(&mut self, delta: MouseScrollDelta);
}
