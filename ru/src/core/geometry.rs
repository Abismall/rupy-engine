use crate::log_debug;

pub enum ObjectDefinition {
    Cube(CubeConfig),
    Sphere(SphereConfig),
}

pub struct CubeConfig {
    pub size: f32,
    pub color: [f32; 4],
}

impl Default for CubeConfig {
    fn default() -> Self {
        CubeConfig {
            size: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

pub struct SphereConfig {
    pub radius: f32,
    pub color: [f32; 4],
}

impl Default for SphereConfig {
    fn default() -> Self {
        SphereConfig {
            radius: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

pub fn generate_geometry(shape: ObjectDefinition) {
    match shape {
        ObjectDefinition::Cube(config) => {
            create_cube(config);
        }
        ObjectDefinition::Sphere(config) => {
            create_sphere(config);
        }
    }
}

fn create_cube(config: CubeConfig) {
    log_debug!(
        "Generating a cube with size: {} and color: {:?}",
        config.size,
        config.color
    );
}

fn create_sphere(config: SphereConfig) {
    log_debug!(
        "Generating a sphere with radius: {} and color: {:?}",
        config.radius,
        config.color
    );
}
