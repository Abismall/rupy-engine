#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    Normal,      // Regular rendering
    OutlineOnly, // Only render the outlines
    Freeze,      // Do not render any new frames
}

pub struct Settings {
    pub fps: bool,
    pub last_frame_time: bool,
    pub mode: RenderMode,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            fps: false,
            last_frame_time: false,
            mode: RenderMode::Normal,
        }
    }

    pub fn toggle_fps(&mut self) {
        self.fps = !self.fps;
    }

    pub fn toggle_last_frame_time(&mut self) {
        self.last_frame_time = !self.last_frame_time;
    }

    pub fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            RenderMode::Normal => RenderMode::OutlineOnly,
            RenderMode::OutlineOnly => RenderMode::Freeze,
            RenderMode::Freeze => RenderMode::Normal,
        };
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            fps: false,
            last_frame_time: false,
            mode: RenderMode::Normal,
        }
    }
}
