extern crate gl;
extern crate glutin;
extern crate rand;

mod components;
mod context;
mod entities;
mod game;
mod systems;
mod util;

use components::renderable::Renderable;
use context::generational::Key;
use context::Context;
use rand::Rng;

struct Game {
    objs: Vec<Box<GameBox>>,
}

impl Game {
    pub fn new() -> Game {
        return Game { objs: Vec::new() };
    }
}

impl game::Game for Game {
    fn initialize(&mut self, context: &mut Context) {
        println!("Initializing game");

        context.camera.set_viewport(
            0.0,
            0.0,
            context.screen.width as f32,
            context.screen.height as f32,
        );

        for _ in 0..500 {
            self.objs.push(Box::new(GameBox::new(context)));
        }
    }

    fn game_loop(&mut self, context: &mut Context) {
        for obj in &mut self.objs {
            obj.update(context);
        }
    }
}

fn main() {
    let game = Game::new();
    let mut game_runner = game::GameRunner::new(Box::new(game));

    let rendering_system = systems::rendering::System::new();
    game_runner.register_system(Box::new(rendering_system));

    game_runner.start();
}

struct Position {
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
}

struct GameBox {
    obj: GameObj,
    direction: (f32, f32),
}

impl GameBox {
    fn new(context: &mut Context) -> GameBox {
        let dx = rand::thread_rng().gen::<f32>();
        let dy = rand::thread_rng().gen::<f32>();
        let size = 10.0 + 200.0 * rand::thread_rng().gen::<f32>();
        let speed = -25.0 + 50.0 * rand::thread_rng().gen::<f32>();
        let mut game_obj = GameObj::new(context);
        game_obj.set_position(context, 300.0, 300.0);
        game_obj.set_size(context, size, size);
        let game_box = GameBox {
            obj: game_obj,
            direction: (speed * dx, speed * dy),
        };

        return game_box;
    }

    fn update(&mut self, context: &mut Context) {
        let pos = self.obj.get_position(context);
        let mut translation_x = self.direction.0;
        let mut translation_y = self.direction.1;

        let width = context.screen.width as f32;
        let height = context.screen.height as f32;

        if pos.0 < 0.0 {
            translation_x = width;
        }

        if pos.0 > width {
            translation_x = -width;
        }

        if pos.1 < 0.0 {
            translation_y = height;
        }

        if pos.1 > height {
            translation_y = -height;
        }

        self.obj
            .set_position(context, pos.0 + translation_x, pos.1 + translation_y);
    }
}

struct GameObj {
    key: Key,
}

impl GameObj {
    fn new(context: &mut Context) -> GameObj {
        let mut entity = entities::Entity::new(32);
        let mut renderable = Renderable::new();
        entity.set_component::<Renderable>(renderable);
        let key = context.entities.add(Box::new(entity));
        return GameObj { key };
    }

    fn set_size(&mut self, context: &mut Context, width: f32, height: f32) {
        if let Some(renderable) = context
            .entities
            .get(&self.key)
            .and_then({ |entity| entity.get_component::<Renderable>(Renderable::name()) })
        {
            renderable.set_size(width, height);
        }
    }

    fn get_position(&mut self, context: &mut Context) -> (f32, f32) {
        if let Some(renderable) = context
            .entities
            .get(&self.key)
            .and_then({ |entity| entity.get_component::<Renderable>(Renderable::name()) })
        {
            return (renderable.x, renderable.y);
        }

        return (0.0, 0.0);
    }

    fn set_position(&mut self, context: &mut Context, x: f32, y: f32) {
        if let Some(renderable) = context
            .entities
            .get(&self.key)
            .and_then({ |entity| entity.get_component::<Renderable>(Renderable::name()) })
        {
            renderable.set_position(x, y);
        }
    }

    /// Returns the position of this object, relative to the top left corner of the screen.
    fn position_on_screen(&mut self, context: &mut Context, out_position: &mut Position) {
        if let Some(renderable) = context
            .entities
            .get(&self.key)
            .and_then({ |entity| entity.get_component::<Renderable>(Renderable::name()) })
        {
            let screen = &context.screen;
            let camera = &context.camera;

            let pixels_per_world_width = screen.width as f32 / (camera.right - camera.left);
            let pixels_per_world_height = screen.height as f32 / (camera.bottom - camera.top);

            let center_x = (renderable.x - camera.left) * pixels_per_world_width;
            let center_y = (renderable.y - camera.top) * pixels_per_world_height;
            let width = renderable.width * pixels_per_world_width;
            let height = renderable.height * pixels_per_world_height;

            out_position.center_x = center_x;
            out_position.center_x = center_y;
            out_position.width = width;
            out_position.height = height;
        }
    }
}
