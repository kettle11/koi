pub use kmath::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    multiplicative_inverse: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let direction = direction.normalized();
        Self {
            origin,
            multiplicative_inverse: direction.reciprocal(),
            direction,
        }
    }

    pub fn transform_by(self, mat4: &Mat4) -> Self {
        let direction = mat4.transform_vector(self.direction);

        Ray {
            origin: mat4.transform_point(self.origin),
            direction,
            multiplicative_inverse: direction.reciprocal(),
        }
    }

    pub fn get_point(self, distance: f32) -> Vec3 {
        self.origin + self.direction * distance
    }
}

// https://tavianator.com/fast-branchless-raybounding-box-intersections-part-2-nans/
pub fn ray_with_bounding_box(r: Ray, b: geometry::BoundingBox<f32, 3>) -> (bool, f32) {
    let min_sub_origin_times_inverse =
        (b.min - r.origin).mul_by_component(r.multiplicative_inverse);
    let max_sub_origin_times_inverse =
        (b.max - r.origin).mul_by_component(r.multiplicative_inverse);

    let min = min_sub_origin_times_inverse.min(max_sub_origin_times_inverse);
    let max = min_sub_origin_times_inverse.max(max_sub_origin_times_inverse);

    let tmin = min.max_component();
    let tmax = max.min_component();

    let tmin = f32::max(tmin, 0.0);
    (tmax >= tmin, tmin)
}

#[test]
fn ray_with_bounding_box0() {
    let bounding_box = BoundingBox::<f32, 3>::new(Vec3::ZERO, Vec3::ONE);
    let ray = Ray::new(Vec3::new(-0.5, 0.5, 0.5), Vec3::X);
    let intersects = ray_with_bounding_box(ray, bounding_box);
    assert!(intersects.0);

    let ray = Ray::new(Vec3::new(0.5, 2.0, 0.5), -Vec3::Y);
    let intersects = ray_with_bounding_box(ray, bounding_box);
    assert!(intersects.0);

    let ray = Ray::new(Vec3::new(1.5, 0.5, 0.5), -Vec3::X);
    let intersects = ray_with_bounding_box(ray, bounding_box);
    assert!(intersects.0);
}

// Möller–Trumbore intersection algorithm
// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
pub fn ray_with_tri(ray: Ray, vertex0: Vec3, vertex1: Vec3, vertex2: Vec3) -> (bool, f32, Vec3) {
    const EPSILON: f32 = 0.000_000_1;
    let edge1 = vertex1 - vertex0;
    let edge2 = vertex2 - vertex0;
    let h = Vec3::cross(ray.direction, edge2);
    let a = Vec3::dot(edge1, h);

    if a > -EPSILON && a < EPSILON {
        return (false, 0., Vec3::ZERO);
    }

    let f = 1.0 / a;
    let s = ray.origin - vertex0;
    let u = f * Vec3::dot(s, h);

    if !(0.0..=1.0).contains(&u) {
        return (false, 0., Vec3::ZERO);
    }

    let q = Vec3::cross(s, edge1);
    let v = f * Vec3::dot(ray.direction, q);
    if v < 0.0 || u + v > 1.0 {
        return (false, 0., Vec3::ZERO);
    }

    // At this stage we can compute t to find out where the intersection point is on the line.
    let t = f * Vec3::dot(edge2, q);

    if t > EPSILON {
        let out_intersection_point = ray.origin + ray.direction * t;
        (true, t, out_intersection_point)
    } else {
        // This means that there is a line intersection but not a ray intersection.
        (false, t, Vec3::ZERO)
    }
}

// Brute force ray with mesh ray test.
pub fn ray_with_mesh(ray: Ray, vertices: &[Vec3], indices: &[[u32; 3]]) -> Option<f32> {
    let mut nearest = std::f32::MAX;
    let mut intersects = false;
    for [i0, i1, i2] in indices.iter() {
        let result = ray_with_tri(
            ray,
            vertices[*i0 as usize],
            vertices[*i1 as usize],
            vertices[*i2 as usize],
        );

        if result.0 {
            let dis = result.1;

            if dis < nearest {
                nearest = dis;
                intersects = true;
            }
        }
    }

    if intersects {
        Some(nearest)
    } else {
        None
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub distance: f32,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(distance: f32, normal: Vec3) -> Self {
        Self { distance, normal }
    }
    pub fn from_point_and_normal(point: Vec3, normal: Vec3) -> Self {
        Self {
            distance: point.dot(normal),
            normal,
        }
    }
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        self.normal.dot(point) - self.distance
    }
}

// https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
/// Returns distance along the ray if it intersects
pub fn ray_with_plane(ray: Ray, plane: Plane) -> Option<f32> {
    let bottom = Vec3::dot(ray.direction, plane.normal);

    if bottom == 0.0 {
        None // No intersection
    } else {
        let top = Vec3::dot((plane.distance * plane.normal) - ray.origin, plane.normal);

        if top == 0.0 {
            None // Technically it intersects the entire plane, because the line is on the plane.
                 // However for now we're just saying it doesn't intersect.
        } else {
            let distance = top / bottom;
            Some(distance)
        }
    }
}

/*
/// Returns the closest point between a bounding box and a sphere if the sphere overlaps
/// the bounding box.
pub fn sphere_with_bounding_box(
    bounding_box: BoundingBox,
    center: Vec3,
    radius: f32,
) -> Option<Vec3> {
    let closest_point = bounding_box.min.max(center.min(bounding_box.max));
    let distance = (closest_point - center).length();

    if distance < radius {
        Some(closest_point)
    } else {
        None
    }
}
*/

pub struct Frustum {
    pub left: Plane,
    pub right: Plane,
    pub top: Plane,
    pub bottom: Plane,
    pub near: Plane,
    pub far: Plane,
}

impl Frustum {
    pub fn from_matrix(matrix: &Mat4) -> Frustum {
        let row0 = matrix.row(0);
        let row1 = matrix.row(1);
        let row2 = matrix.row(2);
        let row3 = matrix.row(3);

        let left = (row3 + row0).normalized();
        let right = (row3 - row0).normalized();
        let top = (row3 - row1).normalized();
        let bottom = (row3 + row1).normalized();
        let near = (row3 + row2).normalized();
        let far = (row3 - row2).normalized();

        Frustum {
            left: Plane::new(left[3], left.xyz()),
            right: Plane::new(right[3], right.xyz()),
            top: Plane::new(top[3], top.xyz()),
            bottom: Plane::new(bottom[3], bottom.xyz()),
            near: Plane::new(near[3], near.xyz()),
            far: Plane::new(far[3], far.xyz()),
        }
    }
}

pub fn perspective_infinite_gl(
    vertical_field_of_view_radians: f32,
    aspect_ratio: f32,
    z_near: f32,
) -> Mat4 {
    let t = (vertical_field_of_view_radians / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    [
        [sx, 0., 0., 0.],
        [0., sy, 0., 0.],
        [0., 0., -1., -1.],
        [0., 0., -2. * z_near, 0.],
    ]
    .into()
}
