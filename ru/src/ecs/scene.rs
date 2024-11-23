use crate::ecs::entity::Entity;

#[derive(Debug)]
pub struct Scene {
    pub name: String,
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn get_entities(&self) -> &[Entity] {
        &self.entities
    }
}
