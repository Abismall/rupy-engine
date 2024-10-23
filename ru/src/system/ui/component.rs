use crate::{
    math::Vec4,
    model::primitive::Primitive,
    traits::buffers::{IndexBuffer, VertexBuffer},
};

use super::layout::{HorizontalAlign, VerticalAlign};

#[derive(Debug)]
pub struct UIComponent {
    pub num_indices: u32,
    pub color: Vec4,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub grid_position: (usize, usize), // (row, column) in the grid
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
}

impl UIComponent {
    pub fn new(
        device: &wgpu::Device,
        indices: Vec<u16>,
        vertices: Vec<Primitive>,
        color: Vec4,
        grid_position: (usize, usize),
        horizontal_align: HorizontalAlign,
        vertical_align: VerticalAlign,
    ) -> Self {
        let num_indices = indices.len() as u32;
        let vertex_buffer = Primitive::create_static_vertex_buffer(device, &vertices);
        let index_buffer = Primitive::create_static_index_buffer(device, &indices);

        Self {
            num_indices,
            color,
            vertex_buffer,
            index_buffer,
            grid_position,
            horizontal_align,
            vertical_align,
        }
    }
}
