use util::matrix;

pub struct Camera {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub view_matrix: Box<matrix::Mat4>,
}

impl Camera {
    pub fn new() -> Camera {
        return Camera {
            left: -1.0,
            top: 1.0,
            right: 1.0,
            bottom: -1.0,
            view_matrix: Box::new(matrix::identity()),
        };
    }

    pub fn set_viewport(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;

        let half_width = (right - left) / 2.0;
        let center_x = left + half_width;

        let half_height = (bottom - top) / 2.0;
        let center_y = top + half_height;

        let width_scale = 1.0 / half_width;
        let height_scale = 1.0 / half_height;

        matrix::to_identity(self.view_matrix.as_mut());
        matrix::scale(self.view_matrix.as_mut(), width_scale, height_scale, 0.0);
        matrix::translate(self.view_matrix.as_mut(), -center_x, -center_y, 0.0);
    }
}
