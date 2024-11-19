use nalgebra::{Matrix4, Vector3};

use crate::ecs::model::Vertex2D;

pub struct Sphere {
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u16>,
    pub model_matrix: Matrix4<f32>,
}

impl Sphere {
    pub fn new(
        radius: f32,
        sectors: usize,
        stacks: usize,
        position: Vector3<f32>,
        scale: f32,
    ) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for stack in 0..=stacks {
            let stack_angle =
                std::f32::consts::PI / 2.0 - (stack as f32 * std::f32::consts::PI / stacks as f32);
            let xy = radius * stack_angle.cos();

            for sector in 0..=sectors {
                let sector_angle = 2.0 * std::f32::consts::PI * sector as f32 / sectors as f32;
                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();

                vertices.push(Vertex2D {
                    position: [x, y],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coords: [sector as f32 / sectors as f32, stack as f32 / stacks as f32],
                });
            }
        }

        for stack in 0..stacks {
            let k1 = stack * (sectors + 1);
            let k2 = k1 + sectors + 1;

            for sector in 0..sectors {
                if stack != 0 {
                    indices.push(k1 as u16 + sector as u16);
                    indices.push(k2 as u16 + sector as u16);
                    indices.push(k1 as u16 + sector as u16 + 1);
                }

                if stack != stacks - 1 {
                    indices.push(k1 as u16 + sector as u16 + 1);
                    indices.push(k2 as u16 + sector as u16);
                    indices.push(k2 as u16 + sector as u16 + 1);
                }
            }
        }
        Self {
            vertices,
            indices,
            model_matrix: (Matrix4::new_translation(&position)
                * Matrix4::new_nonuniform_scaling(&Vector3::new(scale, scale, scale)))
            .into(),
        }
    }
}
