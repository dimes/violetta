pub mod rendering;

use context::Context;
use entities::Entity;

pub trait System {
    fn initialize(&mut self);
    fn apply(&mut self, context: &Context, entities: &mut [&mut Entity]);
}
