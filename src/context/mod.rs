use util::matrix;

pub struct Context {
    pub camera: Camera,
}

pub struct Camera {
    pub view_matrix: Box<matrix::Mat4>,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            camera: Camera::new(),
        };
    }
}

impl Camera {
    fn new() -> Camera {
        return Camera {
            view_matrix: Box::new(matrix::identity()),
        };
    }

    pub fn set_viewport(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        let half_width = (right - left) / 2.0;
        let center_x = left + half_width;

        let half_height = (bottom - top) / 2.0;
        let center_y = top + half_height;

        let width_scale = 1.0 / half_width;
        let height_scale = 1.0 / half_height;

        matrix::to_identity(self.view_matrix.as_mut());
        matrix::translate(self.view_matrix.as_mut(), -center_x, -center_y, 0.0);
        matrix::scale(self.view_matrix.as_mut(), width_scale, height_scale, 0.0);
    }
}
