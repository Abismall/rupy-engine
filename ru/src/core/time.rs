use std::{collections::VecDeque, time::Instant};

#[derive(Debug)]
pub struct Time {
    start_time: Instant,
    last_frame_time: Instant,
    delta_time: f64,
    total_paused_time: f64,
    pause_start_time: Option<Instant>,
    is_paused: bool,
    time_scale: f64,

    frame_times: VecDeque<f64>,
    max_frames_in_window: usize,
    total_frame_time_window: f64,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame_time: now,
            delta_time: 0.0,
            total_paused_time: 0.0,
            pause_start_time: None,
            is_paused: false,
            time_scale: 1.0,
            frame_times: VecDeque::new(),
            max_frames_in_window: 144,
            total_frame_time_window: 0.0,
        }
    }

    pub fn update(&mut self) {
        if self.is_paused {
            return;
        }

        let now = Instant::now();
        self.delta_time = (now - self.last_frame_time).as_secs_f64() * self.time_scale;
        self.last_frame_time = now;

        self.frame_times.push_back(self.delta_time);
        self.total_frame_time_window += self.delta_time;

        if self.frame_times.len() > self.max_frames_in_window {
            if let Some(removed_time) = self.frame_times.pop_front() {
                self.total_frame_time_window -= removed_time;
            }
        }
    }
    pub fn average_frame_time_window(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        self.total_frame_time_window / self.frame_times.len() as f64
    }
    pub fn average_fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total_time: f64 = self.frame_times.iter().sum();
        if total_time > 0.0 {
            self.frame_times.len() as f64 / total_time
        } else {
            0.0
        }
    }

    pub fn delta_time(&self) -> f64 {
        if self.is_paused {
            0.0
        } else {
            self.delta_time
        }
    }

    pub fn total_time(&self) -> f64 {
        let elapsed_time = (self.last_frame_time - self.start_time).as_secs_f64();
        elapsed_time - self.total_paused_time
    }

    pub fn last_frame_instant(&self) -> Instant {
        self.last_frame_time
    }

    pub fn pause(&mut self) {
        if !self.is_paused {
            self.is_paused = true;
            self.pause_start_time = Some(Instant::now());
        }
    }

    pub fn resume(&mut self) {
        if self.is_paused {
            if let Some(pause_time) = self.pause_start_time {
                self.total_paused_time += (Instant::now() - pause_time).as_secs_f64();
            }
            self.is_paused = false;
            self.pause_start_time = None;
        }
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale.clamp(0.0, 10.0);
    }

    pub fn time_scale(&self) -> f64 {
        self.time_scale
    }

    pub fn frames_to_time(&self, frames: u64) -> f64 {
        frames as f64 * self.delta_time
    }

    pub fn real_delta_time(&self) -> f64 {
        (Instant::now() - self.last_frame_time).as_secs_f64()
    }

    pub fn fps(&self) -> f64 {
        if self.delta_time > 0.0 {
            1.0 / self.delta_time
        } else {
            0.0
        }
    }

    pub fn debug(&self) -> String {
        format!(
            "FPS (Instant): {:.0}\n\
             FPS (Avg): {:.0}\n\
             Avg Frame Time (Last {} frames): {:.6} s\n\
             Delta: {:.6} s (Scaled)\n\
             Total Time: {:.2} s\n\
             Time Scale: {:.2}",
            self.fps(),
            self.average_fps(),
            self.frame_times.len(),
            self.average_frame_time_window(),
            self.delta_time(),
            self.total_time(),
            self.time_scale
        )
    }
}
