use std::{mem, num::NonZero};

use nalgebra::Matrix4;
use wgpu::BufferBinding;

use crate::graphics::buffer::WgpuBufferBinding;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Default)]
pub struct ColorUniform {
    pub rgba: [f32; 4],
}
impl From<[f32; 4]> for ColorUniform {
    fn from(array: [f32; 4]) -> Self {
        ColorUniform { rgba: array }
    }
}

impl From<ColorUniform> for [f32; 4] {
    fn from(color_uniform: ColorUniform) -> Self {
        color_uniform.rgba
    }
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Default)]
pub struct ViewProjectionMatrix {
    pub matrix: [[f32; 4]; 4],
}
impl From<Matrix4<f32>> for ViewProjectionMatrix {
    fn from(mat: Matrix4<f32>) -> Self {
        ViewProjectionMatrix { matrix: mat.into() }
    }
}

impl From<ViewProjectionMatrix> for Matrix4<f32> {
    fn from(vp: ViewProjectionMatrix) -> Self {
        Matrix4::from(vp.matrix)
    }
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Default)]
pub struct ModelUniform {
    pub matrix: [[f32; 4]; 4],
}
impl From<Matrix4<f32>> for ModelUniform {
    fn from(mat: Matrix4<f32>) -> Self {
        ModelUniform { matrix: mat.into() }
    }
}

impl From<ModelUniform> for Matrix4<f32> {
    fn from(model: ModelUniform) -> Self {
        Matrix4::from(model.matrix)
    }
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Default)]
pub struct Uniforms {
    pub model: ModelUniform,
    pub view_projection: ViewProjectionMatrix,
    pub color: ColorUniform,
}

impl WgpuBufferBinding for Uniforms {
    fn buffer_binding<'a>(buffer: &'a wgpu::Buffer, offset: u64) -> BufferBinding<'a> {
        BufferBinding {
            buffer,
            offset,
            size: NonZero::new(mem::size_of::<Uniforms>() as u64),
        }
    }
}
