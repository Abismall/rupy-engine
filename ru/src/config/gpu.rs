#[derive(Clone)]
pub struct GpuConfig {
    pub backends: wgpu::Backends,
    pub device_features: wgpu::Features,
    pub device_limits: wgpu::Limits,
    pub max_samples: u8,
}

impl Default for GpuConfig {
    fn default() -> Self {
        let mut features = wgpu::Features::empty();
        features.insert(wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES);
        features.insert(wgpu::Features::POLYGON_MODE_LINE);

        Self {
            backends: wgpu::Backends::all(),
            device_features: features,
            device_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            max_samples: 4,
        }
    }
}

impl GpuConfig {
    pub fn with_custom_samples(mut self, samples: u8) -> Self {
        self.max_samples = samples;
        self
    }

    pub fn enable_feature(mut self, feature: wgpu::Features) -> Self {
        self.device_features.insert(feature);
        self
    }

    pub fn set_limits(mut self, limits: wgpu::Limits) -> Self {
        self.device_limits = limits;
        self
    }
}
