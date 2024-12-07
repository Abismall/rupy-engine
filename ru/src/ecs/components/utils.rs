use std::io::{BufReader, Cursor};

use cgmath::InnerSpace;
use itertools::Itertools;

use crate::{
    core::{cache::ComponentCacheKey, error::AppError, files::FileSystem},
    ecs::{systems::render::BufferManager, traits::Cache},
    graphics::{
        binding::BindGroupManager,
        model::VertexType,
        textures::{manager::TextureManager, Texture},
    },
    prelude::helpers::string_to_u64,
};

use super::{
    material::model::Material,
    mesh::model::Mesh,
    model::model::{ModelRaw, ModelVertex},
    IntoComponentCacheKey,
};

pub async fn load_texture(
    file_name: &str,
    is_normal_map: bool,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<Texture, AppError> {
    let data = FileSystem::load_binary(file_name)?;
    Texture::from_bytes(device, queue, &data, file_name, is_normal_map)
}
pub async fn load_model<'a>(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    buffer_manager: &'a mut BufferManager,
    texture_manager: &'a mut TextureManager,
    bind_group_manager: &'a mut BindGroupManager,
) -> Result<ModelRaw, AppError> {
    let bind_group_key = ComponentCacheKey::from(file_name);
    let obj_text = FileSystem::load_string(file_name)?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);
    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text =
                FileSystem::load_string(&p).expect("load_model::FileSystem::load_string");
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut materials = Vec::new();
    for m in obj_materials? {
        let diffuse_texture_key = ComponentCacheKey::from(string_to_u64(&m.diffuse_texture));
        if !texture_manager.contains(diffuse_texture_key) {
            let diffuse_texture = load_texture(&m.diffuse_texture, false, device, queue).await?;
            texture_manager
                .put(diffuse_texture_key, diffuse_texture.into())
                .ok();
        }

        let normal_texture_key = ComponentCacheKey::from(string_to_u64(&m.normal_texture));
        if !texture_manager.contains(normal_texture_key) {
            let normal_texture = load_texture(&m.normal_texture, true, device, queue).await?;
            texture_manager
                .put(diffuse_texture_key, normal_texture.into())
                .ok();
        }

        materials.push(Material::new(
            device,
            file_name,
            layout,
            texture_manager,
            bind_group_manager,
            Some(diffuse_texture_key),
            Some(normal_texture_key),
        )?);
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| {
                    VertexType::Modeled(ModelVertex {
                        position: [
                            m.mesh.positions[i * 3],
                            m.mesh.positions[i * 3 + 1],
                            m.mesh.positions[i * 3 + 2],
                        ],
                        tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                        normal: [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ],
                        tangent: [0.0; 3],
                        bitangent: [0.0; 3],
                    })
                })
                .collect::<Vec<_>>();

            let indices = m.mesh.indices.iter().map(|&i| i as u16).collect_vec();

            generate_tangent_space(&mut vertices, &indices);

            let mesh = Mesh {
                num_elements: indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            };

            let mesh_cache_key = mesh.into_cache_key();
            buffer_manager.create_vertex_buffer(device, &vertices, mesh_cache_key);
            buffer_manager.create_index_buffer(device, &indices, mesh_cache_key);
            mesh
        })
        .collect::<Vec<_>>();

    Ok(ModelRaw { meshes, materials })
}
fn generate_tangent_space(vertices: &mut Vec<VertexType>, indices: &[u16]) {
    let mut tangents = vec![cgmath::Vector3::new(0.0, 0.0, 0.0); vertices.len()];
    let mut bitangents = vec![cgmath::Vector3::new(0.0, 0.0, 0.0); vertices.len()];
    let mut triangles_included = vec![0; vertices.len()];

    for chunk in indices.chunks_exact(3) {
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
