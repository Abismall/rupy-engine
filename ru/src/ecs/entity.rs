#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

pub struct EntityManager {
    next_entity_id: u32,
    generations: Vec<u32>,
    entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            next_entity_id: 0,
            generations: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        if id as usize >= self.generations.len() {
            self.generations.push(0);
        }

        let generation = self.generations[id as usize];

        let entity = Entity { id, generation };
        self.entities.push(entity);

        entity
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if let Some(gen) = self.generations.get_mut(entity.id as usize) {
            *gen += 1;
        }
    }

    pub fn is_valid(&self, entity: Entity) -> bool {
        self.generations
            .get(entity.id as usize)
            .map_or(false, |&gen| gen == entity.generation)
    }
    pub fn into_iter(self) -> std::vec::IntoIter<Entity> {
        self.entities.into_iter()
    }
}
