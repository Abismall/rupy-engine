use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use nalgebra::RealField;

use crate::ecs::model::{Mesh, Vertex2D, Vertex3D};

#[derive(Hash, Debug)]
pub enum VertexType {
    Vertex2D,
    Vertex3D,
}

#[derive(Hash, Debug)]
pub struct MeshKey {
    vertex_type: VertexType,
    vertex_count: usize,
    index_count: usize,
}
impl MeshKey {
    pub fn new<T: std::ops::Sub<Output = T> + Hash + Copy>(
        vertex_type: VertexType,
        vertices: &[T],
        indices: &[u16],
        position: &[f32; 3],
    ) -> u64 {
        let key = MeshKey {
            vertex_type,
            vertex_count: vertices.len(),
            index_count: indices.len(),
        };

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        position
            .iter()
            .for_each(|&val| val.to_bits().hash(&mut hasher));

        vertices.iter().for_each(|vertex| {
            vertex.hash(&mut hasher);
        });

        vertices.windows(2).for_each(|pair| {
            let difference = pair[1] - pair[0];
            difference.hash(&mut hasher);
        });

        indices.hash(&mut hasher);

        hasher.finish()
    }
}
pub struct BoundingSphere {
    pub center: [f32; 3],
    pub radius: f32,
}

impl Mesh<Vertex3D> {
    pub fn calculate_bounding_sphere(&self) -> BoundingSphere {
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];

        // Find the min and max bounds for AABB
        for vertex in &self.vertices {
            for i in 0..3 {
                min[i] = min[i].min(vertex.position[i]);
                max[i] = max[i].max(vertex.position[i]);
            }
        }

        // Center of the bounding sphere
        let center = [
            (min[0] + max[0]) / 2.0,
            (min[1] + max[1]) / 2.0,
            (min[2] + max[2]) / 2.0,
        ];

        // Calculate the radius
        let mut radius = 0.0;
        for vertex in &self.vertices {
            let distance = ((vertex.position[0] - center[0]).powi(2)
                + (vertex.position[1] - center[1]).powi(2)
                + (vertex.position[2] - center[2]).powi(2))
            .sqrt();
            radius = radius.max(distance);
        }

        BoundingSphere { center, radius }
    }
}

impl Mesh<Vertex2D> {
    pub fn calculate_bounding_circle_with_offset(&self) -> BoundingCircle {
        let mut min = [f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN];

        // Find the min and max bounds for AABB
        for vertex in &self.vertices {
            for i in 0..2 {
                min[i] = min[i].min(vertex.position[i]);
                max[i] = max[i].max(vertex.position[i]);
            }
        }

        // Center of the bounding circle
        let center = [(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0];

        // Offset from the origin in local space
        let offset = [
            center[0], // x-offset
            center[1], // y-offset
        ];

        // Calculate the radius
        let mut radius = 0.0;
        for vertex in &self.vertices {
            let distance = ((vertex.position[0] - center[0]).powi(2)
                + (vertex.position[1] - center[1]).powi(2))
            .sqrt();
            radius = radius.max(distance);
        }

        BoundingCircle {
            center,
            offset,
            radius,
        }
    }
}
pub struct BoundingCircle {
    pub center: [f32; 2], // Center of the circle
    pub offset: [f32; 2], // Offset from the origin in local space
    pub radius: f32,      // Radius of the circle
}
