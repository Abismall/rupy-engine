pub mod model;
use std::collections::HashSet;

use crate::{
    core::{error::AppError, files::FileSystem},
    ecs::model::Entity,
    log_debug,
};

use image::DynamicImage;

#[derive(Debug)]
pub struct Scene {
    _name: String,
    entities: HashSet<Entity>,
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            entities: HashSet::new(),
        }
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }
    pub fn add_entities(&mut self, entity: Vec<Entity>) {
        self.entities.extend(entity);
    }
    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.remove(entity);
    }

    pub fn has_entity(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }
    pub fn get_entities_as_vec_ref(&self) -> Vec<&Entity> {
        self.entities.iter().collect()
    }
    pub fn get_entities(&self) -> &HashSet<Entity> {
        &self.entities
    }
    pub fn get_mut_entities(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }
}

// pub fn load_scene_data(scene_name: &str) -> Result<SceneData, AppError> {
//     let scene_data_path = match FileSystem::list_files_with_extension(
//         FileSystem::get_scene_file_path(&scene_name)?,
//         "rupy",
//     )?
//     .pop()
//     {
//         Some(path) => path,
//         None => {
//             return Err(AppError::FileNotFoundError(format!(
//                 "Could not locate banner for {}",
//                 scene_name
//             )))
//         }
//     };
//     let data_string = FileSystem::read_to_string(scene_data_path)?;
//     let data: SceneData = serde_yaml::from_str(&data_string)?;
//     Ok(data)
// }

pub fn load_scene_image(scene_name: String) -> Result<DynamicImage, AppError> {
    let image_path = match FileSystem::list_files_with_extension(
        FileSystem::get_scene_file_path(&scene_name)?,
        "png",
    )?
    .pop()
    {
        Some(path) => path,
        None => {
            return Err(AppError::FileNotFoundError(format!(
                "Could not locate banner for {}",
                scene_name
            )))
        }
    };

    Ok(FileSystem::load_image_file(&image_path)?)
}
