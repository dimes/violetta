pub mod rendering;

use entities::Entity;

pub trait System {
    fn initialize(&mut self);
    fn apply(&mut self, entities: &mut [&mut Entity]);
}
