pub mod collision;

mod convex_mesh_collider;

use std::fmt::Debug;

use collision::{GJKEpsilon, VeryLargeNumber};
use kmath::geometry::BoundingBox;
use kmath::numeric_traits::NumericFloat;
use kmath::*;

#[derive(Clone, Copy)]
pub struct RigidBodyDataHandle(usize);

#[derive(Clone, Copy)]
pub struct ColliderDataHandle(usize);

#[derive(Clone, Copy)]
pub struct MeshDataHandle(usize);

#[derive(Clone, Debug)]
/// User data that can be used to associate with an Entity later.
pub struct AssociatedEntity {
    pub index: u32,
    pub generation: u32,
}

#[derive(Clone)]
pub struct PhysicsWorld<F: NumericFloat> {
    pub gravity: Vector<F, 3>,
    time_step: F,
    rigid_bodies: Vec<RigidBodyData<F>>,
    colliders: Vec<ColliderData<F>>,
    pub collider_meshes: Vec<MeshData<F>>,
    /// For debug purposes, a collision occurred in the last frame.
    pub collision_occurred: bool,
    pub contact_points: Vec<Vector<F, 3>>,
}

#[derive(Clone, Debug)]
pub struct RigidBodyData<F: NumericFloat> {
    pub mass: F,
    pub position: Vector<F, 3>,
    pub rotation: Quaternion<F>,
    pub velocity: Vector<F, 3>,
    pub angular_velocity: Vector<F, 3>,
    /// Also known as `Coefficient of restitution`
    /// This value determines how much energy is preserved by collisions.
    /// 1.0 means a collision preserves all energy.
    /// 0.0 means a collision loses all energy and objects will not bounce.
    /// When two objects collide their bounciness is multiplied together to determine
    /// the bounciness of the collision.
    pub bounciness: F,
    pub gravity_multiplier: F,
    pub associated_entity: AssociatedEntity,
}

#[derive(Clone)]
pub struct MeshData<F: NumericFloat> {
    pub positions: Vec<Vector<F, 3>>,
    pub normals: Vec<Vector<F, 3>>,
    pub indices: Vec<[u32; 3]>,
    pub planes: Vec<Plane<F, 3>>,
    pub inertia_tensor_divided_by_mass: Matrix<F, 3, 3>,
}

#[derive(Clone)]
pub struct ColliderData<F: NumericFloat> {
    pub offset_from_rigid_body: Vector<F, 3>,
    pub attached_rigid_body: Option<RigidBodyDataHandle>,
    pub mesh_index: MeshDataHandle,
    pub associated_entity: AssociatedEntity,
}

#[doc(hidden)]
pub trait PhysicsDefaults {
    const GRAVITY: Self;
    const TIME_STEP: Self;
}

impl PhysicsDefaults for f32 {
    const GRAVITY: Self = -9.81;
    const TIME_STEP: Self = 1.0 / 60.0;
}

impl PhysicsDefaults for f64 {
    const GRAVITY: Self = -9.81;
    const TIME_STEP: Self = 1.0 / 60.0;
}

impl<F: NumericFloat + PhysicsDefaults + Debug + GJKEpsilon + VeryLargeNumber + OneDividedBy12>
    PhysicsWorld<F>
{
    pub fn new() -> Self {
        let mut gravity = Vector::<F, 3>::ZERO;
        gravity[1] = F::GRAVITY;

        Self {
            gravity, // Meters per second
            time_step: F::TIME_STEP,
            rigid_bodies: Vec::new(),
            colliders: Vec::new(),
            collider_meshes: Vec::new(),
            collision_occurred: false,
            contact_points: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        //println!("PHYSICS UPDATE----------");
        self.collision_occurred = false;
        for rigid_body in &mut self.rigid_bodies {
            // Apply movement and gravity only to non-kinematic rigid-bodies
            if rigid_body.mass != F::INFINITY {
                // When an object is moving less than 3 centimeters per second stop its movement.
                // This probably is too aggressive, but it's the threshold that seems to work for now with the
                // precision of the current system. Perhaps it could be relaxed with 64-bit operations
                // let resting_velocity = 0.03;
                // if rigid_body.velocity.length_squared() < resting_velocity * resting_velocity {
                //     rigid_body.velocity *= 0.;
                // }

                let gravity = self.gravity * rigid_body.gravity_multiplier;
                let acceleration = gravity;
                rigid_body.position = rigid_body.position
                    + rigid_body.velocity * self.time_step
                    + acceleration * F::HALF * self.time_step * self.time_step;

                let angular_velocity_quaternion: Quaternion<F> = Quaternion::from_xyzw(
                    rigid_body.angular_velocity[0],
                    rigid_body.angular_velocity[1],
                    rigid_body.angular_velocity[2],
                    F::ZERO,
                );

                // This stack overflow answer explains why a quaternion add is used here:
                // https://stackoverflow.com/questions/46908345/integrate-angular-velocity-as-quaternion-rotation
                rigid_body.rotation = rigid_body.rotation
                    + rigid_body.rotation * angular_velocity_quaternion * F::HALF * self.time_step;

                rigid_body.rotation = rigid_body.rotation.normalized();
                rigid_body.velocity =
                    rigid_body.velocity + (acceleration + acceleration) * F::HALF * self.time_step;
            } else {
                rigid_body.velocity = Vector::<F, 3>::ZERO;
            }
        }

        let len = self.colliders.len();

        // Number of iterations to converge.
        for _ in 0..1 {
            // Check each collider pair for collision.
            // This should probably be changed to check based on relative offsets to the parent `RigidBody`.
            for i in 0..len {
                for j in i + 1..len {
                    let (a, b) = index_twice(&mut self.colliders, i, j);
                    if let Some(rigid_body_a) = a.attached_rigid_body {
                        if let Some(rigid_body_b) = b.attached_rigid_body {
                            let (rigid_body_a, rigid_body_b) =
                                index_twice(&mut self.rigid_bodies, rigid_body_a.0, rigid_body_b.0);
                            if rigid_body_a.mass != F::INFINITY || rigid_body_b.mass != F::INFINITY
                            {
                                // Ignore scale for now.
                                let a_to_world = Matrix::<F, 4, 4>::from_translation_rotation_scale(
                                    rigid_body_a.position,
                                    rigid_body_a.rotation,
                                    Vector::<F, 3>::ONE,
                                );

                                let b_to_world = Matrix::<F, 4, 4>::from_translation_rotation_scale(
                                    rigid_body_b.position,
                                    rigid_body_b.rotation,
                                    Vector::<F, 3>::ONE,
                                );

                                let mesh_a = &self.collider_meshes[a.mesh_index.0];
                                let mesh_b = &self.collider_meshes[b.mesh_index.0];

                                // Todo: Relative rotation should be calculated here as well.
                                let collision_info = collision::gjk(
                                    a_to_world,
                                    b_to_world,
                                    &mesh_a.positions,
                                    &mesh_b.positions,
                                );
                                if collision_info.collided {
                                    self.collision_occurred = true;
                                    println!("COLLIDED");

                                    let relative_velocity =
                                        rigid_body_b.velocity - rigid_body_a.velocity;

                                    // If these things aren't moving relative to each-other, don't bother colliding.
                                    if relative_velocity.length_squared() < F::GJK_EPSILON {
                                        continue;
                                    }

                                    let world_to_a = a_to_world.inversed();
                                    let world_to_b = b_to_world.inversed();

                                    // Get the relative velocity at the GJK point.
                                    let relative_velocity = {
                                        let velocity_at_point_a = rigid_body_a.velocity
                                            + rigid_body_a.angular_velocity.cross(
                                                collision_info.closest_point_a
                                                    - rigid_body_a.position,
                                            );

                                        let velocity_at_point_b = rigid_body_b.velocity
                                            + rigid_body_b.angular_velocity.cross(
                                                collision_info.closest_point_b
                                                    - rigid_body_b.position,
                                            );

                                        println!("VELOCITY AT A: {:?}", velocity_at_point_a);
                                        println!("VELOCITY AT B: {:?}", velocity_at_point_b);

                                        velocity_at_point_b - velocity_at_point_a
                                    };

                                    let relative_velocity_direction =
                                        relative_velocity.normalized();

                                    println!(
                                        "RELATIVE VELOCITY DIRECTION: {:?}",
                                        relative_velocity_direction
                                    );
                                    // Find the shortest separation direction along the relative velocity direction.
                                    let orientation_along_velocity_to_separate = {
                                        let direction_a = world_to_a
                                            .transform_vector(relative_velocity_direction);
                                        let direction_b = world_to_b
                                            .transform_vector(relative_velocity_direction);

                                        let (min_a, max_a) =
                                            collision::find_min_max_along_direction(
                                                direction_a,
                                                &mesh_a.positions,
                                            );
                                        let offset_a = a_to_world
                                            .transform_point(Vector::ZERO)
                                            .dot(relative_velocity_direction);
                                        let (min_a, max_a) = (min_a + offset_a, max_a + offset_a);
                                        let (min_b, max_b) =
                                            collision::find_min_max_along_direction(
                                                direction_b,
                                                &mesh_b.positions,
                                            );
                                        let offset_b = b_to_world
                                            .transform_point(Vector::ZERO)
                                            .dot(relative_velocity_direction);
                                        let (min_b, max_b) = (min_b + offset_b, max_b + offset_b);
                                        // Probably need to scale min and max here once scale is accounted for.

                                        // Find direction along velocity to separate.
                                        if (max_b - min_a) > (max_a - min_b) {
                                            -F::ONE
                                        } else {
                                            F::ONE
                                        }
                                    };

                                    // This plane points away from a towards b.
                                    let contact_plane = Plane::new(
                                        relative_velocity_direction
                                            * -orientation_along_velocity_to_separate,
                                        // Both collision points are equivalent in the event of a collision.
                                        collision_info.closest_point_a,
                                    );

                                    println!("CONTACT PLANE: {:?}", &contact_plane);

                                    // They're moving apart, do nothing
                                    if contact_plane.normal.dot(relative_velocity) > F::ZERO {
                                        println!("MOVING APART");
                                        continue;
                                    }

                                    // Find the contact points for this collision.
                                    let contact_points = collision::find_contact_points_on_plane(
                                        contact_plane,
                                        &mesh_a.planes,
                                        &mesh_b.planes,
                                        &a_to_world,
                                        &b_to_world,
                                    );

                                    // Calculate how much each object should respond based on their relative masses.
                                    let mut response_b =
                                        rigid_body_a.mass / (rigid_body_a.mass + rigid_body_b.mass);

                                    if response_b.is_nan_numeric() {
                                        response_b = F::ONE;
                                    }

                                    let mut response_a =
                                        rigid_body_b.mass / (rigid_body_b.mass + rigid_body_b.mass);

                                    if response_a.is_nan_numeric() {
                                        response_a = F::ONE;
                                    }

                                    // Add additional bounce
                                    // Low bounce objects absord some of the impact force.
                                    // This simulates in the real world how less bouncy collisions lose energy
                                    // to heat, sound waves, etc.
                                    let bounciness =
                                        rigid_body_a.bounciness * rigid_body_b.bounciness;

                                    // let impulse = impulse + impulse * bounciness;

                                    println!("MASS RATIO A: {:?}", response_b);
                                    println!("MASS RATIO B: {:?}", response_a);

                                    let tensor_a =
                                        mesh_a.inertia_tensor_divided_by_mass * rigid_body_a.mass;

                                    let tensor_b =
                                        mesh_b.inertia_tensor_divided_by_mass * rigid_body_b.mass;

                                    // Todo: Store inversed tensor instead, because usually the inverse is needed.
                                    let inverse_tensor_a: Matrix<F, 3, 3> =
                                        if rigid_body_a.mass == F::INFINITY {
                                            Matrix::ZERO
                                        } else {
                                            tensor_a.inversed()
                                        };
                                    let inverse_tensor_b: Matrix<F, 3, 3> =
                                        if rigid_body_b.mass == F::INFINITY {
                                            Matrix::ZERO
                                        } else {
                                            tensor_b.inversed()
                                        };

                                    println!("INVERSE TENSOR A: {:?}", inverse_tensor_a);
                                    println!("INVERSE TENSOR B: {:?}", inverse_tensor_b);

                                    let velocity_a = rigid_body_a.velocity;
                                    let angular_velocity_a = rigid_body_a.angular_velocity;

                                    let velocity_b = rigid_body_b.velocity;
                                    let angular_velocity_b = rigid_body_b.angular_velocity;

                                    let mut velocity_change_a = Vector::ZERO;
                                    let mut velocity_change_b = Vector::ZERO;

                                    let mut angular_velocity_change_a = Vector::ZERO;
                                    let mut angular_velocity_change_b = Vector::ZERO;

                                    for &point in contact_points.iter() {
                                        let velocity_at_point_a = velocity_a
                                            + angular_velocity_a
                                                .cross(point - rigid_body_a.position);

                                        let velocity_at_point_b = velocity_b
                                            + angular_velocity_b
                                                .cross(point - rigid_body_b.position);

                                        let relative_velocity_at_point =
                                            velocity_at_point_b - velocity_at_point_a;

                                        // Solve the impulse equation:

                                        let numerator = (relative_velocity_at_point
                                            * -(F::ONE + bounciness))
                                            .dot(contact_plane.normal);

                                        let term0 = F::ONE / rigid_body_a.mass;
                                        let term1 = F::ONE / rigid_body_b.mass;

                                        let ra = point - rigid_body_a.position;
                                        let rb = point - rigid_body_b.position;

                                        let ra_cross_normal = ra.cross(contact_plane.normal);
                                        let rb_cross_normal = rb.cross(contact_plane.normal);

                                        let term2 = inverse_tensor_a * (ra_cross_normal).cross(ra);
                                        let term3 = inverse_tensor_b * (rb_cross_normal).cross(rb);

                                        let impulse_magnitude = numerator
                                            / (term0
                                                + term1
                                                + (term2 + term3).dot(contact_plane.normal));
                                        let impulse = contact_plane.normal * impulse_magnitude;

                                        println!("VELOCITY before: {:?}", rigid_body_a.velocity);

                                        velocity_change_a = -impulse / rigid_body_a.mass;
                                        angular_velocity_change_a -= (inverse_tensor_a
                                            * ra_cross_normal)
                                            * impulse_magnitude;

                                        println!(
                                            "ANGULAR VELOCITY CHANGE: {:?}",
                                            (inverse_tensor_a * ra_cross_normal)
                                                * -impulse_magnitude
                                        );
                                        println!("VELOCITY CHANGE A: {:#?}", -impulse * response_a);
                                        println!("VELOCITY after: {:?}", rigid_body_a.velocity);

                                        velocity_change_b = -impulse / rigid_body_b.mass;
                                        angular_velocity_change_b += (inverse_tensor_b
                                            * rb_cross_normal)
                                            * impulse_magnitude;
                                    }

                                    println!("CONTACT POINTS: {:?}", contact_points);
                                    self.contact_points = contact_points;

                                    rigid_body_a.velocity += velocity_change_a;
                                    println!("VELOCITY CHANGE TOTAL: {:?}", velocity_change_a);
                                    rigid_body_b.velocity += velocity_change_b;

                                    rigid_body_a.angular_velocity += angular_velocity_change_a;
                                    println!(
                                        "ANGULAR VELOCITY CHANGE TOTAL: {:?}",
                                        angular_velocity_change_a
                                    );
                                    rigid_body_b.angular_velocity += angular_velocity_change_b;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Position is relative to the `RigidBody`
    pub fn apply_force_at_position(
        &mut self,
        rigid_body_data_handle: RigidBodyDataHandle,
        position: Vector<F, 3>,
        force: Vector<F, 3>,
    ) {
        let linear_force = force;

        // 3D torque is an axis and a magnitude.
        // In 2D it's just a magnitude because
        // there's only one axis.
        let torque = force.cross(position);

        let rigid_body = self.get_rigid_body_data_mut(rigid_body_data_handle);
        Self::apply_linear_force_inner(rigid_body, linear_force);
        Self::apply_torque_inner(rigid_body, torque)
    }

    #[inline]
    fn apply_torque_inner(rigid_body: &mut RigidBodyData<F>, torque: Vector<F, 3>) {
        let angular_velocity_change = torque / rigid_body.mass;
        rigid_body.angular_velocity += angular_velocity_change;
    }

    #[inline]
    fn apply_linear_force_inner(rigid_body: &mut RigidBodyData<F>, linear_force: Vector<F, 3>) {
        let velocity_change = linear_force / rigid_body.mass;
        rigid_body.velocity += velocity_change;
    }

    /// Position and force are relative to the `RigidBody`
    /// A force with length `1.0` for a `RigidBody` with mass 1.0 will
    /// have its velocity increase by 1.0 meter per second.
    /// The force is applied as an impulse applied for a single time-step
    pub fn apply_linear_force(
        &mut self,
        rigid_body_data_handle: RigidBodyDataHandle,
        linear_force: Vector<F, 3>,
    ) {
        let rigid_body = self.get_rigid_body_data_mut(rigid_body_data_handle);
        Self::apply_linear_force_inner(rigid_body, linear_force);
    }

    /// Torque is relative to the `RigidBody`.
    pub fn apply_torque(
        &mut self,
        rigid_body_data_handle: RigidBodyDataHandle,
        torque: Vector<F, 3>,
    ) {
        // Need to account for inertial tensor here.
        let rigid_body = self.get_rigid_body_data_mut(rigid_body_data_handle);
        Self::apply_torque_inner(rigid_body, torque);
    }

    pub fn get_rigid_body_data(
        &self,
        rigid_body_data_handle: RigidBodyDataHandle,
    ) -> &RigidBodyData<F> {
        &self.rigid_bodies[rigid_body_data_handle.0]
    }

    pub fn get_rigid_body_data_mut(
        &mut self,
        rigid_body_data_handle: RigidBodyDataHandle,
    ) -> &mut RigidBodyData<F> {
        &mut self.rigid_bodies[rigid_body_data_handle.0]
    }

    pub fn get_collider_data(&self, collider_data_handle: ColliderDataHandle) -> &ColliderData<F> {
        &self.colliders[collider_data_handle.0]
    }

    pub fn get_collider_data_mut(
        &mut self,
        collider_data_handle: ColliderDataHandle,
    ) -> &mut ColliderData<F> {
        &mut self.colliders[collider_data_handle.0]
    }

    pub fn new_rigid_body(&mut self, rigid_body: RigidBodyData<F>) -> RigidBodyDataHandle {
        println!("NEW RIGID BODY: {:#?}", rigid_body);
        self.rigid_bodies.push(rigid_body);
        RigidBodyDataHandle(self.rigid_bodies.len() - 1)
    }

    /// Position is its position relative to the world.
    pub fn new_collider(&mut self, collider: ColliderData<F>) -> ColliderDataHandle {
        self.colliders.push(collider);
        ColliderDataHandle(self.colliders.len() - 1)
    }

    pub fn update_collider_position_and_scale(
        &mut self,
        collider_data_handle: ColliderDataHandle,
        position: Vector<F, 3>,
        _scale: Vector<F, 3>,
    ) {
        let collider = &mut self.colliders[collider_data_handle.0];
        if let Some(rigid_body_handle) = collider.attached_rigid_body {
            let parent_rigid_body_position = self.rigid_bodies[rigid_body_handle.0].position;
            let offset = position - parent_rigid_body_position;
            collider.offset_from_rigid_body = offset;
        }
    }

    pub fn add_mesh_data(
        &mut self,
        positions: &[Vector<F, 3>],
        normals: &[Vector<F, 3>],
        indices: &[[u32; 3]],
    ) -> MeshDataHandle {
        // Calculate planes for this mesh, but deduplicate.
        let mesh_data = create_mesh_data(positions, normals, indices);

        self.collider_meshes.push(mesh_data);

        MeshDataHandle(self.collider_meshes.len() - 1)
    }

    pub fn get_mesh_data(&self, mesh_data_handle: &MeshDataHandle) -> &MeshData<F> {
        self.collider_meshes.get(mesh_data_handle.0).unwrap()
    }
}

pub trait OneDividedBy12 {
    const ONE_DIVIDED_BY_12: Self;
}

impl OneDividedBy12 for f32 {
    const ONE_DIVIDED_BY_12: Self = 1.0 / 12.0;
}

impl OneDividedBy12 for f64 {
    const ONE_DIVIDED_BY_12: Self = 1.0 / 12.0;
}

pub fn create_mesh_data<F: NumericFloat + GJKEpsilon + OneDividedBy12>(
    positions: &[Vector<F, 3>],
    normals: &[Vector<F, 3>],
    indices: &[[u32; 3]],
) -> MeshData<F> {
    // Calculate planes for this mesh, but deduplicate.
    let mut planes: Vec<Plane<F, 3>> = Vec::new();

    for [i0, i1, i2] in indices {
        let (p0, p1, p2) = (
            positions[*i0 as usize],
            positions[*i1 as usize],
            positions[*i2 as usize],
        );
        let normal = ((p1 - p0).cross(p2 - p1)).normalized();
        let plane = Plane::new(normal, p0);
        // Check that this plane does not already exist.
        let mut new_plane = true;
        for p in planes.iter() {
            let matches = (p.normal - normal).length_squared() < F::GJK_EPSILON
                && (p.distance_along_normal - plane.distance_along_normal).numeric_abs()
                    < F::GJK_EPSILON;
            if matches {
                new_plane = false;
                break;
            }
        }
        if new_plane {
            planes.push(plane);
        }
    }

    // Use bounding box to approximate tensor
    // Using the tensor for a cuboid.
    let bounds = BoundingBox::from_points(positions);
    let size = bounds.size();
    let size2 = size.mul_by_component(size);

    let inertia_tensor_divided_by_mass = [
        [F::ONE_DIVIDED_BY_12 * (size2.y + size2.z), F::ZERO, F::ZERO],
        [F::ZERO, F::ONE_DIVIDED_BY_12 * (size2.x + size2.z), F::ZERO],
        [F::ZERO, F::ZERO, F::ONE_DIVIDED_BY_12 * (size2.x * size2.y)],
    ]
    .into();

    MeshData {
        positions: positions.into(),
        normals: normals.into(),
        indices: indices.into(),
        planes,
        inertia_tensor_divided_by_mass,
    }
}

/// A helper to get two mutable borrows from the same slice.
fn index_twice<T>(slice: &mut [T], first: usize, second: usize) -> (&mut T, &mut T) {
    if first < second {
        let (a, b) = slice.split_at_mut(second);
        (&mut a[first], &mut b[0])
    } else {
        let (a, b) = slice.split_at_mut(first);
        (&mut b[0], &mut a[second])
    }
}
