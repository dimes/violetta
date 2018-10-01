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

        for _ in 0..1700 {
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

struct GameBox {
    obj: GameObj,
    direction: (f32, f32),
}

impl GameBox {
    fn new(context: &mut Context) -> GameBox {
        let x = rand::thread_rng().gen::<f32>() * context.screen.width as f32;
        let y = rand::thread_rng().gen::<f32>() * context.screen.height as f32;
        let z = 2.0 * rand::thread_rng().gen::<f32>() - 1.0;
        let dx = 2.0 * rand::thread_rng().gen::<f32>() - 1.0;
        let dy = 2.0 * rand::thread_rng().gen::<f32>() - 1.0;
        let size = 5.0 + 100.0 * rand::thread_rng().gen::<f32>();
        let speed = -25.0 + 50.0 * rand::thread_rng().gen::<f32>();
        let mut game_obj = GameObj::new(context);
        game_obj.set_position(context, x, y);
        game_obj.set_size(context, size, size);
        game_obj.set_z_index(context, z);
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
        let renderable = Renderable::new();
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
            let z = renderable.z;
            renderable.set_position(x, y, z);
        }
    }

    fn set_z_index(&mut self, context: &mut Context, z: f32) {
        if let Some(renderable) = context
            .entities
            .get(&self.key)
            .and_then({ |entity| entity.get_component::<Renderable>(Renderable::name()) })
        {
            let x = renderable.x;
            let y = renderable.y;
            renderable.set_position(x, y, z);
        }
    }
}
