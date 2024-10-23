use super::Entity;
use std::any::Any;

pub trait Component: Any + Send + Sync {}

impl<T: Any + Send + Sync> Component for T {}

pub trait ComponentStorage {
    fn remove(&self, entity: Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
