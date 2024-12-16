use crate::core::cache::{CacheKey, HasCacheKey};
use crate::graphics::binding::material::create_material_bind_group;
use crate::graphics::binding::BindGroupManager;
use crate::{
    core::{error::AppError, files::FileSystem},
    ecs::traits::Cache,
    graphics::textures::{manager::TextureManager, Texture},
};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
    pub dissolve: f32,
    pub optical_density: f32,
    pub illumination_model: Option<u8>,

    pub ambient_texture_key: Option<CacheKey>,
    pub diffuse_texture_key: Option<CacheKey>,
    pub specular_texture_key: Option<CacheKey>,
    pub normal_texture_key: Option<CacheKey>,
    pub shininess_texture_key: Option<CacheKey>,
    pub dissolve_texture_key: Option<CacheKey>,

    pub cache_key: CacheKey,
}

impl Material {
    pub fn new(
        name: String,
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        shininess: f32,
        dissolve: f32,
        optical_density: f32,
        illumination_model: Option<u8>,
        ambient_texture_key: Option<CacheKey>,
        diffuse_texture_key: Option<CacheKey>,
        specular_texture_key: Option<CacheKey>,
        normal_texture_key: Option<CacheKey>,
        shininess_texture_key: Option<CacheKey>,
        dissolve_texture_key: Option<CacheKey>,
        cache_key: CacheKey,
    ) -> Self {
        Self {
            name,
            ambient,
            diffuse,
            specular,
            shininess,
            dissolve,
            optical_density,
            illumination_model,
            ambient_texture_key,
            diffuse_texture_key,
            specular_texture_key,
            normal_texture_key,
            shininess_texture_key,
            dissolve_texture_key,
            cache_key,
        }
    }

    pub fn from_tobj_material(
        obj_material: tobj::Material,
        cache_key: CacheKey,
        texture_map: HashMap<String, (Option<CacheKey>, Option<CacheKey>)>,
    ) -> Self {
        Self {
            name: obj_material.name,
            ambient: obj_material.ambient,
            diffuse: obj_material.diffuse,
            specular: obj_material.specular,
            shininess: obj_material.shininess,
            dissolve: obj_material.dissolve,
            optical_density: obj_material.optical_density,
            illumination_model: obj_material.illumination_model,

            ambient_texture_key: texture_map
                .get(&obj_material.ambient_texture)
                .and_then(|(d, _)| d.clone()),

            diffuse_texture_key: texture_map
                .get(&obj_material.diffuse_texture)
                .and_then(|(d, _)| d.clone()),

            specular_texture_key: texture_map
                .get(&obj_material.specular_texture)
                .and_then(|(d, _)| d.clone()),

            normal_texture_key: texture_map
                .get(&obj_material.normal_texture)
                .and_then(|(_, n)| n.clone()),

            shininess_texture_key: texture_map
                .get(&obj_material.shininess_texture)
                .and_then(|(d, _)| d.clone()),

            dissolve_texture_key: texture_map
                .get(&obj_material.dissolve_texture)
                .and_then(|(d, _)| d.clone()),

            cache_key,
        }
    }
}

impl Material {
    const LABEL: &'static str = "component:material";
}
impl HasCacheKey for Material {
    fn key(suffixes: Vec<&str>) -> CacheKey {
        let mut base = String::from(Self::LABEL);
        for suffix in suffixes {
            base.push_str(format!(":{}", suffix).as_ref());
        }
        CacheKey::from(&base)
    }
}

pub async fn load_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    file_name: &str,
    is_normal_map: bool,
) -> Result<Texture, AppError> {
    let data = FileSystem::load_binary(file_name)?;
    Texture::from_bytes(device, queue, &data, file_name, is_normal_map)
}

pub async fn load_material_textures(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    obj_materials: Vec<&tobj::Material>,
    texture_manager: &mut TextureManager,
) -> Result<HashMap<String, (Option<CacheKey>, Option<CacheKey>)>, AppError> {
    let mut texture_map = HashMap::new();
    for m in obj_materials {
        let mut textures = (None, None);
        if !m.diffuse_texture.is_empty() {
            let diffuse_texture_key = CacheKey::from(m.diffuse_texture.as_str());

            if !texture_manager.textures.contains(&diffuse_texture_key) {
                let diffuse_texture =
                    load_texture(device, queue, &m.diffuse_texture, false).await?;
                texture_manager
                    .textures
                    .put(diffuse_texture_key, Arc::new(diffuse_texture));
            }
            textures.0 = Some(diffuse_texture_key);
        }
        if !m.normal_texture.is_empty() {
            let normal_texture_key = CacheKey::from(m.normal_texture.as_str());
            if !texture_manager.textures.contains(&normal_texture_key) {
                let normal_texture = load_texture(device, queue, &m.normal_texture, true).await?;
                texture_manager
                    .textures
                    .put(normal_texture_key, Arc::new(normal_texture))
            }
            textures.1 = Some(normal_texture_key);
        }

        texture_map.insert(m.name.clone(), textures);
    }
    Ok(texture_map)
}
pub async fn create_material(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    obj_material: tobj::Material,
    texture_manager: &mut TextureManager,
    bind_group_manager: &mut BindGroupManager,
) -> Result<Material, AppError> {
    let bind_group_key = CacheKey::from(&obj_material.name);
    let texture_map =
        load_material_textures(device, queue, vec![&obj_material], texture_manager).await?;
    let diffuse_texture_key = texture_map
        .get(&obj_material.name)
        .and_then(|(d, _)| d.as_ref());
    let normal_texture_key = texture_map
        .get(&obj_material.name)
        .and_then(|(_, n)| n.as_ref());
    let diffuse_texture =
        diffuse_texture_key.and_then(|key| texture_manager.textures.get(&key).map(Arc::as_ref));
    let normal_texture =
        normal_texture_key.and_then(|key| texture_manager.textures.get(&key).map(Arc::as_ref));
    let bind_group = create_material_bind_group(
        device,
        &bind_group_manager
            .bind_group_layouts
            .texture_bind_group_layout,
        diffuse_texture,
        normal_texture,
    )?;

    bind_group_manager
        .bind_groups
        .put(bind_group_key, bind_group);

    let material = Material::from_tobj_material(obj_material, bind_group_key, texture_map);
    Ok(material)
}
