use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct FrameMetrics {
    pub last_frame_time: Instant,
    pub frame_time_accumulator: Duration,
    pub frame_count: u32,
    pub fps: f32,
    pub delta_time: f32,
    pub total_instances: u32,
    pub culled_instances: u32,
    pub visible_instances: u32,
}

impl FrameMetrics {
    pub fn new() -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_time_accumulator: Duration::ZERO,
            frame_count: 0,
            fps: 0.0,
            delta_time: 0.0,
            total_instances: 0,
            culled_instances: 0,
            visible_instances: 0,
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

    pub fn update_instance_stats(&mut self, total: u32, culled: u32) {
        self.total_instances = total;
        self.culled_instances = culled;
        self.visible_instances = total.saturating_sub(culled);
    }

    pub fn culled_percentage(&self) -> f32 {
        if self.total_instances == 0 {
            0.0
        } else {
            (self.culled_instances as f32 / self.total_instances as f32) * 100.0
        }
    }

    pub fn visible_percentage(&self) -> f32 {
        if self.total_instances == 0 {
            0.0
        } else {
            (self.visible_instances as f32 / self.total_instances as f32) * 100.0
        }
    }
}
