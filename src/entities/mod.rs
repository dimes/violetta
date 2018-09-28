use components::Component;
use std::any::Any;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Entity {
    id: u64,
    components: HashMap<&'static str, Box<Any>>,
}

impl Entity {
    pub fn new(id: u64) -> Entity {
        return Entity {
            id: id,
            components: HashMap::new(),
        };
    }

    pub fn get_component<T: Component + Any>(&mut self, name: &'static str) -> Option<&mut T> {
        let component = self.components.get_mut(name);
        match component {
            Some(component) => component.as_mut().downcast_mut(),
            None => None,
        }
    }

    pub fn set_component<T: Component + Any>(&mut self, component: Box<T>) {
        self.components
            .insert(component.name(), component as Box<Any>);
    }
}
