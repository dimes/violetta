pub struct Screen {
    pub width: u32,
    pub height: u32,
}

impl Screen {
    pub fn new() -> Screen {
        return Screen {
            width: 0,
            height: 0,
        };
    }

    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
