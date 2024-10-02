use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct DebugMetrics {
    pub last_frame_time: Instant,
    pub delta_time: f64,
    pub frame_count: u32,
    pub fps: f32,
}

impl DebugMetrics {
    pub fn new() -> Self {
        Self {
            last_frame_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta_duration = now.duration_since(self.last_frame_time);
        self.delta_time = delta_duration.as_secs_f64();

        self.frame_count += 1;
        if self.delta_time > 0.0 {
            self.fps = 1.0 / self.delta_time as f32;
        }
        if self.frame_count >= 60 {
            self.frame_count = 0;
        }
        self.last_frame_time = now;
    }
}
pub enum DebugEvent {
    Rehydrated,
    Initialized,
    Error(String),  // Error variant can carry a message
    Custom(String), // For custom event messages
}

impl DebugEvent {
    pub fn to_log(&self) -> String {
        match self {
            DebugEvent::Rehydrated => "[REHYDRATED]".to_string(),
            DebugEvent::Initialized => "[INITIALIZED]".to_string(),
            DebugEvent::Error(msg) => format!("[ERROR] {}", msg),
            DebugEvent::Custom(msg) => format!("[EVENT] {}", msg),
        }
    }
}
