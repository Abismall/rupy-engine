use core::fmt;

use rand::Rng;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use wgpu::{BindGroup, Buffer};

use crate::{
    ecs::components::{
        material::Material,
        model::{Mesh, Transform, Vertex2D, Vertex3D},
    },
    log_debug,
    prelude::helpers::string_to_u64,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneData {
    pub name: String,

    #[serde(deserialize_with = "deserialize_flexible_entities")]
    pub entities: Vec<EntityData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityData {
    #[serde(flatten)]
    pub components: Components,

    #[serde(skip)]
    pub uniform_bind_group: Option<BindGroup>,

    #[serde(skip)]
    pub texture_bind_group: Option<BindGroup>,

    #[serde(skip)]
    pub uniform_buffer: Option<Buffer>,
}

fn deserialize_flexible_entities<'de, D>(deserializer: D) -> Result<Vec<EntityData>, D::Error>
where
    D: Deserializer<'de>,
{
    struct EntitiesVisitor;

    impl<'de> Visitor<'de> for EntitiesVisitor {
        type Value = Vec<EntityData>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array or a single entity object")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut entities = Vec::new();
            while let Some(entity) = seq.next_element()? {
                entities.push(entity);
            }
            Ok(entities)
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let entity = EntityData::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(vec![entity])
        }
    }

    deserializer.deserialize_any(EntitiesVisitor)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Components {
    Components2D {
        transform: Option<Transform>,
        material: Option<Material>,
        vertices: Vec<Vertex2D>,
        indices: Vec<u16>,
    },
    Components3D {
        transform: Option<Transform>,
        material: Option<Material>,
        vertices: Vec<Vertex3D>,
        indices: Vec<u16>,
    },
}

impl EntityData {
    pub fn new_2d(
        transform: Option<Transform>,
        material: Option<Material>,
        vertices: Vec<Vertex2D>,
        indices: Vec<u16>,
    ) -> Self {
        let material = if let Some(mat) = material {
            Some(Material {
                color: mat.color,
                texture_id: match mat.texture_id {
                    Some(id) => Some(id),
                    None => None,
                },
            })
        } else {
            None
        };
        Self {
            components: Components::Components2D {
                transform,
                material,
                vertices,
                indices,
            },
            uniform_bind_group: None,
            texture_bind_group: None,
            uniform_buffer: None,
        }
    }

    pub fn new_3d(
        transform: Option<Transform>,
        material: Option<Material>,
        vertices: Vec<Vertex3D>,
        indices: Vec<u16>,
    ) -> Self {
        let material = if let Some(mat) = material {
            Some(Material {
                color: mat.color,
                texture_id: match mat.texture_id {
                    Some(id) => Some(id),
                    None => None,
                },
            })
        } else {
            None
        };
        Self {
            components: Components::Components3D {
                transform,
                material,
                vertices,
                indices,
            },
            uniform_bind_group: None,
            texture_bind_group: None,
            uniform_buffer: None,
        }
    }
}

impl<'de> Deserialize<'de> for Mesh<Vertex2D> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MeshData {
            vertices: Vec<Vertex2D>,
            indices: Vec<u16>,
        }

        let MeshData { vertices, indices } = MeshData::deserialize(deserializer)?;
        Ok(Mesh { vertices, indices })
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

impl<'de> Deserialize<'de> for Mesh<Vertex3D> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MeshData {
            vertices: Vec<Vertex3D>,
            indices: Vec<u16>,
        }

        let MeshData { vertices, indices } = MeshData::deserialize(deserializer)?;
        Ok(Mesh { vertices, indices })
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialData {
    pub color: [f32; 4],
    pub texture: Option<String>,
}

pub fn create_detailed_cube_scene() -> SceneData {
    let mut entities = Vec::new();
    let mut rng = rand::thread_rng();

    let mut create_random_color = || [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];

    let create_transform = |x, y, z| Transform {
        position: [x, y, z],
        scale: [1.0, 1.0, 1.0],
        rotation: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    let vertices = vec![
        Vertex3D {
            position: [-1.0, -1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.0, 0.0],
            color: create_random_color(),
        },
        Vertex3D {
            position: [1.0, -1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [1.0, 0.0],
            color: create_random_color(),
        },
        Vertex3D {
            position: [1.0, 1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [1.0, 1.0],
            color: create_random_color(),
        },
        Vertex3D {
            position: [-1.0, 1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.0, 1.0],
            color: create_random_color(),
        },
        Vertex3D {
            position: [-1.0, -1.0, -1.0],
            normal: [0.0, 0.0, -1.0], // Back face normal
            tex_coords: [1.0, 0.0],
            color: create_random_color(),
        },
        Vertex3D {
            position: [1.0, -1.0, -1.0],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [0.0, 0.0],
            color: create_random_color(),
        },
        Vertex3D {
            position: [1.0, 1.0, -1.0],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [0.0, 1.0],
            color: create_random_color(),
        },
        Vertex3D {
            position: [-1.0, 1.0, -1.0],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [1.0, 1.0],
            color: create_random_color(),
        },
    ];

    let indices = vec![
        0, 1, 2, 0, 2, 3, // Front face
        4, 5, 6, 4, 6, 7, // Back face
        0, 3, 7, 0, 7, 4, // Left face
        1, 2, 6, 1, 6, 5, // Right face
        3, 2, 6, 3, 6, 7, // Top face
        0, 1, 5, 0, 5, 4, // Bottom face
    ];

    let cube_positions = vec![[0.0, 0.0, 0.0]];

    for pos in cube_positions {
        let transform = Some(create_transform(pos[0], pos[1], pos[2]));

        let color = create_random_color();
        let material = Some(Material {
            color,
            texture_id: Some(14402189752926126668),
        });

        let entity = EntityData::new_3d(transform, material, vertices.clone(), indices.clone());
        entities.push(entity);
    }

    SceneData {
        name: "detailed_cube_scene".to_string(),
        entities,
    }
}
