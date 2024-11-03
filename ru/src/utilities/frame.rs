use std::time::Duration;
use std::time::Instant;
#[derive(Debug, Clone, Copy)]
pub struct FrameTime {
    pub last_frame_time: Instant,
    pub frame_time_accumulator: Duration,
    pub frame_count: u32,
    pub fps: f32,
    pub delta_time: f32,
}

impl FrameTime {
    pub fn new() -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_time_accumulator: Duration::default(),
            frame_count: 0,
            fps: 0.0,
            delta_time: 0.0,
        }
    }

    pub fn compute(&mut self) {
        let now = Instant::now();
        let delta_duration = now.duration_since(self.last_frame_time);

        self.delta_time = delta_duration.as_secs_f32();

        self.frame_time_accumulator += delta_duration;
        self.frame_count += 1;

        if self.frame_time_accumulator >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32 / self.frame_time_accumulator.as_secs_f32();
            self.frame_count = 0;
            self.frame_time_accumulator = Duration::ZERO;
        }

        self.last_frame_time = now;
    }
}
