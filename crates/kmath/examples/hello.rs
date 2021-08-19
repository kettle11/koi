use glam::Vec4Swizzles;
use kmath::*;

fn main() {
    let mut v: Vector<f32, 3> = [0., 1., 2.].into();

    v = 10. * v;

    let other_v = v.x;
    // let v = 2.0 * v;
    let values: [f32; 3] = v.into();

    let mut v: Vector<f32, 4> = [0., 1., 2., 5.].into();
    let other_v = v.z;

    let base_matrix = [
        [1.2990382, 0.0, 0.0, 0.0],
        [0.0, 1.7320509, 0.0, 0.0],
        [0.0, 0.0, -1.020202, -1.0],
        [0.0, 0.0, -2.020202, 0.0],
    ];
    let matrix: Mat4 = base_matrix.into();

    let inversed = matrix.inversed();
    println!("INVERSED: {:#?}", inversed);

    let glam_matrix: glam::Mat4 = glam::Mat4::from_cols_array_2d(&base_matrix);

    println!("INVERSED GLAM: {:#?}", glam_matrix.inverse());

    let mat = glam::Mat4::perspective_rh_gl(1.0471976, 1.3333334, 1.0, 100.0);
    println!("PROJECTION MATRIX: {:#?}", mat);

    let mat = mat.inverse();

    let v = mat * glam::Vec4::new(0.0, 0.0, -1.0, 1.0);
    println!("BACK: {:?}", v);
    let v = mat * glam::Vec4::new(0.0, 0.0, 1.0, 1.0);
    let v = v.xyz() / v.w;
    println!("FRONT: {:?}", v);
}

fn glam_test() {
    let v = glam::Vec3::splat(10.);
    let x = 30. * v;
}
