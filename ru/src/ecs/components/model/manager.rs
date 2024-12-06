use crate::{
    core::{
        cache::{CacheId, HashCache},
        error::AppError,
    },
    ecs::{
        components::{material::model::Material, mesh::model::Mesh},
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

    pub async fn create_model(
        &mut self,
        cache_id: u64,
        name: &str,
        mesh_ids: Vec<CacheId>,
        material_ids: Vec<CacheId>,
    ) -> Result<(u64, Model), AppError> {
        let model = Model {
            name: name.to_string(),
            mesh_ids,
            material_ids,
        };
        self.put(cache_id, model.clone())?;
        Ok((cache_id, model))
    }

    pub async fn load_model_from_files<F>(
        &mut self,
        name: &str,
        cache_id: u64,
        mut file_loader: F,
        mesh_manager: &mut impl Cache<Mesh>,
        material_manager: &mut impl Cache<Material>,
    ) -> Result<(u64, Model), AppError>
    where
        F: FnMut(&str) -> Result<ModelRaw, AppError>,
    {
        let model = file_loader(name)?;

        let mesh_ids = model
            .meshes
            .into_iter()
            .map(|mesh| {
                let mesh_id = CacheId::from(mesh.name.as_ref());
                mesh_manager.put(mesh_id.value(), mesh)?;
                Ok(mesh_id)
            })
            .collect::<Result<Vec<_>, AppError>>()?;

        let material_ids = model
            .materials
            .into_iter()
            .map(|material| {
                let material_id = CacheId::from(material.name.as_ref());
                material_manager.put(material_id.value(), material)?;
                Ok(material_id)
            })
            .collect::<Result<Vec<_>, AppError>>()?;

        self.create_model(cache_id, name, mesh_ids, material_ids)
            .await
    }
}

impl Cache<Model> for ModelManager {
    fn get(&self, id: u64) -> Option<&Model> {
        self.models.get(id)
    }

    fn contains(&self, id: u64) -> bool {
        self.models.contains(id)
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut Model> {
        self.models.get_mut(id)
    }

    fn get_or_create<F>(&mut self, id: u64, create_fn: F) -> Result<&mut Model, AppError>
    where
        F: FnOnce() -> Result<Model, AppError>,
    {
        self.models.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: u64, resource: Model) -> Result<(), AppError> {
        self.models.put(id, resource)
    }

    fn remove(&mut self, id: u64) {
        self.models.remove(id);
    }
}
