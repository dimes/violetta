use components::renderable::Renderable;

#[derive(Debug)]
pub struct Entity {
    id: u64,
    renderable: Option<Box<Renderable>>,
}

impl Entity {
    pub fn new(id: u64) -> Entity {
        return Entity {
            id: id,
            renderable: None,
        };
    }

    pub fn get_renderable(&mut self) -> Option<&mut Renderable> {
        match self.renderable.as_mut() {
            Some(r) => return Some(r),
            None => None,
        }
    }

    pub fn set_renderable(&mut self, renderable: Box<Renderable>) {
        self.renderable = Some(renderable);
    }
}
