pub mod model;
use crate::ecs::components::model::{Material, Mesh};
use crate::ecs::components::uniform::Transform;
use crate::ecs::entities::models::Entity;
use crate::ecs::systems::materials::MaterialManager;
use crate::ecs::systems::world::World;

use crate::shape::cube::Cube;
use crate::shape::triangle::Triangle;
use crate::shape::Geometry;

use crate::{
    core::{error::AppError, files::FileSystem},
    log_error,
};
use model::SceneData;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub entities: Vec<Entity>,
    pub name: String,
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Scene {
            entities: Vec::new(),
            name: name.to_string(),
        }
    }
}

pub fn load_scene_data_file(file_path: &PathBuf) -> Result<SceneData, AppError> {
    let scene_data =
        serde_yaml::from_str(&FileSystem::read_to_string(file_path)?).map_err(|e| {
            log_error!("Failed to load scene data: {:?}", e);
            AppError::from(e)
        })?;
    Ok(scene_data)
}

pub fn load_scene_from_file_path(
    world: &mut World,
    material_manager: &mut MaterialManager,
    file_path: &str,
) -> Result<Scene, AppError> {
    let scenes_dir = FileSystem::get_scenes_dir()?;
    let path = FileSystem::join_paths([scenes_dir, file_path.into(), "scene.rupy".into()]);
    let scene_data = load_scene_data_file(&path)?;
    let mut scene = Scene::new(file_path);

    for entity_data in scene_data.entities.into_iter() {
        // Create a new entity in the world
        let entity = world.create_entity(&scene_data.name);
        scene.entities.push(entity);

        // Handle Transform Component
        if let Some(transform_data) = entity_data.components.transform {
            world.add_component(
                entity,
                Transform {
                    position: transform_data.position,
                    rotation: transform_data.rotation,
                    scale: transform_data.scale,
                },
            );
        }

        // Handle Material Component
        if let Some(material_data) = entity_data.components.material {
            if let Some(texture_id) = &material_data.texture_id {
                let material =
                    material_manager.create_textured_material(&material_data.name, *texture_id);

                world.add_component(entity, material);
            } else {
                world.add_component(
                    entity,
                    Material {
                        name: material_data.name,
                        color: material_data.color,
                        texture_id: material_data.texture_id,
                    },
                );
            }
        }

        // Handle Mesh Component
        if let Some(mesh_data) = entity_data.components.mesh {
            world.add_component(
                entity,
                Mesh {
                    geometry: match mesh_data.geometry_type.as_str() {
                        "Cube" => Geometry::Cube(Cube::default()),
                        "Triangle" => Geometry::Triangle(Triangle::default()),
                        _ => {
                            log_error!("Unsupported geometry type: {}", mesh_data.geometry_type);
                            return Err(AppError::MaterialTypeError(
                                "Unsupported geometry type".to_string(),
                            ));
                        }
                    },
                },
            );
        }
    }

    Ok(scene)
}
