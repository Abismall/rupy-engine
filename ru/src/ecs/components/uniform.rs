use std::{mem, num::NonZero};

use bytemuck::{Pod, Zeroable};
use nalgebra::{Matrix4, Vector3};
use serde::{Deserialize, Serialize};
use wgpu::BufferBinding;

use crate::graphics::buffer::WgpuBufferBinding;
#[repr(C)]
#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize, Pod, Zeroable)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [[f32; 4]; 4],
    pub scale: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug, Serialize, Deserialize)]
pub struct UniformColor {
    pub rgba: [f32; 4],
}
impl Default for UniformColor {
    fn default() -> Self {
        Self {
            rgba: [1.0, 1.0, 1.0, 1.0],
        }
    }
}
impl WgpuBufferBinding for UniformColor {
    fn buffer_binding<'a>(buffer: &'a wgpu::Buffer, offset: u64) -> BufferBinding<'a> {
        BufferBinding {
            buffer,
            offset,
            size: NonZero::new(mem::size_of::<UniformColor>() as u64),
        }
    }
}
impl From<[f32; 4]> for UniformColor {
    fn from(array: [f32; 4]) -> Self {
        UniformColor { rgba: array }
    }
}

impl From<UniformColor> for [f32; 4] {
    fn from(color_uniform: UniformColor) -> Self {
        color_uniform.rgba
    }
}
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug, Default, Serialize, Deserialize)]
pub struct ViewProjection {
    pub matrix: [[f32; 4]; 4],
}
impl From<Matrix4<f32>> for ViewProjection {
    fn from(mat: Matrix4<f32>) -> Self {
        ViewProjection { matrix: mat.into() }
    }
}

impl From<ViewProjection> for Matrix4<f32> {
    fn from(vp: ViewProjection) -> Self {
        Matrix4::from(vp.matrix)
    }
}

impl WgpuBufferBinding for ViewProjection {
    fn buffer_binding<'a>(buffer: &'a wgpu::Buffer, offset: u64) -> BufferBinding<'a> {
        BufferBinding {
            buffer,
            offset,
            size: NonZero::new(mem::size_of::<ViewProjection>() as u64),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug, Default, Serialize, Deserialize)]
pub struct UniformModel {
    matrix: [[f32; 4]; 4],
}
impl UniformModel {
    pub fn matrix(&self) -> [[f32; 4]; 4] {
        self.matrix
    }
}
impl From<Matrix4<f32>> for UniformModel {
    fn from(mat: Matrix4<f32>) -> Self {
        UniformModel { matrix: mat.into() }
    }
}

impl From<UniformModel> for Matrix4<f32> {
    fn from(model: UniformModel) -> Self {
        Matrix4::from(model.matrix)
    }
}
impl WgpuBufferBinding for UniformModel {
    fn buffer_binding<'a>(buffer: &'a wgpu::Buffer, offset: u64) -> BufferBinding<'a> {
        BufferBinding {
            buffer,
            offset,
            size: NonZero::new(mem::size_of::<UniformModel>() as u64),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug, Default, Serialize, Deserialize)]
pub struct Uniforms {
    pub model: UniformModel,
    pub view_projection: ViewProjection,
    pub color: UniformColor,
}
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug, Default, Serialize, Deserialize)]
pub struct UniformTransform {
    pub scale: [[f32; 4]; 4],
    pub rotation: [[f32; 4]; 4],
    pub position: [f32; 3],
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
impl From<[[f32; 4]; 4]> for UniformModel {
    fn from(matrix: [[f32; 4]; 4]) -> Self {
        UniformModel { matrix }
    }
}
