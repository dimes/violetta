use components::Component;
use std::cmp::Ordering;
use util::matrix;

#[derive(Debug)]
pub struct Renderable {
    pub dirty: bool,
    pub vertex_range: Option<Box<VertexRange>>,
    pub index_range: Option<Box<VertexRange>>,
    pub local_matrix: Box<matrix::Mat4>,

    // State
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Component for Renderable {
    fn name(&self) -> &'static str {
        return Renderable::name();
    }
}

impl Renderable {
    pub fn name() -> &'static str {
        return "renderable";
    }

    pub fn new() -> Box<Renderable> {
        let mut renderable = Renderable {
            dirty: true,
            vertex_range: None,
            index_range: None,
            local_matrix: Box::new(matrix::identity()),

            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        };
        renderable.update_matrix();
        return Box::new(renderable);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.update_matrix();
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.update_matrix();
    }

    pub fn update_matrix(&mut self) {
        self.dirty = true;
        matrix::to_identity(self.local_matrix.as_mut());
        matrix::translate(&mut self.local_matrix, self.x, self.y, 0.0);
        matrix::scale(self.local_matrix.as_mut(), self.width, self.height, 0.0)
    }
}

#[derive(Eq, Debug, Clone)]
pub struct VertexRange {
    pub start: usize,
    pub length: usize,
}

impl Ord for VertexRange {
    fn cmp(&self, other: &VertexRange) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for VertexRange {
    fn partial_cmp(&self, other: &VertexRange) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for VertexRange {
    fn eq(&self, other: &VertexRange) -> bool {
        self.start == other.start
    }
}
