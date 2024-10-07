use crate::{
    log_debug, log_info,
    prelude::{EventBusProxy, RupyAppEvent},
};
use std::sync::{Arc, Mutex};

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

    pub fn subscribe_to_events(
        audio: Arc<Mutex<Audio>>,
        event_bus: &mut EventBusProxy<RupyAppEvent>,
    ) {
        let audio_toggle = Arc::clone(&audio);

        event_bus.subscribe("audio_toggle", move |event| {
            if let RupyAppEvent::ToggleAudio = event {
                if let Ok(mut audio) = audio_toggle.lock() {
                    audio.toggle();
                } else {
                    log_debug!("Failed to lock audio instance.");
                }
            }
        });
        let audio_up = Arc::clone(&audio);
        event_bus.subscribe("audio_volume_up", move |event| {
            if let RupyAppEvent::VolumeUp = event {
                if let Ok(mut audio) = audio_up.lock() {
                    audio.volume_up();
                } else {
                    log_debug!("Failed to lock audio instance.");
                }
            }
        });

        let audio_down = Arc::clone(&audio);
        event_bus.subscribe("audio_volume_down", move |event| {
            if let RupyAppEvent::VolumeDown = event {
                if let Ok(mut audio) = audio_down.clone().lock() {
                    audio.volume_down();
                } else {
                    log_debug!("Failed to lock audio instance.");
                }
            }
        });
    }
}
