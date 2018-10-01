use std::collections::BTreeSet;

const NUM_TEXTURES_PER_SET: i32 = 512;

pub struct TextureManager {
    groups: Vec<Box<TextureGroup>>,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        return TextureManager { groups: Vec::new() };
    }

    pub fn add_group(&mut self, width: i32, height: i32) -> usize {
        self.groups.push(Box::new(TextureGroup::new(width, height)));
        return self.groups.len() - 1;
    }

    pub fn get_group(&mut self, index: usize) -> &mut TextureGroup {
        return self.groups[index].as_mut();
    }

    pub fn num_groups(&self) -> usize {
        return self.groups.len();
    }

    pub fn add_texture(&mut self, group_index: usize, texture: Box<[u8]>) -> TextureId {
        let group = &mut self.groups[group_index];
        let mut last_index = -1;
        for i in &group.used_indices {
            if *i > last_index + 1 {
                break;
            }
            last_index = *i;
        }

        let index = last_index + 1;
        group.used_indices.insert(index);

        let pending_texture = PendingTexture { index, texture };
        group.pending_textures.push(pending_texture);

        return TextureId {
            group: group_index,
            index: index,
        };
    }
}

pub struct TextureId {
    group: usize,
    index: i32,
}

pub struct TextureGroup {
    pub handle: u32,
    pub max_size: i32,
    pub width: i32,
    pub height: i32,
    pub pending_textures: Vec<PendingTexture>,
    used_indices: BTreeSet<i32>,
}

pub struct PendingTexture {
    pub index: i32,
    pub texture: Box<[u8]>,
}

impl TextureGroup {
    pub fn new(width: i32, height: i32) -> TextureGroup {
        return TextureGroup {
            handle: 0,
            max_size: NUM_TEXTURES_PER_SET,
            width: width,
            height: height,
            used_indices: BTreeSet::new(),
            pending_textures: Vec::new(),
        };
    }
}
