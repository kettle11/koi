//! Implements Chromatic Adaptation Transformation (CAT).
//!
//! Chromatic Adaptation Transformation means transforming a linear
//! color space's coordinate system from one white point reference to another.
//!
//! [LMSConeSpace] defines the set of supported conversion methods.
//! [LMSConeSpace::Sharp] is used as a default for conversions by
//! [ColorConversion][crate::details::conversion::ColorConversion].
use crate::{const_mat3, Mat3, Vec3};

pub enum LMSConeSpace {
    VonKries,
    Bradford,
    Sharp,
    CmcCat2000,
    Cat02,
}
// from S. Bianco. "Two New von Kries Based
// Chromatic Adapatation Transforms Found by Numerical Optimization."
#[rustfmt::skip]
pub fn lms_cone_space_matrix(cone_space: LMSConeSpace) -> Mat3 {
    match cone_space {
        LMSConeSpace::VonKries => {
            const_mat3!([0.40024, -0.2263, 0.0, 0.7076, 1.16532, 0.0, -0.08081, 0.0457, 0.91822])
        }
        LMSConeSpace::Bradford => {
            const_mat3!([0.8951, -0.7502, 0.0389, 0.2664, 1.7135, -0.0685, -0.1614, 0.0367, 1.0296])
        }
        LMSConeSpace::Sharp => {
            const_mat3!([1.2694, -0.8364, 0.0297, -0.0988, 1.8006, -0.0315, -0.1706, 0.0357, 1.0018])
        }
        LMSConeSpace::CmcCat2000 => {
            const_mat3!([0.7982, -0.5918, 0.0008, 0.3389, 1.5512, 0.239, -0.1371, 0.0406, 0.9753])
        }
        LMSConeSpace::Cat02 => {
            const_mat3!([0.7328, -0.7036, 0.0030, 0.4296, 1.6975, 0.0136, -0.1624, 0.0061, 0.9834])
        }
    }
}

#[rustfmt::skip]
pub fn chromatic_adaptation_transform(
    src_illuminant: Vec3,
    dst_illuminant: Vec3,
    cone_space: LMSConeSpace,
) -> Mat3 {
    let cone_space_transform = lms_cone_space_matrix(cone_space);
    let src_cone_response = cone_space_transform * src_illuminant;
    let dst_cone_response = cone_space_transform * dst_illuminant;
    let src_to_dst_cone = Mat3::from_cols_array(&[
        dst_cone_response.x / src_cone_response.x, 0.0, 0.0,
        0.0, dst_cone_response.y / src_cone_response.y, 0.0,
        0.0, 0.0, dst_cone_response.z / src_cone_response.z,
    ]);

    cone_space_transform.inverse() * src_to_dst_cone * cone_space_transform
}
