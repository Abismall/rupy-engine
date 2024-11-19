use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};

use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position},
    window::{Fullscreen, Window, WindowAttributes},
};

pub fn window_inner_size_to_vector2(window: &Window) -> nalgebra::Vector2<u32> {
    let window_size = window.inner_size();
    nalgebra::Vector2::new(window_size.width.max(1), window_size.height.max(1))
}

pub fn window_position_logical(
    window_size: LogicalSize<f64>,
    screen_size: LogicalSize<f64>,
) -> Position {
    let x = (screen_size.width - window_size.width) / 2.0;
    let y = (screen_size.height - window_size.height) / 2.0;

    Position::Logical(LogicalPosition::new(x, y))
}

pub fn window_position_physical(
    window_size: PhysicalSize<u32>,
    screen_size: PhysicalSize<u32>,
) -> Position {
    let x = (screen_size.width.saturating_sub(window_size.width)) / 2;
    let y = (screen_size.height.saturating_sub(window_size.height)) / 2;

    Position::Physical(PhysicalPosition::new(x as i32, y as i32))
}

pub fn default_window_attributes(
    fullscreen: Option<Fullscreen>,
    title: Option<&str>,
) -> WindowAttributes {
    WindowAttributes::default()
        .with_fullscreen(fullscreen)
        .with_title(title.unwrap_or("Rupy"))
}

pub fn u32_to_physical_size(width: u32, height: u32) -> PhysicalSize<u32> {
    PhysicalSize::new(width, height)
}

pub fn f64_to_logical_size(width: f64, height: f64) -> LogicalSize<f64> {
    LogicalSize::new(width, height)
}

pub fn calculate_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}
pub fn calculate_hashes<T: Hash>(item: &Vec<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for t in item.iter() {
        t.hash(&mut hasher);
    }
    hasher.finish()
}
pub fn string_to_u64(s: &str) -> u64 {
    calculate_hash(&s.to_string())
}

pub fn get_window_attributes() -> (u32, u32, i32, i32) {
    let width: u32 = env::var("RUPY_ENGINE_WINDOW_WIDTH")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(1000);
    let height: u32 = env::var("RUPY_ENGINE_WINDOW_HEIGHT")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(800);
    let x: i32 = env::var("RUPY_ENGINE_WINDOW_X_ANCHOR")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(450);
    let y: i32 = env::var("RUPY_ENGINE_WINDOW_Y_ANCHOR")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(100);
    (width, height, x, y)
}
