use crate::{
    core::{
        cache::{CacheKey, HasCacheKey},
        error::AppError,
        files::FileSystem,
    },
    ecs::{
        components::{
            material::model::create_material,
            mesh::{manager::create_cached_mesh_with_buffers, model::Mesh},
            ResourceContext,
        },
        traits::Cache,
    },
    log_error,
};
use std::io::{BufReader, Cursor};

#[derive(Debug, Clone)]
pub struct Model {
    pub mesh_ids: Vec<CacheKey>,
    pub material_ids: Vec<CacheKey>,
}
impl Model {
    pub const LABEL: &'static str = "component:model";
}
impl HasCacheKey for Model {
    fn key(suffixes: Vec<&str>) -> CacheKey {
        let mut base = String::from(Self::LABEL);
        for suffix in suffixes {
            base.push_str(format!(":{}", suffix).as_ref());
        }
        CacheKey::from(&base)
    }
}

#[derive(Debug)]
pub struct ModelRaw {
    pub models: Vec<tobj::Model>,
    pub materials: Vec<tobj::Material>,
}
impl ModelRaw {
    pub const LABEL: &'static str = "component:model_raw";
}

pub async fn load_model_raw(file_name: &str) -> Result<ModelRaw, AppError> {
    let obj_text = load_obj_file_string(file_name).await?;
    let mut obj_reader = read_object(obj_text).await?;

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

    Ok(ModelRaw {
        models,
        materials: obj_materials?,
    })
}

pub async fn load_obj_file_string(file_name: &str) -> Result<String, AppError> {
    let result = FileSystem::load_string(file_name);
    match result {
        Ok(text) => Ok(text),
        Err(e) => Err(e),
    }
}

pub async fn read_object(obj_text: String) -> Result<BufReader<Cursor<String>>, AppError> {
    let obj_cursor = Cursor::new(obj_text);
    let obj_reader = BufReader::new(obj_cursor);
    return Ok(obj_reader);
}

async fn build_material_ids(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    obj_materials: Vec<tobj::Material>,
    resources: &mut ResourceContext,
) -> Result<Vec<CacheKey>, AppError> {
    let mut material_ids = Vec::new();

    let texture_manager = &mut resources.texture_manager;
    let bind_group_manager = &mut resources.bind_group_manager;

    for obj_material in obj_materials {
        let material = create_material(
            device,
            queue,
            obj_material,
            texture_manager,
            bind_group_manager,
        )
        .await?;
        let id: CacheKey = material.cache_key.clone();
        resources
            .material_manager
            .materials
            .put(material.cache_key.clone(), material);

        material_ids.push(id);
    }

    Ok(material_ids)
}

fn build_mesh_ids(
    device: &wgpu::Device,
    resources: &mut ResourceContext,
    chunk_size: usize,
    models: Vec<tobj::Model>,
) -> Result<Vec<CacheKey>, AppError> {
    let mut mesh_ids = Vec::new();

    for model in models {
        let mut vertices = Mesh::generate_vertices(
            &model.mesh.positions,
            &model.mesh.texcoords,
            &model.mesh.normals,
            chunk_size,
        );
        Mesh::generate_tangent_space(&mut vertices, &model.mesh.indices, chunk_size);

        match create_cached_mesh_with_buffers(
            device,
            &mut resources.mesh_manager.meshes,
            &mut resources.buffer_manager.buffers,
            vertices,
            model.mesh.indices,
            model.mesh.material_id,
            model.name,
        ) {
            Ok(cache_id) => mesh_ids.push(cache_id),
            Err(e) => {
                log_error!("model:build_mesh_ids: {:?}", e);
            }
        }
    }

    Ok(mesh_ids)
}

pub fn assemble_model(mesh_ids: Vec<CacheKey>, material_ids: Vec<CacheKey>) -> Model {
    Model {
        mesh_ids,
        material_ids,
    }
}

pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    resources: &mut ResourceContext,
) -> Result<Model, AppError> {
    let chunk_size: usize = 3;
    let raw_model = load_model_raw(file_name).await?;
    let material_ids = build_material_ids(device, queue, raw_model.materials, resources).await?;
    let mesh_ids = build_mesh_ids(device, resources, chunk_size, raw_model.models)?;
    Ok(assemble_model(mesh_ids, material_ids))
}
