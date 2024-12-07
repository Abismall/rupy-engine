use crate::{
    core::{
        cache::{ComponentCacheKey, HashCache},
        error::AppError,
    },
    ecs::{
        components::{material::model::Material, mesh::model::Mesh, IntoComponentCacheKey},
        traits::Cache,
    },
};

use super::model::{Model, ModelRaw};

pub struct ModelManager {
    models: HashCache<Model>,
}
impl ModelManager {
    pub fn new() -> Self {
        Self {
            models: HashCache::new(),
        }
    }
    pub fn create_model(
        &mut self,
        mesh_ids: Vec<ComponentCacheKey>,
        material_ids: Vec<ComponentCacheKey>,
    ) -> Model {
        Model {
            mesh_ids,
            material_ids,
        }
    }

    pub async fn load_model_from_files<F>(
        &mut self,
        name: &str,
        mut file_loader: F,
        mesh_manager: &mut impl Cache<Mesh>,
        material_manager: &mut impl Cache<Material>,
    ) -> Result<(ComponentCacheKey, Model), AppError>
    where
        F: FnMut(&str) -> Result<ModelRaw, AppError>,
    {
        let model = file_loader(name)?;

        let mesh_ids = model
            .meshes
            .into_iter()
            .map(|mesh| {
                let mesh_id = mesh.into_cache_key();
                mesh_manager.put(mesh_id, mesh)?;
                Ok(mesh_id)
            })
            .collect::<Result<Vec<_>, AppError>>()?;

        let material_ids = model
            .materials
            .into_iter()
            .map(|material| {
                let material_id = material.into_cache_key();
                material_manager.put(material_id, material)?;
                Ok(material_id)
            })
            .collect::<Result<Vec<_>, AppError>>()?;

        let model = self.create_model(mesh_ids, material_ids);
        let model_cache_key = model.into_cache_key();
        Ok((model_cache_key, model))
    }
}
impl Cache<Model> for ModelManager {
    fn get(&self, id: ComponentCacheKey) -> Option<&Model> {
        self.models.get(id)
    }

    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.models.contains(id)
    }

    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut Model> {
        self.models.get_mut(id)
    }

    fn get_or_create<F>(
        &mut self,
        id: ComponentCacheKey,
        create_fn: F,
    ) -> Result<&mut Model, AppError>
    where
        F: FnOnce() -> Result<Model, AppError>,
    {
        self.models.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: ComponentCacheKey, resource: Model) -> Result<(), AppError> {
        self.models.put(id, resource)
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.models.remove(id);
    }
}
