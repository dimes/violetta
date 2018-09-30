pub mod camera;
pub mod clock;
pub mod generational;
pub mod screen;

use entities::Entity;

pub struct Context {
    pub camera: camera::Camera,
    pub clock: clock::Clock,
    pub entities: generational::GenerationalArray<Entity>,
    pub screen: screen::Screen,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            camera: camera::Camera::new(),
            clock: clock::Clock::new(),
            entities: generational::GenerationalArray::new(),
            screen: screen::Screen::new(),
        };
    }
}
