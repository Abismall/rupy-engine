use std::time::Instant;

#[derive(Debug)]
pub struct Time {
    start_time: Instant,               // When the clock started
    last_frame_time: Instant,          // Time of the last frame
    delta_time: f64,                   // Time between the last two frames
    total_paused_time: f64,            // Total time spent in paused state
    pause_start_time: Option<Instant>, // If the clock is paused, this stores when it was paused
    is_paused: bool,                   // Whether the clock is currently paused
    time_scale: f64, // Scales how time flows (1.0 = normal speed, 0.5 = half speed, etc.)
    frame_count: u64, // Number of frames elapsed
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
            time_scale: 1.0, // Default time scale to normal speed
            frame_count: 0,
        }
    }

    /// Updates the clock. This should be called once per frame.
    pub fn update(&mut self) {
        if self.is_paused {
            return; // Don't update if the clock is paused
        }

        let now = Instant::now();
        self.delta_time = (now - self.last_frame_time).as_secs_f64() * self.time_scale;
        self.last_frame_time = now;
        self.frame_count += 1;
    }

    /// Returns the delta time (time between the last two frames) in seconds, accounting for time scaling.
    pub fn delta_time(&self) -> f64 {
        if self.is_paused {
            0.0 // No time advances when paused
        } else {
            self.delta_time
        }
    }

    /// Returns the total time since the `Time` instance was created in seconds, accounting for pauses.
    pub fn total_time(&self) -> f64 {
        let elapsed_time = (self.last_frame_time - self.start_time).as_secs_f64();
        elapsed_time - self.total_paused_time
    }

    /// Returns the `Instant` of the last frame.
    pub fn last_frame_instant(&self) -> Instant {
        self.last_frame_time
    }

    /// Pauses the clock.
    pub fn pause(&mut self) {
        if !self.is_paused {
            self.is_paused = true;
            self.pause_start_time = Some(Instant::now());
        }
    }

    /// Resumes the clock if it was paused.
    pub fn resume(&mut self) {
        if self.is_paused {
            if let Some(pause_time) = self.pause_start_time {
                self.total_paused_time += (Instant::now() - pause_time).as_secs_f64();
            }
            self.is_paused = false;
            self.pause_start_time = None;
        }
    }

    /// Sets the time scale. A value of 1.0 is real-time, less than 1.0 is slower, and greater than 1.0 is faster.
    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale.clamp(0.0, 10.0); // Limit time scaling to a reasonable range
    }

    /// Returns the current time scale.
    pub fn time_scale(&self) -> f64 {
        self.time_scale
    }

    /// Returns the number of frames elapsed since the clock started.
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Converts a given number of frames to time in seconds, based on the current frame rate.
    pub fn frames_to_time(&self, frames: u64) -> f64 {
        frames as f64 * self.delta_time
    }

    /// Returns the actual elapsed time in seconds, without accounting for pauses.
    pub fn real_elapsed_time(&self) -> f64 {
        (self.last_frame_time - self.start_time).as_secs_f64()
    }

    /// Returns the actual elapsed time (without time scaling or pausing).
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
    /// Compiles all debug metrics into a formatted string.
    pub fn debug(&self) -> String {
        format!(
            "FPS: {:.0}\n\
             Delta: {:.6} s (Scaled)\n\
             Total Time: {:.2} s\n\
             Real Time: {:.2} s\n\
             Frame Count: {}\n\
             Time Scale: {:.2}",
            self.fps(),               // Current FPS
            self.delta_time(),        // Scaled delta time in seconds
            self.total_time(),        // Total (scaled) time in seconds
            self.real_elapsed_time(), // Real elapsed time (unscaled)
            self.frame_count,         // Frame count
            self.time_scale           // Time scale
        )
    }
}
