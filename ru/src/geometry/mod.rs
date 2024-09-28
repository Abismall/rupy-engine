pub(crate) mod cube;
pub(crate) mod hexagon;
pub(crate) mod rectangle;
pub(crate) mod spatial;
pub(crate) mod sphere;
pub(crate) mod triangle;
use cube::CubeStructure;
use nalgebra::Matrix4;
use rectangle::RectangleStructure;
use sphere::SphereStructure;
use triangle::TriangleStructure;
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Uniforms {
    pub model: [[f32; 4]; 4],     // Model matrix
    pub view_proj: [[f32; 4]; 4], // View-projection matrix
    pub color: [f32; 4],          // RGBA color
}

#[derive(Debug, Clone)]
pub enum Shape {
    Triangle(TriangleStructure),
    Cube(CubeStructure),
    Rectangle(RectangleStructure),
    Sphere(SphereStructure),
}

impl Shape {
    /// Returns the vertex buffer data as a byte slice for the current geometry
    pub fn vertex_buffer_data(&self) -> &[u8] {
        match self {
            Shape::Triangle(triangle) => bytemuck::cast_slice(triangle.vertices()),
            Shape::Cube(cube) => bytemuck::cast_slice(cube.vertices()),
            Shape::Rectangle(rectangle) => bytemuck::cast_slice(rectangle.vertices()),
            Shape::Sphere(sphere) => bytemuck::cast_slice(sphere.vertices()),
        }
    }
    pub fn set_vertex_buffer(&mut self, buffer: Vec<Vertex>) {
        match self {
            Shape::Triangle(triangle) => triangle.vertices = buffer,
            Shape::Cube(cube) => cube.vertices = buffer,
            Shape::Rectangle(rectangle) => rectangle.vertices = buffer,
            Shape::Sphere(sphere) => sphere.vertices = buffer,
        }
    }
    pub fn set_index_buffer(&mut self, buffer: Vec<u32>) {
        match self {
            Shape::Triangle(triangle) => triangle.indices = buffer,
            Shape::Cube(cube) => cube.indices = buffer,
            Shape::Rectangle(rectangle) => rectangle.indices = buffer,
            Shape::Sphere(sphere) => sphere.indices = buffer,
        }
    }
    pub fn vertex_buffer(&self) -> Vec<Vertex> {
        match self {
            Shape::Triangle(triangle) => triangle.vertices.clone(),
            Shape::Cube(cube) => cube.vertices.clone(),
            Shape::Rectangle(rectangle) => rectangle.vertices.clone(),
            Shape::Sphere(sphere) => sphere.vertices.clone(),
        }
    }
    pub fn index_buffer(&self) -> Vec<u32> {
        match self {
            Shape::Triangle(triangle) => triangle.indices.clone(),
            Shape::Cube(cube) => cube.indices.clone(),
            Shape::Rectangle(rectangle) => rectangle.indices.clone(),
            Shape::Sphere(sphere) => sphere.indices.clone(),
        }
    }
    /// Returns the index buffer data as a byte slice for the current geometry
    pub fn index_buffer_data(&self) -> &[u8] {
        match self {
            Shape::Triangle(triangle) => bytemuck::cast_slice(triangle.indices()),
            Shape::Cube(cube) => bytemuck::cast_slice(cube.indices()),
            Shape::Rectangle(rectangle) => bytemuck::cast_slice(rectangle.indices()),
            Shape::Sphere(sphere) => bytemuck::cast_slice(sphere.indices()),
        }
    }

    /// Returns the number of indices in the current geometry
    pub fn num_indices(&self) -> u32 {
        match self {
            Shape::Triangle(triangle) => triangle.indices().len() as u32,
            Shape::Cube(cube) => cube.indices().len() as u32,
            Shape::Rectangle(rectangle) => rectangle.indices().len() as u32,
            Shape::Sphere(sphere) => sphere.indices().len() as u32,
        }
    }

    /// Returns the model matrix of the current geometry
    pub fn model_matrix(&self) -> Matrix4<f32> {
        match self {
            Shape::Triangle(triangle) => triangle.model_matrix(),
            Shape::Cube(cube) => cube.model_matrix(),
            Shape::Rectangle(rectangle) => rectangle.model_matrix(),
            Shape::Sphere(sphere) => sphere.model_matrix(),
        }
    }

    /// Returns whether the object is textured
    pub fn is_textured(&self) -> bool {
        match self {
            Shape::Triangle(triangle) => triangle.is_textured(),
            Shape::Cube(cube) => cube.is_textured(),
            Shape::Rectangle(rectangle) => rectangle.is_textured(),
            Shape::Sphere(sphere) => sphere.is_textured(),
        }
    }

    /// Optionally handle texture updates if the object is textured
    pub fn update_texture(&self, queue: &wgpu::Queue) {
        match self {
            Shape::Triangle(triangle) => triangle.update_texture(queue),
            Shape::Cube(cube) => cube.update_texture(queue),
            Shape::Rectangle(rectangle) => rectangle.update_texture(queue),
            Shape::Sphere(sphere) => sphere.update_texture(queue),
        }
    }

    /// Update any internal state before rendering (e.g., animations)
    pub fn update(&mut self) {
        match self {
            Shape::Triangle(triangle) => triangle.update(),
            Shape::Cube(cube) => cube.update(),
            Shape::Rectangle(rectangle) => rectangle.update(),
            Shape::Sphere(sphere) => sphere.update(),
        }
    }
}

pub trait Renderable {
    type VertexType; // Associated type for vertex type

    // Update any internal state before rendering (e.g., animations)
    fn update(&mut self);

    // Return the object's model matrix
    fn model_matrix(&self) -> Matrix4<f32>;

    // Return the vertices for rendering
    fn vertices(&self) -> &[Self::VertexType];

    // Return the indices for rendering
    fn indices(&self) -> &[u32];

    // Optionally, handle texture updates if the renderable uses textures
    fn update_texture(&self, _queue: &wgpu::Queue) {
        // Default implementation does nothing; objects can override this if they use textures
    }

    // Optionally, return whether the object supports textures
    fn is_textured(&self) -> bool {
        false
    }
}
use crate::material::vertex::{TexturedVertex, Vertex};

pub enum RenderableObject {
    Shaded {
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    Textured {
        vertices: Vec<TexturedVertex>,
        indices: Vec<u32>,
    },
}

impl Renderable for RenderableObject {
    type VertexType = Vertex;

    fn update(&mut self) {
        // Update logic if needed (e.g., animations, transformations)
    }

    fn model_matrix(&self) -> Matrix4<f32> {
        Matrix4::identity()
    }

    fn vertices(&self) -> &[Self::VertexType] {
        match self {
            RenderableObject::Shaded { vertices, .. } => vertices,
            RenderableObject::Textured { .. } => {
                panic!("Textured objects use a different vertex type!")
            }
        }
    }

    fn indices(&self) -> &[u32] {
        match self {
            RenderableObject::Shaded { indices, .. } => indices,
            RenderableObject::Textured { indices, .. } => indices,
        }
    }

    // Override to support texture updates for textured objects
    fn update_texture(&self, queue: &wgpu::Queue) {
        match self {
            RenderableObject::Textured { .. } => {
                // Handle texture updates if necessary
            }
            _ => {}
        }
    }

    fn is_textured(&self) -> bool {
        matches!(self, RenderableObject::Textured { .. })
    }
}
