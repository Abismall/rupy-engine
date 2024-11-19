use nalgebra::{Matrix4, Quaternion, Vector3};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use wgpu::TextureViewDescriptor;

use crate::{
    ecs::{
        components::{
            mesh::{MeshKey, VertexType},
            Components, EntityData,
        },
        geometry::{cube::Cube3D, plane::Plane3D},
        model::{Material, Mesh, Transform, Vertex2D, Vertex3D},
    },
    gpu::binding::{
        groups::sampled_texture_bind_group, layouts::sampled_texture_bind_group_layout,
    },
    log_debug,
    texture::manager::TextureManager,
};

#[derive(Debug)]
pub struct SceneData {
    pub name: String,
    pub entities: Vec<EntityData>,
}

impl EntityData {
    pub fn new_2d(
        device: &wgpu::Device,
        texture_manager: &TextureManager,
        transform: Option<Transform>,
        material: Option<Material>,
        vertices: Vec<Vertex2D>,
        indices: Vec<u16>,
    ) -> Self {
        let mut texture_bind_group: Option<wgpu::BindGroup> = None;
        let position = match transform {
            Some(value) => value.position,
            None => Vector3::identity().into(),
        };
        let mesh_key = MeshKey::new(VertexType::Vertex2D, &vertices, &indices, &position);
        let mesh = Mesh::<Vertex2D> {
            id: mesh_key,
            vertices,
            indices,
        };
        let material = if let Some(mat) = material {
            Some(Material {
                texture_id: match mat.texture_id {
                    Some(id) => {
                        if let Some(cached_texture) = texture_manager.get_texture(id) {
                            let layout = sampled_texture_bind_group_layout(device);
                            let texture_view = cached_texture
                                .texture
                                .create_view(&TextureViewDescriptor::default());
                            texture_bind_group = Some(sampled_texture_bind_group(
                                device,
                                &layout,
                                &texture_view,
                                &cached_texture.sampler,
                            ));
                        }

                        Some(id)
                    }
                    None => None,
                },
                color: mat.color,
                shininess: mat.shininess,
                ambient_strength: mat.ambient_strength,
                diffuse_strength: mat.diffuse_strength,
                specular_strength: mat.specular_strength,
            })
        } else {
            None
        };
        Self {
            components: Components::Components2D {
                transform,
                material,
                mesh,
            },
            texture_bind_group,
        }
    }

    pub fn new_3d(
        transform: Option<Transform>,
        material: Option<Material>,
        vertices: Vec<Vertex3D>,
        indices: Vec<u16>,
    ) -> Self {
        let position = match transform {
            Some(value) => value.position,
            None => Vector3::identity().into(),
        };
        let mesh_key = MeshKey::new(VertexType::Vertex3D, &vertices, &indices, &position);
        let mesh = Mesh {
            id: mesh_key,
            vertices,
            indices,
        };
        let material = if let Some(mat) = material {
            Some(Material {
                texture_id: match mat.texture_id {
                    Some(id) => Some(id),
                    None => None,
                },
                color: [position[0] * 1.0, position[1] * 1.0, position[2] * 1.0, 1.0],
                shininess: mat.shininess,
                ambient_strength: mat.ambient_strength,
                diffuse_strength: mat.diffuse_strength,
                specular_strength: mat.specular_strength,
            })
        } else {
            None
        };
        Self {
            components: Components::Components3D {
                transform,
                material,

                mesh,
            },
            texture_bind_group: None,
        }
    }
}

impl Serialize for Mesh<Vertex2D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Mesh", 2)?;
        state.serialize_field("vertices", &self.vertices)?;
        state.serialize_field("indices", &self.indices)?;
        state.end()
    }
}

impl Serialize for Mesh<Vertex3D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Mesh", 2)?;
        state.serialize_field("vertices", &self.vertices)?;
        state.serialize_field("indices", &self.indices)?;
        state.end()
    }
}

pub fn create_detailed_cube_scene() -> SceneData {
    let mut entities = Vec::new();

    let create_transform = |x, y, z| Transform {
        position: Vector3::from([x, y, z]).into(),
        scale: [1.0, 1.0, 1.0],
        rotation: Quaternion::new(1.0, 1.0, 1.0, 1.0),
        velocity: [0.0, 0.0, 0.0],
    };

    let mut cube_positions = Vec::new();
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                cube_positions.push([x as f32 * 1.0, y as f32 * 1.0, z as f32 * 1.0]);
            }
        }
    }
    for pos in cube_positions {
        let transform = Some(create_transform(pos[0] * 2.0, pos[1] * 2.0, pos[2] * 2.0));
        let cube = Cube3D::new(
            1.0,
            Vector3::new(pos[0] * 2.0, pos[1] * 2.0, pos[2] * 2.0),
            None,
        );
        let material = Some(Material {
            texture_id: Some(14402189752926126668),
            color: [0.0, 1.0, 1.0, 1.0],
            shininess: Some(32.0),
            ambient_strength: Some(0.8),
            diffuse_strength: Some(0.6),
            specular_strength: Some(1.0),
        });

        let entity = EntityData::new_3d(
            transform,
            material,
            cube.vertices.into(),
            cube.indices.into(),
        );
        entities.push(entity);
    }

    SceneData {
        name: "detailed_cube_scene".to_string(),
        entities,
    }
}

pub fn create_detailed_plane_scene() -> SceneData {
    let mut entities = Vec::new();

    let create_transform = |x, y, z| Transform {
        position: [x, y, z].into(),
        scale: [1.0, 1.0, 1.0],
        rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
        velocity: [0.0, 0.0, 1.0],
    };

    let plane = Plane3D::new(10.0, 10.0, 1.0);
    let plane_positions = vec![[0.0, -1.0, 0.0]];

    for pos in plane_positions {
        let transform = Some(create_transform(pos[0], pos[1], pos[2]));

        let material = Some(Material {
            texture_id: Some(14402189752926126668),
            color: [1.0, 1.0, 1.0, 1.0],
            shininess: Some(32.0),
            ambient_strength: Some(0.8),
            diffuse_strength: Some(0.6),
            specular_strength: Some(1.0),
        });
        let entity = EntityData::new_3d(
            transform,
            material,
            plane.vertices.clone(),
            plane.indices.clone(),
        );
        entities.push(entity);
    }
    let cube_positions = vec![[1.0, 1.0, 0.0, 5.0, 5.0, 0.0]];

    for pos in cube_positions {
        let transform = Some(create_transform(15.0, 15.0, pos[2]));
        let cube = Cube3D::new(1.0, Vector3::new(pos[0], pos[1], pos[2]), None);
        let material = Some(Material {
            texture_id: Some(14402189752926126668),
            color: [pos[0] * 1.0, pos[0] * 1.0, pos[0] * 1.0, 1.0],
            shininess: Some(32.0),
            ambient_strength: Some(0.8),
            diffuse_strength: Some(0.6),
            specular_strength: Some(1.0),
        });

        let entity = EntityData::new_3d(
            transform,
            material,
            cube.vertices.into(),
            cube.indices.into(),
        );
        entities.push(entity);
    }
    SceneData {
        name: "detailed_plane_scene".to_string(),
        entities,
    }
}
