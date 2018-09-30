extern crate gl;

use gl::types::GLfloat;

pub type Mat4 = [GLfloat; 16];

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

pub fn translate(what: &mut Mat4, dx: f32, dy: f32, dz: f32) {
    let translate = [
        1.0, 0.0, 0.0, dx, //
        0.0, 1.0, 0.0, dy, //
        0.0, 0.0, 1.0, dz, //
        0.0, 0.0, 0.0, 1.0, //
    ];
    multiply(what, &translate);
}

pub fn scale(target: &mut Mat4, scale_x: GLfloat, scale_y: GLfloat, scale_z: GLfloat) {
    let scale: Mat4 = [
        scale_x, 0.0, 0.0, 0.0, //
        0.0, scale_y, 0.0, 0.0, //
        0.0, 0.0, scale_z, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    ];
    multiply(target, &scale);
}

pub fn rotate(target: &mut Mat4, x_rads: f32, y_rads: f32, z_rads: f32) {
    let r_x: Mat4 = [
        1.0,
        0.0,
        0.0,
        0.0, //
        0.0,
        x_rads.cos(),
        -x_rads.sin(),
        0.0, //
        0.0,
        x_rads.sin(),
        x_rads.cos(),
        0.0, //
        0.0,
        0.0,
        0.0,
        1.0, //
    ];

    let r_y: Mat4 = [
        y_rads.cos(),
        0.0,
        y_rads.sin(),
        0.0, //
        0.0,
        1.0,
        0.0,
        0.0, //
        -y_rads.sin(),
        0.0,
        y_rads.cos(),
        0.0, //
        0.0,
        0.0,
        0.0,
        1.0, //
    ];

    let r_z: Mat4 = [
        z_rads.cos(),
        -z_rads.sin(),
        0.0,
        0.0, //
        z_rads.sin(),
        z_rads.cos(),
        0.0,
        0.0, //
        0.0,
        0.0,
        1.0,
        0.0, //
        0.0,
        0.0,
        0.0,
        1.0, //
    ];

    multiply(target, &r_x);
    multiply(target, &r_y);
    multiply(target, &r_z);
}
