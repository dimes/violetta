use entities::Entity;

pub struct GenerationalArray<T> {
    entities: Vec<Container<T>>,
    free_list: Vec<usize>,
    generation: u64,
}

pub struct Key {
    index: usize,
    generation: u64,
}

struct Container<T> {
    generation: u64,
    value: Option<Box<T>>,
}

impl<T> GenerationalArray<T> {
    pub fn new() -> GenerationalArray<T> {
        return GenerationalArray {
            entities: Vec::new(),
            free_list: Vec::new(),
            generation: 0,
        };
    }

    pub fn add(&mut self, element: Box<T>) -> Key {
        let free_index = self.free_list.pop();
        match free_index {
            Some(index) => {
                let container = &mut self.entities[index];
                container.generation = self.generation;
                container.value = Some(element);
                return Key {
                    index: index,
                    generation: self.generation,
                };
            }
            None => {
                let container = Container {
                    generation: self.generation,
                    value: Some(element),
                };
                &self.entities.push(container);
                return Key {
                    index: self.entities.len() - 1,
                    generation: self.generation,
                };
            }
        };
    }

    pub fn get(&mut self, key: &Key) -> Option<&mut T> {
        let container = &mut self.entities[key.index];
        if container.generation > key.generation {
            return None;
        }

        return match container.value {
            Some(ref mut value) => Some(value.as_mut()),
            None => None,
        };
    }

    pub fn get_raw(&mut self, index: usize) -> Option<&mut T> {
        let container = &mut self.entities[index];
        return match container.value {
            Some(ref mut value) => Some(value.as_mut()),
            None => None,
        };
    }

    pub fn raw_len(&self) -> usize {
        return self.entities.len();
    }

    pub fn remove(&mut self, key: &Key) {
        let container = &mut self.entities[key.index];
        if container.generation > key.generation {
            return;
        }

        container.value = None;
        self.free_list.push(key.index);
        self.generation += 1;
    }
}
