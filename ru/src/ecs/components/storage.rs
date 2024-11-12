use super::{
    model::{ComponentVec, Entity},
    Component,
};

pub trait ComponentStorage {
    fn remove(&self, entity: Entity);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Component> ComponentVec<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.data.iter().enumerate().filter_map(|(id, entry)| {
            if let Some((generation, component)) = entry {
                Some((
                    Entity {
                        id: id as u32,
                        generation: *generation,
                    },
                    component,
                ))
            } else {
                None
            }
        })
    }
    pub fn insert(&mut self, entity: Entity, component: T) {
        let index = entity.id as usize;
        if index >= self.data.len() {
            self.data.reserve(self.data.len() + 1);
        }
        self.data.push(Some((entity.generation, component)));
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data
            .get(entity.id as usize)
            .and_then(|entry| entry.as_ref())
            .and_then(|(gen, comp)| {
                if *gen == entity.generation {
                    Some(comp)
                } else {
                    None
                }
            })
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(entry) = self.data.get_mut(entity.id as usize) {
            if let Some((gen, _)) = entry {
                if *gen == entity.generation {
                    *entry = None;
                }
            }
        }
    }
}
