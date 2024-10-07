use crate::prelude::Mat4;

pub struct Keyframe {
    pub transform: Mat4, // Transformation at this keyframe
    pub time: f32,       // Time in the animation this keyframe occurs
}

pub struct Animation {
    pub keyframes: Vec<Keyframe>, // List of keyframes for an animation
    pub current_time: f32,        // Current time in the animation
}

impl Animation {
    pub fn new(keyframes: Vec<Keyframe>) -> Self {
        Self {
            keyframes,
            current_time: 0.0,
        }
    }

    // Placeholder for advancing animation
    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        // Future implementation for interpolating between keyframes
    }

    pub fn apply(&self, object_transform: &mut Mat4) {
        // Future implementation for applying keyframe transforms
    }
}
