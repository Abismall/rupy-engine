pub mod cache;
pub mod schema;

// Debug
const LABEL_TEXTURE_BIND_GROUP: &str = "Texture BindGroupLayout";
const LABEL_CAMERA_BIND_GROUP: &str = "Camera BindGroupLayout";
const LABEL_MODEL_BIND_GROUP: &str = "Model BindGroupLayout";

// Bindings
const BINDING_CAMERA_UNIFORM: u32 = 0;
const BINDING_MODEL_UNIFORM: u32 = 0;
const BINDING_TEXTURE: u32 = 0;
const BINDING_SAMPLER: u32 = 1;
