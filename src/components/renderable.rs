use std::cmp::Ordering;
use util::matrix;

#[derive(Debug)]
pub struct Renderable {
    pub dirty: bool,
    pub vertex_range: Option<Box<VertexRange>>,
    pub index_range: Option<Box<VertexRange>>,
    pub local_matrix: Box<matrix::Mat4>,

    x: f32,
    y: f32,
}

impl Renderable {
    pub fn new() -> Renderable {
        return Renderable {
            dirty: true,
            vertex_range: None,
            index_range: None,
            local_matrix: matrix::identity(),
            x: 0.0,
            y: 0.0,
        };
    }

    fn set_position(&mut self, x: f32, y: f32) {
        let dx = x - self.x;
        let dy = y - self.y;

        matrix::translate(&mut self.local_matrix, dx, dy, 0.0);

        self.x = x;
        self.y = y;
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
