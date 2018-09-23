extern crate gl;
extern crate glutin;

use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use systems::System;

mod components;
mod entities;
mod systems;
mod util;

fn main() {
    use glutin::GlContext;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe { gl_window.make_current().unwrap() };

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let mut rendering_system = systems::rendering::System::new();
    rendering_system.initialize();

    let entity = &mut entities::Entity::new();
    entity.set_renderable(Box::new(components::renderable::Renderable::new()));
    let mut entities = [entity];

    let mut i = 0;
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

        rendering_system.apply(&mut entities);

        gl_window.swap_buffers().unwrap();

        let duration = match now.elapsed() {
            Ok(duration) => duration,
            Err(err) => panic!("Could not compute frame duration {:?}", err),
        };

        if duration < frame_period {
            let sleep_duration = frame_period - duration;
            sleep(sleep_duration);
        }

        match event_loop_proxy.wakeup() {
            Ok(_) => (),
            Err(_) => panic!("Error waking up event loop"),
        };

        ControlFlow::Continue
    });
}
