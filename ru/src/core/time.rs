use std::time::Instant;

pub struct Time {
    accumulator: Instant,
    last_frame_time: Option<Instant>,
    delta_time: f64,
}

impl Time {
    pub fn start() -> Self {
        Self {
            delta_time: 0.0,
            last_frame_time: None,
            accumulator: Instant::now(),
        }
    }
    fn tick(&mut self) -> Instant {
        let captured = self.accumulator;
        self.accumulator = Instant::now();
        captured
    }

    fn last_frame_time(&mut self) -> Instant {
        self.last_frame_time = Some(self.tick());
        self.last_frame_time.unwrap()
    }

    fn timedelta(&mut self) -> f64 {
        self.last_frame_time();
        self.delta_time = (self.accumulator - self.last_frame_time.unwrap()).as_secs_f64();
        self.delta_time
    }
    pub fn time_delta(&mut self) -> f64 {
        self.timedelta()
    }
}
