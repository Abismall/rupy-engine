pub mod mesh;
pub mod transform;
pub mod vertices;

use std::any::Any;

use wgpu::BindGroup;

use super::model::{Material, Mesh, Transform, Vertex2D, Vertex3D};

pub trait Component: Any + Send + Sync {}

impl<T: Any + Send + Sync> Component for T {}

#[derive(Debug)]
pub struct EntityData {
    pub components: Components,
    pub texture_bind_group: Option<BindGroup>,
}

#[derive(Debug)]

pub enum Components {
    Components2D {
        transform: Option<Transform>,
        material: Option<Material>,
        mesh: Mesh<Vertex2D>,
    },
    Components3D {
        transform: Option<Transform>,
        material: Option<Material>,
        mesh: Mesh<Vertex3D>,
    },
}
