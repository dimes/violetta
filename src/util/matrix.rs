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
    multiply(what, &translate);
}

pub fn identity() -> Mat4 {
    return [
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    ];
}

pub fn to_identity(target: &mut Mat4) {
    for i in 0..16 {
        target[i] = 0.0;
    }

    target[0] = 1.0;
    target[5] = 1.0;
    target[10] = 1.0;
    target[15] = 1.0;
}

// Stores the result in left
pub fn multiply(left: &mut Mat4, right: &Mat4) {
    // TODO: Improve this
    let mut temp = [0.0f32; 16];
    for k in 0..16 {
        let i = k / 4 * 4;
        let j = k % 4;
        for pos in 0..4 {
            temp[k] = temp[k] + left[i + pos] * right[j + (4 * pos)];
        }
    }
    left.copy_from_slice(&temp)
}

pub fn scale(target: &mut Mat4, scaleX: GLfloat, scaleY: GLfloat, scaleZ: GLfloat) {
    let scale: Mat4 = [
        scaleX, 0.0, 0.0, 0.0, //
        0.0, scaleY, 0.0, 0.0, //
        0.0, 0.0, scaleZ, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    ];
    return multiply(target, &scale);
}
