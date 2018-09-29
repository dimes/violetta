extern crate gl;
extern crate glutin;

use components::renderable::Renderable;
use context::Context;
use entities;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use systems::System;

pub trait Game {
    fn initialize(&mut self, context: &mut Context);
    fn game_loop(&mut self, context: &mut Context);
}

pub struct GameRunner {
    game: Box<Game>,
    systems: Vec<Box<System>>,
}

impl GameRunner {
    pub fn new(game: Box<Game>) -> GameRunner {
        return GameRunner {
            game: game,
            systems: Vec::new(),
        };
    }

    pub fn register_system(&mut self, system: Box<System>) {
        self.systems.push(system)
    }

    pub fn start(&mut self) {
        use glutin::GlContext;

        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe { gl_window.make_current().unwrap() };

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        let mut context = Context::new();

        for system in &mut self.systems {
            system.initialize();
        }

        self.game.initialize(&mut context);

        let mut entity = entities::Entity::new(32);
        let mut renderable = Renderable::new();
        renderable.set_size(1.0, 1.0);
        entity.set_component::<Renderable>(renderable);
        context.entities.add(Box::new(entity));

        let mut frame_count = 0;
        let frame_period = Duration::from_millis(16);
        let event_loop_proxy = events_loop.create_proxy();
        events_loop.run_forever(|event| {
            use glutin::{ControlFlow, Event, WindowEvent};

            if let Event::WindowEvent { event, .. } = event {
                if let WindowEvent::CloseRequested = event {
                    return ControlFlow::Break;
                }
            }

            let now = SystemTime::now();
            self.game.game_loop(&mut context);

            for system in &mut self.systems {
                system.apply(&mut context)
            }

            gl_window.swap_buffers().unwrap();

            frame_count += 1;
            let duration = match now.elapsed() {
                Ok(duration) => duration,
                Err(err) => panic!("Could not compute frame duration {:?}", err),
            };

            if duration < frame_period {
                sleep(frame_period - duration);
            }

            match event_loop_proxy.wakeup() {
                Ok(_) => (),
                Err(_) => panic!("Error waking up event loop"),
            };

            ControlFlow::Continue
        })
    }
}
