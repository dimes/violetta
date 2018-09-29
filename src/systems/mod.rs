pub mod rendering;

use context::Context;

pub trait System {
    fn initialize(&mut self);
    fn apply(&mut self, context: &mut Context);
}
