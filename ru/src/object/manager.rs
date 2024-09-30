use crate::log_debug;

use super::object::Object;

#[derive(Default, Debug)]
pub struct ObjectManager {
    objects: Vec<Object>,
}

impl ObjectManager {
    pub fn new() -> Self {
        let instance = Self {
            objects: Vec::new(),
        };
        log_debug!("{:?}", instance);
        instance
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, index: usize) {
        if index < self.objects.len() {
            self.objects.remove(index);
        }
    }

    pub fn update_object_model_matrices(&mut self) {
        self.objects
            .iter_mut()
            .for_each(|f| f.update_model_matrix());
    }

    pub fn get_render_data(&self) -> &[Object] {
        &self.objects
    }

    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut Object> {
        self.objects.get_mut(index)
    }

    pub fn get_object(&self, index: usize) -> Option<&Object> {
        self.objects.get(index)
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}
