use std::fmt::Debug;

use kmath::numeric_traits::NumericFloat;
use kmath::*;

pub trait GJKEpsilon {
    const GJK_EPSILON: Self;
}

// These consts are chosen totally arbitrarily.
impl GJKEpsilon for f32 {
    const GJK_EPSILON: Self = 0.00001;
}
impl GJKEpsilon for f64 {
    const GJK_EPSILON: Self = 0.00001;
}

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

impl<T: Copy + Default, const SIZE: usize> std::ops::DerefMut for StackVec<T, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
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

#[derive(Copy, Clone, Default, Debug)]
struct SimplexVertex<F: NumericFloat> {
    point_a: Vector<F, 3>, // Support point in polygon a
    point_b: Vector<F, 3>, // Support point in polygon b
    point: Vector<F, 3>,   // point_b - point_a
    u: F,                  // unnormalized barycentric coordinate for closest point,
}

#[derive(Debug, Clone)]
pub struct CollisionInfo<F: NumericFloat> {
    pub collided: bool,
    pub closest_point_a: Vector<F, 3>,
    pub closest_point_b: Vector<F, 3>,
}

pub fn gjk<F: NumericFloat + Debug + GJKEpsilon>(
    a_to_world: Matrix<F, 4, 4>,
    b_to_world: Matrix<F, 4, 4>,
    shape_a: &[Vector<F, 3>],
    shape_b: &[Vector<F, 3>],
) -> CollisionInfo<F> {
    let world_to_a = a_to_world.inversed();
    let world_to_b = b_to_world.inversed();

    let mut simplex = Simplex::<F>::new();

    let mut iterations = 0;

    let mut last_simplex_distance = F::MAX;

    loop {
        if iterations > 1000 {
            panic!()
        }
        iterations += 1;
        simplex.evolve();

        let closest_simplex_point = simplex.closest_simplex_point();
        let search_direction = match simplex.points.count {
            0 => Vector::<F, 3>::X, // Completely arbitrary search direction for now
            1 | 2 | 3 => -closest_simplex_point,
            4 => {
                // The simplex is still a tetehedron which means the
                // origin was contained within it which indicates an overlap
                let (closest_point_a, closest_point_b) = simplex.closest_points();
                return CollisionInfo {
                    collided: true,
                    closest_point_a,
                    closest_point_b,
                };
            }
            _ => unreachable!(),
        };

        if simplex.points.count != 0 {
            let distance_squared = closest_simplex_point.length_squared();

            if distance_squared >= last_simplex_distance {
                // Not getting closer to origin

                let (closest_point_a, closest_point_b) = simplex.closest_points();
                return CollisionInfo {
                    collided: false,
                    closest_point_a,
                    closest_point_b,
                };
            }
            last_simplex_distance = distance_squared;
        }

        // This is probably wrong.
        if search_direction.length_squared() <= F::GJK_EPSILON {

            let (closest_point_a, closest_point_b) = simplex.closest_points();
            return CollisionInfo {
                collided: true,
                closest_point_a,
                closest_point_b,
            };
        }

        // Transform the direction into the space of each polyhedron
        // and then find the supports.
        let da = world_to_a.transform_vector(search_direction);
        let db = world_to_b.transform_vector(-search_direction);

        let support_a = find_support(shape_a, da);
        let support_b = find_support(shape_b, db);
        let support_a = a_to_world.transform_point(support_a);
        let support_b = b_to_world.transform_point(support_b);

        let support = support_a - support_b;

        // The following check can be used to early out if we're just looking for intersection and don't
        // care about the closest points on the two objects.
        // if support.dot(search_direction) < F::ZERO

        simplex.points.push(SimplexVertex {
            point_a: support_a,
            point_b: support_b,
            point: support,
            u: F::ONE, // This wil be set later.
        });
    }
}

#[derive(Debug)]
struct Simplex<F: NumericFloat> {
    points: StackVec<SimplexVertex<F>, 4>,
}

impl<F: NumericFloat> Simplex<F> {
    fn new() -> Self {
        Self {
            points: StackVec::new(),
        }
    }

    /// Calculate the point on the simplex closest to the origin
    fn closest_simplex_point(&self) -> Vector<F, 3> {
        // Denominator
        let mut denom = F::ZERO;
        for p in &self.points[0..self.points.count] {
            denom = denom + p.u;
        }
        denom = F::ONE / denom;

        let mut u = [F::ZERO; 4];
        for i in 0..self.points.count {
            u[i] = self.points[i].u * denom;
        }

        let mut a = Vector::<F, 3>::ZERO;

        for i in 0..self.points.count {
            a += self.points[i].point * u[i];
        }
        a
    }

    /// Calculates the closest world space points from their barycentric coordinates.
    fn closest_points(&self) -> (Vector<F, 3>, Vector<F, 3>) {
        // Denominator
        let mut denom = F::ZERO;
        for p in &self.points[0..self.points.count] {
            denom = denom + p.u;
        }
        denom = F::ONE / denom;

        let mut u = [F::ZERO; 4];
        for i in 0..self.points.count {
            u[i] = self.points[i].u * denom;
        }

        let mut a = Vector::<F, 3>::ZERO;
        let mut b = Vector::<F, 3>::ZERO;

        for i in 0..self.points.count {
            a += self.points[i].point_a * u[i];
            b += self.points[i].point_b * u[i]
        }
        (a, b)
    }

    fn evolve(&mut self) {
        match self.points.count {
            0 | 1 => {}
            2 => self.edge_simplex(),
            3 => self.triangle_simplex(),
            4 => self.tetrahedron_simplex(),
            _ => unreachable!(),
        }
    }

    fn edge_simplex(&mut self) {
        let a = self.points[0].point;
        let b = self.points[1].point;

        let u = b.dot(b - a);
        let v = a.dot(a - b);

        // A little diagram of the regions of the voronoi regions of a line segment:
        // A |----AB----| B

        if v <= F::ZERO {
            // Region A
            // B isn't contributing to the simplex so remove it.
            self.points.remove(1);
            self.points[0].u = F::ONE;
        } else if v <= F::ZERO {
            // Region B
            // A isn't contributing to the simplex so remove it.
            self.points.remove(0);
            self.points[0].u = F::ONE;
        } else {
            // Region AB
            // Both vertices are contributing, so keep them.
            self.points[0].u = u;
            self.points[1].u = v;
        }
    }

    fn triangle_simplex(&mut self) {
        let a = self.points[0].point;
        let b = self.points[1].point;
        let c = self.points[2].point;

        let ba = b - a;
        let ca = c - a;
        let ab = a - b;
        let cb = c - b;
        let bc = b - c;
        let ac = a - c;

        // Compute edge barycentric coordinates (pre-division).
        let u_ab = b.dot(ba);
        let v_ab = a.dot(ab);

        let u_bc = c.dot(cb);
        let v_bc = b.dot(bc);

        let u_ca = a.dot(ac);
        let v_ca = c.dot(ca);

        // These first three tests check if the origin is closest to a corner
        if v_ab <= F::ZERO && u_ca <= F::ZERO {
            // Region A
            // Remove B, C
            self.points.count = 1;
            self.points[0].u = F::ONE;
        } else if u_ab <= F::ZERO && v_bc <= F::ZERO {
            // Region B
            // Remove A, C
            self.points[0] = self.points[1];
            self.points.count = 1;
            self.points[0].u = F::ONE;
        } else if u_bc <= F::ZERO && v_ca <= F::ZERO {
            // Region C
            // Remove A, B
            self.points[0] = self.points[2];
            self.points.count = 1;
            self.points[0].u = F::ONE;
        } else {
            // Triangle area is the magnitude of the cross of the sides

            // Normal of the triangle
            let n = ba.cross(ca);

            // Calculate the area of the triangles formed with the origin.
            // Because it's the origin the edges are simply the points.
            let n0 = b.cross(c);
            let n1 = c.cross(a);
            let n2 = a.cross(b);

            // Calculate the area of the sub-triangles (the barycentric coordinates)
            let u_abc = n.dot(n0);
            let v_abc = n.dot(n1);
            let w_abc = n.dot(n2);

            if u_ab > F::ZERO && v_ab > F::ZERO && w_abc <= F::ZERO {
                // Region AB
                // Remove C
                self.points[0].u = u_ab;
                self.points[1].u = v_ab;
                self.points.count = 2;
            } else if u_bc > F::ZERO && v_bc > F::ZERO && u_abc <= F::ZERO {
                // Region BC
                // Remove A
                self.points[0] = self.points[1];
                self.points[1] = self.points[2];
                self.points.count = 2;
                self.points[0].u = u_bc;
                self.points[1].u = v_bc;
            } else if u_ca > F::ZERO && v_ca > F::ZERO && v_abc <= F::ZERO {
                // Region CA
                // Remove B, reorder A and C
                self.points[1] = self.points[0];
                self.points[0] = self.points[2];
                self.points.count = 2;
                self.points[0].u = u_ca;
                self.points[1].u = v_ca;
            } else {
                // Region ABC
                assert!(u_abc > F::ZERO && v_abc > F::ZERO && w_abc > F::ZERO);
                self.points[0].u = u_abc;
                self.points[1].u = v_abc;
                self.points[2].u = w_abc;
            }
        }
    }

    fn tetrahedron_simplex(&mut self) {
        let a = self.points[0].point;
        let b = self.points[1].point;
        let c = self.points[2].point;
        let d = self.points[3].point;

        let ba = b - a;
        let ca = c - a;
        let ab = a - b;
        let cb = c - b;
        let bc = b - c;
        let ac = a - c;

        let db = d - b;
        let bd = b - d;
        let dc = d - c;
        let cd = c - d;
        let da = d - a;
        let ad = a - d;

        // Compute barycentric coordinates
        let u_ab = b.dot(ba);
        let v_ab = a.dot(ab);

        let u_bc = c.dot(cb);
        let v_bc = b.dot(bc);

        let u_ca = a.dot(ac);
        let v_ca = c.dot(ca);

        let u_bd = d.dot(db);
        let v_bd = b.dot(bd);

        let u_dc = c.dot(cd);
        let v_dc = d.dot(dc);

        let u_ad = d.dot(da);
        let v_ad = a.dot(ad);

        /* check verticies for closest point */
        if v_ab <= F::ZERO && u_ca <= F::ZERO && v_ad <= F::ZERO {
            // Region A
            // Remove B, C, D
            self.points.count = 1;
            self.points[0].u = F::ONE;
            return;
        }
        if u_ab <= F::ZERO && v_bc <= F::ZERO && v_bd <= F::ZERO {
            // Region B
            // Remove A, C, D
            self.points[0] = self.points[1];
            self.points.count = 1;
            self.points[0].u = F::ONE;
            return;
        }
        if u_bc <= F::ZERO && v_ca <= F::ZERO && u_dc <= F::ZERO {
            // Region C
            // Remove A, B, D
            self.points[0] = self.points[2];
            self.points.count = 1;
            self.points[0].u = F::ONE;
            return;
        }
        if u_bd <= F::ZERO && v_dc <= F::ZERO && u_ad <= F::ZERO {
            // Region D
            // Remove A, B, C
            self.points[0] = self.points[3];
            self.points.count = 1;
            self.points[0].u = F::ONE;
            return;
        }

        /* calculate fractional area */
        let n = da.cross(ba);
        let n1 = d.cross(b);
        let n2 = b.cross(a);
        let n3 = a.cross(d);

        let u_adb = n1.dot(n);
        let v_adb = n2.dot(n);
        let w_adb = n3.dot(n);

        let n = ca.cross(da);
        let n1 = c.cross(d);
        let n2 = d.cross(a);
        let n3 = a.cross(c);

        let u_acd = n1.dot(n);
        let v_acd = n2.dot(n);
        let w_acd = n3.dot(n);

        let n = bc.cross(dc);
        let n1 = b.cross(d);
        let n2 = d.cross(c);
        let n3 = c.cross(b);

        let u_cbd = n1.dot(n);
        let v_cbd = n2.dot(n);
        let w_cbd = n3.dot(n);
        let n = ba.cross(ca);
        let n1 = b.cross(c);
        let n2 = c.cross(a);
        let n3 = a.cross(b);

        let u_abc = n1.dot(n);
        let v_abc = n2.dot(n);
        let w_abc = n3.dot(n);

        /* check edges for closest point */
        if w_abc <= F::ZERO && v_adb <= F::ZERO && u_ab > F::ZERO && v_ab > F::ZERO {
            /* region AB */
            // Remove C D
            self.points[0].u = u_ab;
            self.points[1].u = v_ab;
            self.points.count = 2;
            return;
        }
        if u_abc <= F::ZERO && w_cbd <= F::ZERO && u_bc > F::ZERO && v_bc > F::ZERO {
            /* region BC */
            // Remove A D
            self.points[0] = self.points[1];
            self.points[1] = self.points[2];
            self.points.count = 2;
            self.points[0].u = u_bc;
            self.points[1].u = v_bc;
            return;
        }
        if v_abc <= F::ZERO && w_acd <= F::ZERO && u_ca > F::ZERO && v_ca > F::ZERO {
            /* region CA */
            // Remove B D, swap C A
            self.points[1] = self.points[0];
            self.points[0] = self.points[2];
            self.points.count = 2;
            self.points[0].u = u_ca;
            self.points[1].u = v_ca;
            return;
        }
        if v_cbd <= F::ZERO && u_acd <= F::ZERO && u_dc > F::ZERO && v_dc > F::ZERO {
            /* region DC */
            // Remove A B, swap D C
            self.points[0] = self.points[3];
            self.points[1] = self.points[2];
            self.points.count = 2;
            self.points[0].u = u_dc;
            self.points[1].u = v_dc;
            return;
        }
        if v_acd <= F::ZERO && w_adb <= F::ZERO && u_ad > F::ZERO && v_ad > F::ZERO {
            /* region AD */
            // Remove B C
            self.points[1] = self.points[3];
            self.points.count = 2;
            self.points[0].u = u_ad;
            self.points[1].u = v_ad;
            return;
        }
        if u_cbd <= F::ZERO && u_adb <= F::ZERO && u_bd > F::ZERO && v_bd > F::ZERO {
            /* region BD */
            // Remove A C
            self.points[0] = self.points[1];
            self.points[1] = self.points[3];
            self.points.count = 2;
            self.points[0].u = u_bd;
            self.points[1].u = v_bd;
            return;
        }

        /* calculate fractional volume (volume can be negative!) */
        let denom = cb.cross(ab).dot(db);
        let volume = if denom == F::ZERO {
            F::ONE
        } else {
            F::ONE / denom
        };
        let u_abcd = c.cross(d).dot(b) * volume;
        let v_abcd = c.cross(a).dot(d) * volume;
        let w_abcd = d.cross(a).dot(b) * volume;
        let x_abcd = b.cross(a).dot(c) * volume;

        /* check faces for closest point */
        if x_abcd < F::ZERO && u_abc > F::ZERO && v_abc > F::ZERO && w_abc > F::ZERO {
            /* region ABC */
            // Remove D
            self.points[0].u = u_abc;
            self.points[1].u = v_abc;
            self.points[2].u = w_abc;
            self.points.count = 3;
            return;
        }
        if u_abcd < F::ZERO && u_cbd > F::ZERO && v_cbd > F::ZERO && w_cbd > F::ZERO {
            /* region CBD */
            // Remove A, swap BC
            self.points[0] = self.points[2];
            self.points[2] = self.points[3];

            self.points[0].u = u_cbd;
            self.points[1].u = v_cbd;
            self.points[2].u = w_cbd;
            self.points.count = 3;
            return;
        }
        if v_abcd < F::ZERO && u_acd > F::ZERO && v_acd > F::ZERO && w_acd > F::ZERO {
            /* region ACD */
            // Remove B
            self.points[1] = self.points[2];
            self.points[2] = self.points[3];

            self.points[0].u = u_acd;
            self.points[1].u = v_acd;
            self.points[2].u = w_acd;
            self.points.count = 3;
            return;
        }
        if w_abcd < F::ZERO && u_adb > F::ZERO && v_adb > F::ZERO && w_adb > F::ZERO {
            /* region ADB */
            // Remove C, swap D and B
            self.points[2] = self.points[1];
            self.points[1] = self.points[3];

            self.points[0].u = u_adb;
            self.points[1].u = v_adb;
            self.points[2].u = w_adb;
            self.points.count = 3;
            return;
        }
        /* region ABCD */
        assert!(u_abcd >= F::ZERO && v_abcd >= F::ZERO && w_abcd >= F::ZERO && x_abcd >= F::ZERO);
        self.points[0].u = u_abcd;
        self.points[1].u = v_abcd;
        self.points[2].u = w_abcd;
        self.points[3].u = x_abcd;
    }
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

    let world_to_a = Mat4::IDENTITY;
    let world_to_b = Mat4::from_translation(Vec3::fill(0.5));
    let result = gjk(world_to_a, world_to_b, &shape_a, &shape_a);
    assert!(result.collided);

    let world_to_b = Mat4::from_translation(Vec3::fill(2.5));
    let result = gjk(world_to_a, world_to_b, &shape_a, &shape_a);
    assert!(!result.collided);
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

    let a_to_world = Mat4::IDENTITY;
    let b_to_world = Mat4::from_translation(Vec3::X * 0.5);
    let result = gjk(a_to_world, b_to_world, &shape_a, &shape_a);
    assert!(result.collided);
}

#[test]
fn cube_vs_cube_points() {
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

    let world_to_a = Mat4::IDENTITY;
    let world_to_b = Mat4::from_translation(Vec3::fill(-1.5));
    let result = gjk(world_to_a, world_to_b, &shape_a, &shape_a);
    println!("RESULT: {:#?}", result);
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

    let a_to_world = Mat4::from_translation(Vec3::X * 1.5);
    let b_to_world = Mat4::IDENTITY;
    let result = gjk(a_to_world, b_to_world, &shape_a, &shape_a);
    assert!(!result.collided);
}
