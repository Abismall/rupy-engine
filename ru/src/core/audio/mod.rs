use crate::log_info;

#[derive(Debug)]
pub struct Audio {
    is_muted: bool,
    volume: f32,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            is_muted: false,
            volume: 1.0,
        }
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn mute(&mut self) {
        self.is_muted = true;
        log_info!("Muted");
    }

    pub fn unmute(&mut self) {
        self.is_muted = false;
        log_info!("Unmuted");
    }

    pub fn toggle(&mut self) {
        if self.is_muted {
            self.unmute();
        } else {
            self.mute();
        }
    }

    pub fn volume_up(&mut self) {
        if self.volume < 1.0 {
            self.volume = (self.volume + 0.1).min(1.0);
            log_info!("Volume increased to {:.1}", self.volume);
        } else {
            log_info!("Volume is already at maximum.");
        }
    }

    pub fn volume_down(&mut self) {
        if self.volume > 0.0 {
            self.volume = (self.volume - 0.1).max(0.0);
            log_info!("Volume decreased to {:.1}", self.volume);
        } else {
            log_info!("Volume is already at minimum.");
        }
    }
}
