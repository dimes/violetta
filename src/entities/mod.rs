use components::renderable::Renderable;

#[derive(Debug)]
pub struct Entity {
    renderable: Option<Box<Renderable>>,
}

impl Entity {
    pub fn new() -> Entity {
        return Entity { renderable: None };
    }

    pub fn get_renderable(&mut self) -> Option<&mut Renderable> {
        return self.renderable.as_mut().map(|value| &mut **value);
    }

    pub fn set_renderable(&mut self, renderable: Box<Renderable>) {
        self.renderable = Some(renderable);
    }
}
