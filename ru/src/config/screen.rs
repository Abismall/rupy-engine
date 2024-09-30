use std::time::Duration;

use crate::object::color::Color;

#[derive(Clone, Debug)]
pub struct ScreenConfig {
    pub clear_color: Option<Color>,
    pub max_fps: Option<u32>,
    pub vsync: bool,
    pub(crate) changed: bool,
}

impl Default for ScreenConfig {
    fn default() -> Self {
        Self {
            clear_color: Some(Color::BLACK),
            max_fps: None,
            vsync: false,
            changed: true,
        }
    }
}

impl ScreenConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_clear_color(mut self, clear_color: Option<Color>) -> Self {
        self.clear_color = clear_color;
        self.changed = true;
        self
    }

    pub fn with_max_fps(mut self, max_fps: Option<u32>) -> Self {
        self.max_fps = max_fps;
        self.changed = true;
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self.changed = true;
        self
    }

    pub fn clear_color(&self) -> Option<Color> {
        self.clear_color
    }

    pub fn max_fps(&self) -> Option<u32> {
        self.max_fps
    }

    pub fn vsync(&self) -> bool {
        self.vsync
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.changed = true;
        self.vsync = vsync;
    }

    pub fn set_clear_color(&mut self, clear_color: Option<Color>) {
        self.changed = true;
        self.clear_color = clear_color;
    }

    pub fn set_max_fps(&mut self, max_fps: Option<u32>) {
        self.changed = true;
        self.max_fps = max_fps;
    }

    pub fn max_frame_time(&self) -> Option<Duration> {
        self.max_fps
            .map(|fps| Duration::from_secs_f32(1.0 / fps as f32))
    }

    pub fn reset_changed(&mut self) {
        self.changed = false;
    }
}
