use crate::graphics::{
    binding::{hdr::create_hdr_pipeline_bind_group, BindGroupLayouts},
    shaders::manager::ShaderManager,
    textures::{cube_texture::CubeTexture, BindableTexture, Texture},
};
use image::codecs::hdr::HdrDecoder;
use std::io::Cursor;
use wgpu::TextureFormat;
use winit::dpi::PhysicalSize;

use crate::core::error::AppError;

use super::common::{create_render_pipeline, PipelineBase};

pub struct HdrPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    texture: Texture,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
}

impl PipelineBase for HdrPipeline {
    fn create_layout(
        device: &wgpu::Device,
        bind_group_layouts: &BindGroupLayouts,
    ) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("hdr_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layouts.hdr_pipeline_bind_group_layout],
            push_constant_ranges: &[],
        })
    }

    fn new(
        device: &wgpu::Device,
        format: TextureFormat,
        width: u32,
        height: u32,
        shader_manager: &mut ShaderManager,
        bind_group_layouts: &BindGroupLayouts,
    ) -> Self {
        let layout = Self::create_layout(device, &bind_group_layouts);

        let pipeline = create_render_pipeline(
            device,
            &layout,
            format.add_srgb_suffix(),
            None,
            &[],
            wgpu::PrimitiveTopology::TriangleList,
            "effects/tone_mapping.wgsl",
            shader_manager,
        )
        .expect("Hdr pipeline");

        let texture = Texture::create_2d_texture(
            device,
            width,
            height,
            format,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            wgpu::FilterMode::Nearest,
            Some("hdr_texture"),
        );

        let bind_group = create_hdr_pipeline_bind_group(
            device,
            &bind_group_layouts.hdr_pipeline_bind_group_layout,
            &texture,
        );

        Self {
            pipeline,
            texture,
            width,
            height,
            format,
            bind_group,
        }
    }

    fn resize<P: winit::dpi::Pixel>(
        &mut self,
        device: &wgpu::Device,
        new_size: PhysicalSize<P>,
        bind_group_layouts: &BindGroupLayouts,
    ) {
        self.width = new_size.width.cast();
        self.height = new_size.height.cast();

        self.texture = Texture::create_2d_texture(
            device,
            self.width,
            self.height,
            self.format,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            wgpu::FilterMode::Nearest,
            Some("hdr_texture"),
        );

        self.bind_group = create_hdr_pipeline_bind_group(
            device,
            &bind_group_layouts.hdr_pipeline_bind_group_layout,
            &self.texture,
        );
    }

    fn view(&self) -> &wgpu::TextureView {
        &self.texture.view
    }

    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    fn process(&self, encoder: &mut wgpu::CommandEncoder, output: &wgpu::TextureView) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Hdr::process"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}
pub struct HdrLoader {
    texture_format: wgpu::TextureFormat,
    equirect_layout: wgpu::BindGroupLayout,
    equirect_to_cubemap: wgpu::ComputePipeline,
}

impl HdrLoader {
    pub fn new(device: &wgpu::Device) -> Self {
        let module = device.create_shader_module(wgpu::include_wgsl!(
            "../../assets/shaders/compute/equirectangular.wgsl"
        ));
        let texture_format = wgpu::TextureFormat::Rgba32Float;
        let equirect_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("HdrLoader::equirect_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: texture_format,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&equirect_layout],
            push_constant_ranges: &[],
        });

        let equirect_to_cubemap =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("equirect_to_cubemap"),
                layout: Some(&pipeline_layout),
                module: &module,
                entry_point: "compute_equirect_to_cubemap",
                compilation_options: Default::default(),
                cache: None,
            });

        Self {
            equirect_to_cubemap,
            texture_format,
            equirect_layout,
        }
    }

    pub fn from_equirectangular_bytes(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[u8],
        dst_size: u32,
        label: Option<&str>,
    ) -> Result<CubeTexture, AppError> {
        let hdr_decoder = HdrDecoder::new(Cursor::new(data))?;
        let meta = hdr_decoder.metadata();

        #[cfg(not(target_arch = "wasm32"))]
        let pixels = {
            let mut pixels = vec![[0.0, 0.0, 0.0, 0.0]; meta.width as usize * meta.height as usize];
            hdr_decoder.read_image_transform(
                |pix| {
                    let rgb = pix.to_hdr();
                    [rgb.0[0], rgb.0[1], rgb.0[2], 1.0f32]
                },
                &mut pixels[..],
            )?;
            pixels
        };

        let src = Texture::create_2d_texture(
            device,
            meta.width,
            meta.height,
            self.texture_format,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            wgpu::FilterMode::Linear,
            None,
        );

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &src.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &bytemuck::cast_slice(&pixels),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(src.size.width * std::mem::size_of::<[f32; 4]>() as u32),
                rows_per_image: Some(src.size.height),
            },
            src.size,
        );

        let dst = CubeTexture::create_2d(
            device,
            dst_size,
            dst_size,
            self.texture_format,
            1,
            wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            wgpu::FilterMode::Nearest,
            label,
        );

        let dst_view = dst.texture().create_view(&wgpu::TextureViewDescriptor {
            label,

            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &self.equirect_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&src.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&dst_view),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label,
            timestamp_writes: None,
        });

        let num_workgroups = (dst_size + 15) / 16;
        pass.set_pipeline(&self.equirect_to_cubemap);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(num_workgroups, num_workgroups, 6);

        drop(pass);

        queue.submit([encoder.finish()]);

        Ok(dst)
    }
}
