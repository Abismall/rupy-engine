use winit::dpi::PhysicalPosition;

pub struct MenuMouseInputState {
    clicked: bool,
    mouse_coords: PhysicalPosition<f64>,
}
