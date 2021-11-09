use kmath::numeric_traits::NumericFloat;
use kmath::*;

// A helper data structure that makes the GJK algorithm a bit cleaner.
#[derive(Debug)]
struct StackVec<T: Copy + Default, const SIZE: usize> {
    items: [T; SIZE],
    count: usize,
}

impl<T: Copy + Default, const SIZE: usize> StackVec<T, SIZE> {
    fn new() -> Self {
        Self {
            items: [T::default(); SIZE],
            count: 0,
        }
    }

    fn push(&mut self, item: T) {
        self.items[self.count] = item;
        self.count += 1;
    }

    fn remove(&mut self, index: usize) {
        for i in index + 1..SIZE {
            self.items[i - 1] = self.items[i];
        }
        self.count -= 1;
    }
}

impl<T: Copy + Default, const SIZE: usize> std::ops::Deref for StackVec<T, SIZE> {
    type Target = [T; SIZE];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

// Later this should take in other types of colliders instead of just sets of points.
// This GJK implementation is modeled after the approach described here:
// https://blog.hamaluik.ca/posts/building-a-collision-engine-part-3-3d-gjk-collision-detection/
pub fn gjk<F: NumericFloat + core::fmt::Debug>(
    shape_a: &[Vector<F, 3>],
    shape_b: &[Vector<F, 3>],
) -> bool {
    let mut simplex = StackVec::<Vector<F, 3>, 4>::new();
    let mut direction = (shape_b[0] - shape_a[0]).normalized();

    let mut iterations = 0;
    // Evolve a simplex in Minkowski Difference space to attempt to enclose the origin.
    // At each iteration choose an evolution that gets closer to containing the origin.
    loop {
        iterations += 1;
        /*
        if iterations >= 30 {
            println!("SHAPE A: {:#?}", shape_a);
            println!("SHAPE B: {:#?}", shape_b);
            println!("SIMPLEX: {:#?}", simplex);
        }*/
        assert!(iterations < 30);

        match simplex.count {
            0 => {}
            1 => direction *= -F::ONE,
            2 => {
                let ab = simplex[1] - simplex[0];
                let ao = -simplex[0];

                // Calculate a new direction perpendicular to ab
                // in the direction of the origin.
                direction = ab.cross(ao).cross(ab);
            }
            3 => {
                let ac = simplex[2] - simplex[0];
                let ab = simplex[1] - simplex[0];
                direction = ac.cross(ab);

                // Ensure the direction faces the origin.
                // Why can't we just use the triple-product approach here?
                let d = direction.dot(-simplex[2]);
                if d < F::ZERO {
                    direction = -direction;
                }
            }
            4 => {
                // Check if the origin is within the simplex tetrahedron.
                let da = simplex[3] - simplex[0];
                let db = simplex[3] - simplex[1];
                let dc = simplex[3] - simplex[2];

                // Why is `d_origin` different from earlier `ao`?
                let d_origin = -simplex[3];
                let abd_normal = da.cross(db);
                let bcd_normal = db.cross(dc);
                let cad_normal = dc.cross(da);

                // In the case of 2D-intersecting polygons the tetahedron will be completely flat
                // and all the normals will point the same direction. How / should that be handled?
                // Maybe all colliders need a bit of depth?

                // Check if the origin is within the planes of the polyhedron and refine the simplex if not.
                if abd_normal.dot(d_origin) > F::ZERO {
                    simplex.remove(2);
                    direction = abd_normal;
                } else if bcd_normal.dot(d_origin) > F::ZERO {
                    simplex.remove(0);
                    direction = bcd_normal;
                } else if cad_normal.dot(d_origin) > F::ZERO {
                    simplex.remove(1);
                    direction = cad_normal;
                } else {
                    // The origin is inside, we've found an intersection!
                    return true;
                }
            }
            _ => unreachable!(),
        }

        // Add a point here.
        let support_a = find_support(shape_a, direction);
        let support_b = find_support(shape_b, -direction);
        let support = support_a - support_b;

        let d = support.dot(direction);
        // If the new point is not beyond the origin then there can be no intersection.
        if d < F::ZERO {
            return false;
        }
        println!("direction: {:?}", direction);
        simplex.push(support);
    }
}

fn find_support<F: NumericFloat>(points: &[Vector<F, 3>], direction: Vector<F, 3>) -> Vector<F, 3> {
    let mut max_distance = F::MIN;
    let mut max_point = Vector::<F, 3>::ZERO;
    for point in points {
        let distance = direction.dot(*point);
        if distance > max_distance {
            max_distance = distance;
            max_point = *point;
        }
    }
    max_point
}

#[test]
fn cube_vs_cube0() {
    let shape_a = [
        Vec3::ZERO,
        Vec3::X,
        Vec3::X + Vec3::Z,
        Vec3::Z,
        Vec3::ZERO + Vec3::Y,
        Vec3::X + Vec3::Y,
        Vec3::X + Vec3::Z + Vec3::Y,
        Vec3::Z + Vec3::Y,
    ];

    let shape_b: Vec<_> = shape_a.iter().map(|a| *a + Vec3::fill(0.5)).collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(result);

    let shape_b: Vec<_> = shape_a.iter().map(|a| *a + Vec3::fill(-0.5)).collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(result);

    let shape_b: Vec<_> = shape_a.iter().map(|a| *a + Vec3::Y * 0.5).collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(result);

    let shape_b: Vec<_> = shape_a.iter().map(|a| *a + Vec3::Y * -0.5).collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(result);
}

#[test]
fn cube_vs_cube1() {
    let shape_a = [
        Vec3::ZERO,
        Vec3::X,
        Vec3::X + Vec3::Z,
        Vec3::Z,
        Vec3::ZERO + Vec3::Y,
        Vec3::X + Vec3::Y,
        Vec3::X + Vec3::Z + Vec3::Y,
        Vec3::Z + Vec3::Y,
    ];

    let shape_b: Vec<_> = shape_a.iter().map(|a| *a + Vec3::fill(2.0)).collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(!result);

    let result = gjk(&shape_b, &shape_a);
    assert!(!result);
}

#[test]
fn cube_vs_cube_on_edge() {
    let shape_a = [
        Vec3::ZERO,
        Vec3::X,
        Vec3::X + Vec3::Z,
        Vec3::Z,
        Vec3::ZERO + Vec3::Y,
        Vec3::X + Vec3::Y,
        Vec3::X + Vec3::Z + Vec3::Y,
        Vec3::Z + Vec3::Y,
    ];

    let shape_b: Vec<_> = shape_a.iter().map(|a| *a + Vec3::X).collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(result);

    let result = gjk(&shape_b, &shape_a);
    assert!(result);

    let shape_b: Vec<_> = shape_a.iter().map(|a| *a + -Vec3::X).collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(result);
}

#[test]
fn cube_vs_cube_with_point() {
    let shape_a = [
        Vec3::ZERO,
        Vec3::X,
        Vec3::X + Vec3::Z,
        Vec3::Z,
        Vec3::ZERO + Vec3::Y,
        Vec3::X + Vec3::Y,
        Vec3::X + Vec3::Z + Vec3::Y,
        Vec3::Z + Vec3::Y,
    ];

    // This shape is entirely below the original cube other than one point that's in the center.
    let mut shape_b: Vec<_> = shape_a.iter().map(|a| *a - Vec3::Y * 1.2).collect();
    shape_b.push(Vec3::fill(0.5));
    let result = gjk(&shape_a, &shape_b);
    assert!(result);
}

#[test]
fn cube_vs_cube2() {
    let shape_a = [
        Vec3::ZERO,
        Vec3::X,
        Vec3::X + Vec3::Z,
        Vec3::Z,
        Vec3::ZERO + Vec3::Y,
        Vec3::X + Vec3::Y,
        Vec3::X + Vec3::Z + Vec3::Y,
        Vec3::Z + Vec3::Y,
    ];

    let shape_b: Vec<_> = shape_a
        .iter()
        .map(|a| *a + Vec3::new(-0.5, 0.0, 1.2))
        .collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(!result);
}

#[test]
fn plane_vs_plane() {
    let shape_a = [Vec3::ZERO, Vec3::X, Vec3::X + Vec3::Z, Vec3::Z];

    let shape_b: Vec<_> = shape_a
        .iter()
        .map(|a| *a + Vec3::new(-0.5, 0.0, 1.2))
        .collect();
    let result = gjk(&shape_a, &shape_b);
    assert!(!result);
}
