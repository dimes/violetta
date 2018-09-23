extern crate gl;

use gl::types::GLfloat;

pub type Mat4 = [GLfloat; 16];

pub fn translate(what: &mut Mat4, dx: f32, dy: f32, dz: f32) {
    let translate = [
        1.0, 0.0, 0.0, dx, //
        0.0, 1.0, 0.0, dy, //
        0.0, 0.0, 1.0, dz, //
        0.0, 0.0, 0.0, 1.0, //
    ];
    multiply(what, &translate)
}

pub fn identity() -> Box<Mat4> {
    return Box::new([
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    ]);
}

// Stores the result in left
pub fn multiply(left: &mut Mat4, right: &Mat4) {
    // TODO: Improve this
    let mut temp = [0.0f32; 16];
    for k in 0..16 {
        let i = k / 4 * 4;
        let j = k % 4;
        for pos in 0..4 {
            temp[k] = left[i + pos] * right[j + pos]
        }
    }
    left.copy_from_slice(&temp)
}
