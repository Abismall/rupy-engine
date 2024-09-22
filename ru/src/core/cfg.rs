use std::time::Duration;

use crate::graphics::color::Color;

pub(crate) const RELATIVE_CAMERA_SIZE: f32 = 0.5;

#[derive(Clone)]

pub struct GpuConfig {
    pub backends: wgpu::Backends,
    pub device_features: wgpu::Features,
    pub device_limits: wgpu::Limits,
    pub max_samples: u8,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            backends: wgpu::Backends::all(),
            device_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            device_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            max_samples: 4,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScreenConfig {
    pub clear_color: Option<Color>,
    pub max_fps: Option<u32>,
    vsync: bool,
    pub(crate) changed: bool,
}

impl Default for ScreenConfig {
    fn default() -> Self {
        Self {
            clear_color: Some(Color::BLACK),
            max_fps: None,
            vsync: false,
            changed: true,
        }
    }
}

impl ScreenConfig {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub fn clear_color(&self) -> Option<Color> {
        self.clear_color
    }

    pub fn max_fps(&self) -> Option<u32> {
        self.max_fps
    }

    pub fn vsync(&self) -> bool {
        self.vsync
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.changed = true;
        self.vsync = vsync;
    }

    pub fn set_clear_color(&mut self, clear_color: Option<Color>) {
        self.clear_color = clear_color;
    }

    pub fn set_max_fps(&mut self, max_fps: Option<u32>) {
        self.max_fps = max_fps;
    }

    pub fn max_frame_time(&self) -> Option<Duration> {
        if let Some(max_fps) = self.max_fps {
            return Some(Duration::from_secs_f32(1.0 / max_fps as f32));
        }
        None
    }
}
pub struct GPUCreateConfig {
    pub backends: wgpu::Backends,
    pub power_preference: wgpu::PowerPreference,
    pub minimal_required_features: wgpu::Features,
    pub minimal_required_limits: wgpu::Limits,
}

impl Default for GPUCreateConfig {
    fn default() -> Self {
        let mut minimal_required_features = wgpu::Features::all_webgpu_mask();

        #[cfg(target_os = "macos")]
        {
            minimal_required_features.remove(wgpu::Features::TIMESTAMP_QUERY);
        }

        Self {
            backends: wgpu::Backends::all(),
            power_preference: wgpu::PowerPreference::HighPerformance,
            minimal_required_features,
            minimal_required_limits: Default::default(),
        }
    }
}
impl GPUCreateConfig {
    /// Tries to create a device and a surface while ensuring minimal requirements are met.
    /// Returns the created surface if successful, or an error if requirements aren't met.
    pub async fn request_instance(
        &self,
        surface: Option<wgpu::Surface<'static>>,
    ) -> Result<
        (
            wgpu::Device,
            wgpu::Queue,
            wgpu::Adapter,
            Option<wgpu::Surface>,
        ),
        String,
    > {
        // Create a new instance of wgpu::Instance using the backends provided in the config
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: self.backends,
            ..Default::default()
        });

        // Request an adapter that matches the configuration and optional surface
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: self.power_preference,
                compatible_surface: surface.as_ref(),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Ensure the adapter supports the minimal required features and limits
        let _adapter_features = adapter.features();
        let _adapter_limits = adapter.limits();

        // if !_adapter_features.contains(self.minimal_required_features)
        //     | !_adapter_limits.contains(&self.minimal_required_limits)
        // {
        //     return Err(CreateAdapterError::UnableToMeetLimitMinimalRequirement(
        //         _adapter_limits,
        //     ));
        // }

        // Request a device with the required features and limits
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: self.minimal_required_features,
                    required_limits: self.minimal_required_limits.clone(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .unwrap();

        Ok((device, queue, adapter, surface))
    }
}
pub struct AppConfig {
    pub window: winit::window::WindowAttributes,
    pub gpu: GpuConfig,
    pub scene_id: u32,

    #[cfg(feature = "log_feature")]
    pub logger: Option<LogBuilder>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl AppConfig {
    pub const FIRST_SCENE_ID: u32 = 0;
    pub fn new() -> Self {
        AppConfig {
            window: winit::window::WindowAttributes::default()
                .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
                .with_title("App Game"),
            gpu: GpuConfig::default(),
            scene_id: Self::FIRST_SCENE_ID,

            #[cfg(feature = "log_feature")]
            logger: Some(Default::default()),
        }
    }

    pub fn window(mut self, window: winit::window::WindowAttributes) -> Self {
        self.window = window;
        self
    }

    pub fn gpu(mut self, gpu: GpuConfig) -> Self {
        self.gpu = gpu;
        self
    }

    pub fn scene_id(mut self, scene_id: u32) -> Self {
        self.scene_id = scene_id;
        self
    }

    // pub fn storage(mut self, storage: impl StorageManager) -> Self {
    //     self.storage = Arc::new(storage);
    //     self
    // }

    // pub fn assets(mut self, assets: impl AssetManager) -> Self {
    //     self.assets = Arc::new(assets);
    //     self
    // }

    #[cfg(feature = "log_feature")]
    pub fn logger(mut self, logger: Option<LogBuilder>) -> Self {
        self.logger = logger;
        self
    }
}
