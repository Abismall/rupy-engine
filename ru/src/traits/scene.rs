use super::world::World;

pub trait System {
    fn update(&mut self, world: &mut World);
}
