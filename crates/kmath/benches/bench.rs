use core::f32;

use bencher::{benchmark_group, benchmark_main, Bencher};

fn mat4_mul_100k_kmath(b: &mut Bencher) {
    use kmath::*;

    let translation = Matrix::from_translation(Vector::new([1. as f32, 2., 3.]));
    let scale = Matrix::<f32, 4, 4>::from_scale(10.);

    let translation_scale = translation * scale;
    b.iter(|| {
        let mut v_out = Matrix::IDENTITY;
        for _ in 0..100_000 {
            v_out = v_out * translation_scale;
        }
        v_out
    })
}

fn mat4_mul_100k_glam(b: &mut Bencher) {
    use glam::*;
    let translation = Mat4::from_translation([1., 2., 3.].into());
    let scale = Mat4::from_scale(Vec3::splat(10.));

    let translation_scale = translation * scale;

    b.iter(|| {
        let mut v_out = Mat4::IDENTITY;
        for _ in 0..100_000 {
            v_out = v_out * translation_scale;
        }
        v_out
    })
}

fn mat4_mul_100k_ultraviolet(b: &mut Bencher) {
    use ultraviolet::*;
    let translation = Mat4::from_translation([1., 2., 3.].into());
    let scale = Mat4::from_scale(10.);

    let translation_scale = translation * scale;

    b.iter(|| {
        let mut v_out = Mat4::identity();
        for _ in 0..100_000 {
            v_out = v_out * translation_scale;
        }
        v_out
    })
}

fn mat4_mul_100k_nalgebra(b: &mut Bencher) {
    use nalgebra::*;
    let translation = Matrix4::identity().append_translation(&Vector3::new(1.0f32, 2., 3.));
    let scale = Matrix4::identity().append_scaling(10.);

    let translation_scale = translation * scale;

    b.iter(|| {
        let mut v_out: Matrix4<f32> = Matrix4::identity();
        for _ in 0..100_000 {
            v_out = v_out * translation_scale;
        }
        v_out
    })
}

fn mat4_inverse_100k_kmath(b: &mut Bencher) {
    use kmath::*;

    b.iter(|| {
        let mut v_out: Matrix<f32, 4, 4> = Matrix::IDENTITY;
        for _ in 0..100_000 {
            v_out = v_out.inversed();
        }
        v_out
    })
}

fn mat4_inverse_100k_glam(b: &mut Bencher) {
    use glam::*;

    b.iter(|| {
        let mut v_out = Mat4::IDENTITY;
        for _ in 0..100_000 {
            v_out = v_out.inverse();
        }
        v_out
    })
}

fn mat4_inverse_100k_ultraviolet(b: &mut Bencher) {
    use ultraviolet::*;

    b.iter(|| {
        let mut v_out = Mat4::identity();
        for _ in 0..100_000 {
            v_out = v_out.inversed();
        }
        v_out
    })
}

fn mat4_inverse_100k_nalgebra(b: &mut Bencher) {
    use nalgebra::*;

    b.iter(|| {
        let mut v_out: Matrix4<f32> = Matrix4::identity();
        for _ in 0..100_000 {
            v_out = v_out.try_inverse().unwrap()
        }
        v_out
    })
}

fn mat6_mul_100k_kmath(bench: &mut Bencher) {
    use kmath::*;

    let a = Matrix::<f32, 6, 6>::IDENTITY;
    let b = Matrix::<f32, 6, 6>::IDENTITY;

    bench.iter(|| {
        let mut v_out = Matrix::IDENTITY;
        for _ in 0..100_000 {
            v_out += a * b;
        }
        v_out
    })
}

fn mat6_mul_100k_nalgebra(bench: &mut Bencher) {
    use nalgebra::*;

    let a = Matrix6::<f32>::identity();
    let b = Matrix6::<f32>::identity();

    bench.iter(|| {
        let mut v_out = Matrix6::<f32>::identity();
        for _ in 0..100_000 {
            v_out += a * b;
        }
        v_out
    })
}

benchmark_group!(
    benches,
    mat4_mul_100k_kmath,
    mat4_mul_100k_glam,
    mat4_mul_100k_ultraviolet,
    mat4_mul_100k_nalgebra,
    mat4_inverse_100k_kmath,
    mat4_inverse_100k_glam,
    mat4_inverse_100k_ultraviolet,
    mat4_inverse_100k_nalgebra,
    mat6_mul_100k_kmath,
    mat6_mul_100k_nalgebra
);
benchmark_main!(benches);
