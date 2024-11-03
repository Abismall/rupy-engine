use crate::{core::error::AppError, graphics::global::get_device};

pub enum SamplerType {
    Linear,
    Nearest,
    Default,
    Textured,
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
) -> Result<wgpu::Sampler, AppError> {
    let default_sampler_desc = wgpu::SamplerDescriptor::default();
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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
    });
    Ok(sampler)
}
pub fn create_sampler_from_type(sampler_type: SamplerType) -> Result<wgpu::Sampler, AppError> {
    let device = get_device()?;
    let sampler = match sampler_type {
        SamplerType::Linear => setup_sampler(
            &device,
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
        )?,
        SamplerType::Nearest => setup_sampler(
            &device,
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
        )?,

        SamplerType::Default => setup_sampler(
            &device,
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
        )?,
        SamplerType::Textured => device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        }),
    };
    Ok(sampler)
}
