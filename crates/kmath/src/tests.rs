use std::convert::TryInto;

use crate::*;

#[test]
fn to_translation_rotation_scale() {
    let base_m = [
        -16.864699217806663f32,
        5.3354828517975426,
        -98.42314803393586,
        0.,
        -77.429395652955307,
        -62.506756090694715,
        9.8789742299747747,
        0.,
        -60.994021448907333,
        77.874502053626955,
        14.672807413371247,
        0.,
        -150.59115600585938,
        156.88896179199219,
        416.95681762695312,
        1.,
    ];
    let m: Matrix<f32, 4, 4> = (&base_m).try_into().unwrap();

    println!("DECOMPOSED: {:#?}", m.to_translation_rotation_scale());

    let m: glam::Mat4 = glam::Mat4::from_cols_array(&base_m);
    println!("DECOMPOSED: {:#?}", m.to_scale_rotation_translation());
}

#[test]
fn inverse() {
    let base_m = [
        1.2990382, 0.0, 0.0, 0.0, 0.0, 1.7320509, 0.0, 0.0, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0, -2.0,
        0.0,
    ];

    let m: Matrix<f32, 4, 4> = (&base_m).try_into().unwrap();
    println!("INVERSE: {:#?}", m.inversed());
    println!(
        "TRANSFORM: {:?}",
        m.inversed().transform_vector([0., -0., -1.].into())
    );
    let m: glam::Mat4 = glam::Mat4::from_cols_array(&base_m);
    println!("INVERSE: {:#?}", m.inverse());
    println!(
        "TRANSFORM: {:?}",
        m.inverse().transform_vector3([0., -0., -1.].into())
    );
}

#[test]
fn transform() {
    let base_m = [
        0.7698003, 0.0, -0.0, 0.0, 0.0, 0.5773502, 0.0, -0.0, -0.0, 0.0, -0.0, -0.5, 0.0, -0.0,
        -1.0, 5.4999995,
    ];

    let m: glam::Mat4 = glam::Mat4::from_cols_array(&base_m);
    println!(
        "POINT: {:?}",
        m.transform_point3([0.07689452, 0.10095054, 0.0].into())
    );
    println!(
        "POINT: {:?}",
        m.transform_point3([0.07689452, 0.10095054, 1.0].into())
    );
}
