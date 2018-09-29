use entities::Entity;

pub mod camera;
pub mod generational;

pub struct Context {
    pub camera: camera::Camera,
    pub entities: generational::GenerationalArray<Entity>,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            camera: camera::Camera::new(),
            entities: generational::GenerationalArray::new(),
        };
    }
}
