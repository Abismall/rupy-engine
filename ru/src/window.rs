use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use winit::window::{Fullscreen, WindowAttributes};

pub fn calculate_center_position(
    window_size: LogicalSize<f64>,
    screen_size: LogicalSize<f64>,
) -> Position {
    let x = (screen_size.width - window_size.width) / 2.0;
    let y = (screen_size.height - window_size.height) / 2.0;

    Position::Logical(LogicalPosition::new(x, y))
}

pub fn build_window_attributes(
    size: (u32, u32),
    title: &str,
    position: Position,
    fullscreen: bool,
) -> WindowAttributes {
    let size = PhysicalSize::new(size.0.max(1), size.1.max(1));

    let mut window_attributes = WindowAttributes::default()
        .with_inner_size(size)
        .with_position(position)
        .with_title(title);

    if fullscreen {
        window_attributes.fullscreen = Some(Fullscreen::Borderless(None));
    }

    window_attributes
}
