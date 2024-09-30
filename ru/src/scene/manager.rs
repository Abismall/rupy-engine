use crate::{log_debug, scene::scene::Scene};
#[derive(Debug)]
pub struct SceneManager {
    scenes: Vec<Scene>,
    active_scene_index: usize,
}

impl SceneManager {
    pub fn new() -> Self {
        let instance = Self {
            scenes: Vec::new(),
            active_scene_index: 0,
        };
        log_debug!("{:?}", instance);
        instance
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn set_active_scene(&mut self, index: usize) {
        if index < self.scenes.len() {
            self.active_scene_index = index;
        }
    }

    pub fn update(&mut self) {
        if let Some(active_scene) = self.scenes.get_mut(self.active_scene_index) {
            active_scene.update_model_matrices();
        }
    }

    pub fn get_active_scene(&self) -> Option<&Scene> {
        self.scenes.get(self.active_scene_index)
    }

    pub fn get_active_scene_mut(&mut self) -> Option<&mut Scene> {
        self.scenes.get_mut(self.active_scene_index)
    }
}
