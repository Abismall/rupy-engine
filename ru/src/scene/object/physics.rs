use crate::prelude::{add_vec3, scale_vec3, Vec3, VEC3_ZERO};

pub struct PhysicsComponent {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
}

impl PhysicsComponent {
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: [0.0, 0.0, 0.0],
            acceleration: [0.0, 0.0, 0.0],
            mass,
        }
    }

    pub fn apply_force(&mut self, force: [f32; 3]) {
        self.acceleration = add_vec3(self.acceleration, scale_vec3(force, 1.0 / self.mass));
    }

    pub fn update(&mut self, delta_time: f32) {
        self.velocity = add_vec3(self.velocity, scale_vec3(self.acceleration, delta_time));
        self.acceleration = VEC3_ZERO
    }
}
