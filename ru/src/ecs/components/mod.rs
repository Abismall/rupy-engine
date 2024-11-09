pub mod model;
pub mod storage;
pub mod uniform;
pub mod vertex;

use std::any::Any;

use nalgebra::{Matrix4, Vector3};
use uniform::UniformModel;

pub trait Component: Any + Send + Sync {}

impl<T: Any + Send + Sync> Component for T {}

pub fn create_model_matrix(
    position: [f32; 3],
    rotation: [[f32; 4]; 4],
    scale: [f32; 3],
) -> UniformModel {
    // Convert position array to nalgebra Vector3
    let position = Vector3::new(position[0], position[1], position[2]);

    // Convert rotation and scale arrays to nalgebra Matrix4
    let rotation_matrix = Matrix4::from_column_slice(&rotation.concat());
    let scale_matrix = Matrix4::from_column_slice(&scale);

    // Create translation matrix
    let translation_matrix = Matrix4::new_translation(&position);

    // Combine all transformations to form the final model matrix
    let model_matrix = translation_matrix * rotation_matrix * scale_matrix;

    // Assuming ModelUniform::from can accept a Matrix4
    UniformModel::from(model_matrix)
}
