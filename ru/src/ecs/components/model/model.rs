use std::io::{BufReader, Cursor};

use crate::{
    core::{cache::ComponentCacheKey, error::AppError, files::FileSystem},
    ecs::{
        components::{
            material::model::Material, mesh::model::Mesh, utils::load_texture,
            IntoComponentCacheKey,
        },
        systems::render::BufferManager,
        traits::{Cache, Vertex},
    },
    graphics::{binding::BindGroupManager, model::VertexType, textures::manager::TextureManager},
    log_info,
    prelude::helpers::string_to_u64,
};

#[derive(Debug, Clone)]
pub struct Model {
    pub mesh_ids: Vec<ComponentCacheKey>,
    pub material_ids: Vec<ComponentCacheKey>,
}
impl IntoComponentCacheKey for Model {
    fn into_cache_key(&self) -> ComponentCacheKey {
        ComponentCacheKey::from(
            self.mesh_ids
                .iter()
                .fold(0u64, |acc, &key| acc ^ key.value())
                ^ self
                    .material_ids
                    .iter()
                    .fold(0u64, |acc, &key| acc ^ key.value()),
        )
    }
}
#[derive(Debug)]
pub struct ModelRaw {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
}
impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 11]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    buffer_manager: &mut BufferManager,
    texture_manager: &mut TextureManager,
    bind_group_manager: &mut BindGroupManager,
) -> Result<Model, AppError> {
    log_info!("Loading model from file: {:?}", file_name);
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
            let mat_text = FileSystem::load_string(&p).expect("Failed to load MTL file");
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut material_ids = Vec::new();
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

        let material = Material::new(
            device,
            file_name,
            layout,
            texture_manager,
            bind_group_manager,
            Some(diffuse_texture_key),
            Some(normal_texture_key),
        )?;
        let material_cache_id = material.into_cache_key();
        material_ids.push(material_cache_id);
    }

    let mut mesh_ids = Vec::new();
    for model in models {
        let vertices = (0..model.mesh.positions.len() / 3)
            .map(|i| {
                VertexType::Modeled(ModelVertex {
                    position: [
                        model.mesh.positions[i * 3],
                        model.mesh.positions[i * 3 + 1],
                        model.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [
                        model.mesh.texcoords[i * 2],
                        1.0 - model.mesh.texcoords[i * 2 + 1],
                    ],
                    normal: [
                        model.mesh.normals[i * 3],
                        model.mesh.normals[i * 3 + 1],
                        model.mesh.normals[i * 3 + 2],
                    ],
                    tangent: [0.0; 3],
                    bitangent: [0.0; 3],
                })
            })
            .collect::<Vec<_>>();
        let num_elements = model.mesh.indices.len() as u32;

        let mesh = Mesh {
            num_elements,
            material: model.mesh.material_id.unwrap_or(0),
        };
        let mesh_cache_id = mesh.into_cache_key();
        mesh_ids.push(mesh_cache_id);
    }

    Ok(Model {
        mesh_ids,
        material_ids,
    })
}
