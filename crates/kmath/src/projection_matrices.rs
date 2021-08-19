use crate::{numeric_traits::NumericFloat, Matrix};

/// An infinite right handed coordinate system for GL.
pub fn perspective_infinite_gl<T: NumericFloat>(
    vertical_field_of_view_radians: T,
    aspect_ratio: T,
    z_near: T,
) -> Matrix<T, 4, 4> {
    let t = (vertical_field_of_view_radians * T::HALF).tan_numeric();
    let sy = T::ONE / t;
    let sx = sy / aspect_ratio;
    Matrix([
        [sx, T::ZERO, T::ZERO, T::ZERO],
        [T::ZERO, sy, T::ZERO, T::ZERO],
        [T::ZERO, T::ZERO, -T::ONE, -T::ONE],
        [T::ZERO, T::ZERO, -T::TWO * z_near, T::ZERO],
    ])
}

pub fn perspective_gl<T: NumericFloat>(
    vertical_field_of_view_radians: T,
    aspect_ratio: T,
    z_near: T,
    z_far: T,
) -> Matrix<T, 4, 4> {
    let inv_length = T::ONE / (z_near - z_far);
    let f = T::ONE / (vertical_field_of_view_radians * T::HALF).tan_numeric();
    let a = f / aspect_ratio;
    let b = (z_near + z_far) * inv_length;
    let c = (T::TWO * z_near * z_far) * inv_length;
    Matrix([
        [a, T::ZERO, T::ZERO, T::ZERO],
        [T::ZERO, f, T::ZERO, T::ZERO],
        [T::ZERO, T::ZERO, b, -T::ONE],
        [T::ZERO, T::ZERO, c, T::ZERO],
    ])
}

pub fn orthographic_gl<T: NumericFloat>(
    left: T,
    right: T,
    bottom: T,
    top: T,
    near: T,
    far: T,
) -> Matrix<T, 4, 4> {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    let fpn = far + near;
    Matrix([
        [T::TWO / rml, T::ZERO, T::ZERO, T::ZERO],
        [T::ZERO, T::TWO / tmb, T::ZERO, T::ZERO],
        [T::ZERO, T::ZERO, -T::TWO / fmn, T::ZERO],
        [-(rpl / rml), -(tpb / tmb), -(fpn / fmn), T::ONE],
    ])
}
