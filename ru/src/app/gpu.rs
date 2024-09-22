use wgpu::{Backends, Features, Limits};

#[derive(Clone)]
pub struct GPU {
    pub backends: Backends,
    pub device_features: Features,
    pub device_limits: Limits,
    pub max_samples: u8,
}

impl Default for GPU {
    fn default() -> Self {
        Self {
            backends: Backends::all(),
            device_features: Features::empty(),
            device_limits: Limits::downlevel_webgl2_defaults(),
            max_samples: Default::default(),
        }
    }
}
