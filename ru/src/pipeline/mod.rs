pub mod manager;
pub mod setup;
pub mod state;
use state::{DepthType, PrimitiveType, ShadingType};

pub fn get_pipeline_label(
    primitive: &PrimitiveType,
    shading: &ShadingType,
    depth: &DepthType,
) -> String {
    format!("[{:?}, {:?}, {:?}]", primitive, shading, depth)
}
