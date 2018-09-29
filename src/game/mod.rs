extern crate gl;
extern crate glutin;

use context::Context;
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
        let window_context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, window_context, &events_loop).unwrap();

        unsafe { gl_window.make_current().unwrap() };

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        let mut context = Context::new();
        self.update_screen_dimensions(&mut context, &gl_window);

        for system in &mut self.systems {
            system.initialize();
        }

        self.game.initialize(&mut context);

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
            self.update_screen_dimensions(&mut context, &gl_window);
            self.game.game_loop(&mut context);

            for system in &mut self.systems {
                system.apply(&mut context)
            }

            gl_window.swap_buffers().unwrap();

            frame_count += 1;
            let duration = match now.elapsed() {
                Ok(duration) => duration,
                Err(err) => {
                    println!("Could not compute frame duration {:?}", err);
                    Duration::from_millis(16)
                }
            };

            println!("Frame duration was {:?}", duration);

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

    fn update_screen_dimensions(&mut self, context: &mut Context, window: &glutin::GlWindow) {
        let dpi = window.get_current_monitor().get_hidpi_factor();
        if let Some(screen_size) = window
            .get_inner_size()
            .map({ |value| value.to_physical(dpi) })
        {
            let dimensions: (u32, u32) = screen_size.into();
            context.screen.set_dimensions(dimensions.0, dimensions.1);
        }
    }
}
