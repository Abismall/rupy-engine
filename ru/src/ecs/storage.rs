use std::{any::TypeId, collections::HashMap};

use crate::{
    ecs::model::{ComponentVec, Entity},
    log_debug,
};

use super::components::Component;
pub fn get_component_from_map<T: Component>(
    components: &HashMap<TypeId, Box<dyn std::any::Any>>,
    entity: Entity,
) -> Option<&T> {
    components
        .get(&TypeId::of::<T>())?
        .downcast_ref::<ComponentVec<T>>()?
        .get(entity)
}
pub fn get_component_from_map_mut<T: Component>(
    components: &mut HashMap<TypeId, Box<dyn std::any::Any>>,
    entity: Entity,
) -> Option<&mut T> {
    components
        .get_mut(&TypeId::of::<T>())?
        .downcast_mut::<ComponentVec<T>>()?
        .get_mut(entity)
}
#[derive(Debug)]
pub struct ComponentManager {
    storages: HashMap<TypeId, Box<dyn std::any::Any>>,
}

impl ComponentManager {
    pub const CAPACITY: usize = 100;
    pub fn new() -> Self {
        Self {
            storages: HashMap::with_capacity(Self::CAPACITY),
        }
    }
    pub fn get_component_from_map<T: Component>(&self, entity: Entity) -> std::option::Option<&T> {
        get_component_from_map::<T>(&self.storages, entity)
    }
    pub fn get_component_from_map_mut<T: Component>(
        &mut self,
        entity: Entity,
    ) -> std::option::Option<&mut T> {
        get_component_from_map_mut::<T>(&mut self.storages, entity)
    }
    pub fn query<T: Component>(&self, mut f: impl FnMut(Entity, &T)) {
        if let Some(storage) = self.get_storage::<T>() {
            for (entity, component) in storage.iter() {
                f(entity, component);
            }
        }
    }
    pub fn query_mut<T: Component>(&mut self, mut f: impl FnMut(Entity, &mut T)) {
        if let Some(storage) = self.get_storage_mut::<T>() {
            for (entity, component) in storage.iter_mut() {
                f(entity, component);
            }
        }
    }
    pub fn query_two<C1: Component, C2: Component>(&self, mut f: impl FnMut(Entity, &C1, &C2)) {
        if let (Some(storage_t), Some(storage_u)) =
            (self.get_storage::<C1>(), self.get_storage::<C2>())
        {
            for (entity, component_t) in storage_t.iter() {
                if let Some(component_u) = storage_u.get(entity) {
                    f(entity, component_t, component_u);
                }
            }
        } else {
            log_debug!(
                "One or both storages not found for components {:?} and {:?}",
                TypeId::of::<C1>(),
                TypeId::of::<C2>()
            );
        }
    }
    pub fn query_three<C1: Component, C2: Component, C3: Component>(
        &self,
        mut f: impl FnMut(Entity, &C1, &C2, &C3),
    ) {
        if let (Some(storage_t), Some(storage_u), Some(storage_m)) = (
            self.get_storage::<C1>(),
            self.get_storage::<C2>(),
            self.get_storage::<C3>(),
        ) {
            for (entity, component_t) in storage_t.iter() {
                if let Some(component_u) = storage_u.get(entity) {
                    if let Some(component_m) = storage_m.get(entity) {
                        f(entity, component_t, component_u, component_m);
                    }
                }
            }
        } else {
            log_debug!(
                "One of the requested storages not found for components {:?}, {:?} and {:?}",
                TypeId::of::<C1>(),
                TypeId::of::<C2>(),
                TypeId::of::<C3>()
            );
        }
    }
    pub fn iter_mut<C: Component>(
        &mut self,
    ) -> std::collections::hash_map::IterMut<'_, TypeId, Box<(dyn std::any::Any + 'static)>> {
        self.storages.iter_mut()
    }
    pub fn get_component_vec_mut<C: Component>(&mut self) -> &mut ComponentVec<C> {
        let type_id = TypeId::of::<C>();
        self.storages
            .entry(type_id)
            .or_insert_with(|| Box::new(ComponentVec::<C>::new()))
            .as_mut()
            .downcast_mut::<ComponentVec<C>>()
            .expect("Type mismatch in component storage")
    }

    pub fn get_component_vec<C: Component>(&self) -> Option<&ComponentVec<C>> {
        let type_id = TypeId::of::<C>();
        self.storages
            .get(&type_id)?
            .downcast_ref::<ComponentVec<C>>()
    }
    pub fn insert_component_storage<C: Component>(&mut self) {
        let type_id = TypeId::of::<C>();
        self.storages
            .insert(type_id, Box::new(ComponentVec::<C>::new()));
    }

    pub fn get_storage<C: Component>(&self) -> Option<&ComponentVec<C>> {
        let type_id = TypeId::of::<C>();
        self.storages
            .get(&type_id)
            .and_then(|storage| storage.downcast_ref::<ComponentVec<C>>())
    }

    pub fn get_storage_mut<C: Component>(&mut self) -> Option<&mut ComponentVec<C>> {
        let type_id = TypeId::of::<C>();
        self.storages
            .get_mut(&type_id)
            .and_then(|storage| storage.downcast_mut::<ComponentVec<C>>())
    }

    pub fn insert_component<C: Component + std::fmt::Debug>(
        &mut self,
        entity: Entity,
        component: C,
    ) {
        if let Some(storage) = self.get_storage_mut::<C>() {
            storage.insert(entity, component);
        } else {
            log_debug!("Creating new storage for component {:?}", TypeId::of::<C>());
            let mut new_storage = ComponentVec::<C>::new();
            new_storage.insert(entity, component);
            self.storages
                .insert(TypeId::of::<C>(), Box::new(new_storage));
        }
    }
    pub fn downcast_storage<C: Component>(&self) -> Option<&ComponentVec<C>> {
        Some(self.get_storage::<C>()?)
    }

    pub fn downcast_storage_mut<C: Component>(&mut self) -> Option<&mut ComponentVec<C>> {
        Some(self.get_storage_mut::<C>()?)
    }
    pub fn remove_component<C: Component>(&mut self, entity: Entity) {
        if let Some(storage) = self.get_storage_mut::<C>() {
            storage.remove(entity);
        }
    }
}
impl<C: Component> ComponentVec<C> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &C)> {
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
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut C)> {
        self.data.iter_mut().enumerate().filter_map(|(id, entry)| {
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

    pub fn insert(&mut self, entity: Entity, component: C) {
        let index = entity.id as usize;
        if index >= self.data.len() {
            self.data.reserve(self.data.len() + 1);
        }
        self.data.push(Some((entity.generation, component)));
    }
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.data
            .get_mut(entity.id as usize)
            .and_then(|entry| entry.as_mut())
            .and_then(|(gen, comp)| {
                if *gen == entity.generation {
                    Some(comp)
                } else {
                    None
                }
            })
    }
    pub fn get(&self, entity: Entity) -> Option<&C> {
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
