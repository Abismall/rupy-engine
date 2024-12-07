use crate::ecs::entity::Entity;

#[derive(Debug)]
pub struct Scene {
    name: String,
    id: u64,
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new(name: &str, id: u64) -> Self {
        Self {
            name: name.to_string(),
            id,
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
