use crate::{FType, Mat3, Vec3};

pub fn xyz_to_rgb(primaries: &[[FType; 2]; 3], white_point: &[FType; 3]) -> Mat3 {
    rgb_to_xyz(primaries, white_point).inverse()
}

#[rustfmt::skip]
#[allow(non_snake_case)]
pub fn rgb_to_xyz(primaries: &[[FType;2]; 3], white_point: &[FType;3]) -> Mat3 {
    let [[xr, yr], [xg, yg], [xb, yb]] = *primaries;
    let [Wx, Wy, Wz] = *white_point;

    let Xr = xr / yr;
    let Yr = 1.0;
    let Zr = (1.0 - xr - yr) / yr;
    let Xg = xg / yg;
    let Yg = 1.0;
    let Zg = (1.0 - xg - yg) / yg;
    let Xb = xb / yb;
    let Yb = 1.0;
    let Zb = (1.0 - xb - yb) / yb;
    let mut base_matrix = Mat3::from_cols_array(&[
        Xr, Yr, Zr,
        Xg, Yg, Zg,
        Xb, Yb, Zb
    ]);

    let scale = base_matrix.inverse() * Vec3::new(Wx, Wy, Wz);
    base_matrix.x_axis *= scale.x;
    base_matrix.y_axis *= scale.y;
    base_matrix.z_axis *= scale.z;

    base_matrix
}
