extern crate gl;
extern crate glutin;

mod components;
mod context;
mod entities;
mod game;
mod systems;
mod util;

struct Game {}

impl game::Game for Game {
    fn initialize(&mut self, context: &mut context::Context) {
        println!("Initializing game");
        context.camera.set_viewport(-1.5, 1.0, 0.5, -1.0);
    }

    fn game_loop(&mut self, context: &mut context::Context) {
        println!("Game loop");
    }
}

fn main() {
    let game = Game {};
    let mut game_runner = game::GameRunner::new(Box::new(game));

    let rendering_system = systems::rendering::System::new();
    game_runner.register_system(Box::new(rendering_system));

    game_runner.start();
}
