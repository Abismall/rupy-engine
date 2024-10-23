pub enum SamplerType {
    Linear,
    Nearest,
    Default,
}

pub fn setup_sampler(
    device: &wgpu::Device,
    label: Option<String>,
    address_mode_u: Option<wgpu::AddressMode>,
    address_mode_v: Option<wgpu::AddressMode>,
    address_mode_w: Option<wgpu::AddressMode>,
    mag_filter: Option<wgpu::FilterMode>,
    min_filter: Option<wgpu::FilterMode>,
    mipmap_filter: Option<wgpu::FilterMode>,
    lod_min_clamp: Option<f32>,
    lod_max_clamp: Option<f32>,
    anisotropy_clamp: Option<u16>,
    compare: Option<wgpu::CompareFunction>,
    border_color: Option<wgpu::SamplerBorderColor>,
) -> wgpu::Sampler {
    let default_sampler_desc = wgpu::SamplerDescriptor::default();

    device.create_sampler(&wgpu::SamplerDescriptor {
        label: label.as_deref(),
        address_mode_u: address_mode_u.unwrap_or(default_sampler_desc.address_mode_u),
        address_mode_v: address_mode_v.unwrap_or(default_sampler_desc.address_mode_v),
        address_mode_w: address_mode_w.unwrap_or(default_sampler_desc.address_mode_w),
        mag_filter: mag_filter.unwrap_or(default_sampler_desc.mag_filter),
        min_filter: min_filter.unwrap_or(default_sampler_desc.min_filter),
        mipmap_filter: mipmap_filter.unwrap_or(default_sampler_desc.mipmap_filter),
        lod_min_clamp: lod_min_clamp.unwrap_or(default_sampler_desc.lod_min_clamp),
        lod_max_clamp: lod_max_clamp.unwrap_or(default_sampler_desc.lod_max_clamp),
        compare: compare.or(default_sampler_desc.compare),
        anisotropy_clamp: anisotropy_clamp.unwrap_or(default_sampler_desc.anisotropy_clamp),
        border_color: border_color.or(default_sampler_desc.border_color),
    })
}
pub fn create_sampler_from_type(device: &wgpu::Device, sampler_type: SamplerType) -> wgpu::Sampler {
    match sampler_type {
        SamplerType::Linear => setup_sampler(
            device,
            Some("Linear Sampler".to_string()),
            Some(wgpu::AddressMode::ClampToEdge),
            Some(wgpu::AddressMode::ClampToEdge),
            Some(wgpu::AddressMode::ClampToEdge),
            Some(wgpu::FilterMode::Linear),
            Some(wgpu::FilterMode::Linear),
            Some(wgpu::FilterMode::Linear),
            Some(0.0),
            Some(100.0),
            None,
            None,
            None,
        ),
        SamplerType::Nearest => setup_sampler(
            device,
            Some("Linear Sampler".to_string()),
            Some(wgpu::AddressMode::Repeat),
            Some(wgpu::AddressMode::Repeat),
            Some(wgpu::AddressMode::Repeat),
            Some(wgpu::FilterMode::Nearest),
            Some(wgpu::FilterMode::Nearest),
            Some(wgpu::FilterMode::Nearest),
            Some(0.0),
            Some(100.0),
            Some(1),
            None,
            None,
        ),

        SamplerType::Default => setup_sampler(
            device,
            Some("Default Sampler".to_string()),
            Some(wgpu::AddressMode::ClampToEdge),
            Some(wgpu::AddressMode::ClampToEdge),
            Some(wgpu::AddressMode::ClampToEdge),
            Some(wgpu::FilterMode::Linear),
            Some(wgpu::FilterMode::Linear),
            Some(wgpu::FilterMode::Linear),
            Some(0.0),
            Some(100.0),
            Some(1),
            None,
            None,
        ),
    }
}
