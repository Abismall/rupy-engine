use std::time::{Duration, Instant};

use crate::log_debug;

pub struct PerformanceMetrics {
    last_frame_time: Instant,
    frame_count: u32,
    total_time: Duration,
    fps: u32,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_count: 0,
            total_time: Duration::new(0, 0),
            fps: 0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;

        self.total_time += frame_time;
        self.frame_count += 1;

        if self.total_time.as_secs() >= 1 {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.total_time = Duration::new(0, 0);
            log_debug!(
                "FPS: {}, Frame Time: {:.2} ms",
                self.fps,
                frame_time.as_secs_f64() * 1000.0
            );
        }
    }
}
