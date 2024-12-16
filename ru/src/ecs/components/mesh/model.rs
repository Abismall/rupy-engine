use crate::{
    core::cache::{CacheKey, HasCacheKey, HashCache},
    ecs::{
        components::{IndexData, VertexData},
        systems::render::BufferFactory,
        traits::{BufferCreator, Cache},
    },
    graphics::vertex::{ModelVertex, VertexType},
};

#[derive(Debug, Clone)]
pub struct Mesh {
    pub num_elements: u32,
    pub material: Option<usize>,
    pub vertices: Vec<VertexType>,
    pub indices: Vec<u32>,
    pub cache_key: CacheKey,
    pub vertex_buffer_key: CacheKey,
    pub index_buffer_key: CacheKey,
}
impl Mesh {
    pub fn new(
        cache_key: CacheKey,
        material: Option<usize>,
        vertices: Vec<VertexType>,
        indices: Vec<u32>,
    ) -> Mesh {
        use cgmath::num_traits::ToPrimitive;
        let cache_key_string = cache_key.0.to_string();
        let num_elements = indices.len().to_u32().unwrap_or(0);

        let vertex_buffer_key = Mesh::new_vertex_cache_key(&cache_key_string);
        let index_buffer_key = Mesh::new_index_cache_key(&cache_key_string);

        Self {
            num_elements,
            material,
            vertices,
            indices,
            cache_key,
            vertex_buffer_key,
            index_buffer_key,
        }
    }

    pub fn generate_vertices(
        positions: &Vec<f32>,
        texcoords: &Vec<f32>,
        normals: &Vec<f32>,
        chunk_size: usize,
    ) -> Vec<VertexType> {
        let v_range = Mesh::chunk_size_range(&positions, chunk_size);
        let mut vertices = Vec::with_capacity(v_range.end);
        let tangent = [0.0; 3];
        let bitangent = [0.0; 3];

        fn vertex(
            position: [f32; 3],
            tex_coords: [f32; 2],
            normal: [f32; 3],
            tangent: [f32; 3],
            bitangent: [f32; 3],
        ) -> VertexType {
            VertexType::Modeled(ModelVertex {
                position,
                tex_coords,
                normal,
                tangent,
                bitangent,
            })
        }

        for index in 0..vertices.capacity() {
            vertices.push(vertex(
                [
                    positions[index * 3],
                    positions[index * 3 + 1],
                    positions[index * 3 + 2],
                ],
                [texcoords[index * 2], 1.0 - texcoords[index * 2 + 1]],
                [
                    normals[index * 3],
                    normals[index * 3 + 1],
                    normals[index * 3 + 2],
                ],
                tangent,
                bitangent,
            ));
        }

        vertices
    }

    pub fn vertex_flat_map(&self) -> Vec<u8> {
        self.vertices
            .iter()
            .flat_map(|vertex| vertex.as_pod())
            .collect()
    }
}

impl Mesh {
    pub fn from_tobj_mesh_with_material(mesh: tobj::Mesh, name: String) -> Mesh {
        let chunk_size: usize = 3;
        let indices = mesh.indices;
        let num_elements = indices.len() as u32;
        let material = mesh.material_id;
        let cache_key = Mesh::key(vec![&name, &material.unwrap_or(0).to_string()]);
        let cache_key_string = cache_key.0.to_string();
        let vertex_buffer_key = Mesh::new_vertex_cache_key(&cache_key_string);
        let index_buffer_key = Mesh::new_index_cache_key(&cache_key_string);

        let vertices =
            Mesh::generate_vertices(&mesh.positions, &mesh.texcoords, &mesh.normals, chunk_size);

        Mesh {
            num_elements,
            material,
            vertices,
            indices,
            cache_key,
            vertex_buffer_key,
            index_buffer_key,
        }
    }

    pub fn new_vertex_cache_key(base: &str) -> CacheKey {
        Mesh::key(vec![base, "vertex", "buffer"])
    }
    pub fn new_index_cache_key(base: &str) -> CacheKey {
        Mesh::key(vec![base, "index", "buffer"])
    }

    pub fn generate_tangent_space(
        vertices: &mut Vec<VertexType>,
        indices: &[u32],
        chunk_size: usize,
    ) {
        use cgmath::InnerSpace;
        let mut tangents = vec![cgmath::Vector3::new(0.0, 0.0, 0.0); vertices.len()];
        let mut bitangents = vec![cgmath::Vector3::new(0.0, 0.0, 0.0); vertices.len()];
        let mut triangles_included = vec![0; vertices.len()];

        for chunk in indices.chunks_exact(chunk_size) {
            let [i0, i1, i2] = [chunk[0] as usize, chunk[1] as usize, chunk[2] as usize];
            let (v0, v1, v2) = (&vertices[i0], &vertices[i1], &vertices[i2]);

            if let (
                VertexType::Modeled(ref v0),
                VertexType::Modeled(ref v1),
                VertexType::Modeled(ref v2),
            ) = (v0, v1, v2)
            {
                let pos0 = cgmath::Vector3::from(v0.position);
                let pos1 = cgmath::Vector3::from(v1.position);
                let pos2 = cgmath::Vector3::from(v2.position);

                let uv0 = cgmath::Vector2::from(v0.tex_coords);
                let uv1 = cgmath::Vector2::from(v1.tex_coords);
                let uv2 = cgmath::Vector2::from(v2.tex_coords);

                let delta_pos1 = pos1 - pos0;
                let delta_pos2 = pos2 - pos0;
                let delta_uv1 = uv1 - uv0;
                let delta_uv2 = uv2 - uv0;

                let det = delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x;
                if det.abs() < 1e-6 {
                    continue;
                }
                let r = 1.0 / det;

                let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

                tangents[i0] += tangent;
                tangents[i1] += tangent;
                tangents[i2] += tangent;

                bitangents[i0] += bitangent;
                bitangents[i1] += bitangent;
                bitangents[i2] += bitangent;

                triangles_included[i0] += 1;
                triangles_included[i1] += 1;
                triangles_included[i2] += 1;
            }
        }

        for (i, vertex) in vertices.iter_mut().enumerate() {
            if triangles_included[i] > 0 {
                let count = triangles_included[i] as f32;
                let tangent = tangents[i] / count;
                let bitangent = bitangents[i] / count;

                if let VertexType::Modeled(v) = vertex {
                    v.tangent = tangent.normalize().into();
                    v.bitangent = bitangent.normalize().into();
                }
            }
        }
    }

    pub fn chunk_size_range(positions: &Vec<f32>, chunk_size: usize) -> std::ops::Range<usize> {
        0..positions.len() / chunk_size
    }
    pub fn cache_index_buffer(
        mesh: &Mesh,
        device: &wgpu::Device,
        cache: &mut HashCache<wgpu::Buffer>,
    ) {
        if !cache.contains(&mesh.index_buffer_key) {
            cache.put(
                mesh.index_buffer_key,
                BufferFactory::create_index_buffer(device, &mesh.indices),
            );
        }
    }
    pub fn cache(mesh: Mesh, cache: &mut HashCache<Mesh>) -> &Mesh {
        let cache_key_clone = mesh.cache_key.clone();
        if !cache.contains(&mesh.index_buffer_key) {
            cache.put(mesh.cache_key, mesh);
        }
        cache
            .get(&cache_key_clone)
            .expect("Mesh should exist in cache at this point.")
    }
    pub fn cache_vertex_buffer(
        mesh: &Mesh,
        device: &wgpu::Device,
        cache: &mut HashCache<wgpu::Buffer>,
    ) {
        if !cache.contains(&mesh.vertex_buffer_key) {
            cache.put(
                mesh.vertex_buffer_key,
                BufferFactory::create_buffer(
                    device,
                    &mesh.vertex_flat_map(),
                    wgpu::BufferUsages::VERTEX,
                    &mesh.vertex_buffer_key.0.to_string(),
                ),
            );
        }
    }
}

impl VertexData for Mesh {
    fn vertices(&self) -> Vec<crate::graphics::vertex::VertexType> {
        self.vertices.to_vec()
    }
}
impl IndexData for Mesh {
    fn indices(&self) -> Vec<u32> {
        self.indices.to_vec()
    }
}
impl Mesh {
    const LABEL: &'static str = "component:mesh";
}
impl HasCacheKey for Mesh {
    fn key(suffixes: Vec<&str>) -> CacheKey {
        let mut base = String::from(Self::LABEL);
        for suffix in suffixes {
            base.push_str(format!(":{}", suffix).as_ref());
        }
        CacheKey::from(&base)
    }
}
