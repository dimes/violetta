pub mod camera;
pub mod generational;
pub mod screen;

use entities::Entity;

pub struct Context {
    pub camera: camera::Camera,
    pub entities: generational::GenerationalArray<Entity>,
    pub screen: screen::Screen,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            camera: camera::Camera::new(),
            entities: generational::GenerationalArray::new(),
            screen: screen::Screen::new(),
        };
    }
}
