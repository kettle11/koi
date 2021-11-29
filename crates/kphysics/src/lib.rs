mod collision;

mod convex_mesh_collider;

use std::fmt::Debug;

use collision::GJKEpsilon;
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

impl<F: NumericFloat + PhysicsDefaults + Debug + GJKEpsilon> PhysicsWorld<F> {
    pub fn new() -> Self {
        let mut gravity = Vector::<F, 3>::ZERO;
        gravity[1] = F::GRAVITY;

        Self {
            gravity, // Meters per second
            time_step: F::TIME_STEP,
            rigid_bodies: Vec::new(),
            colliders: Vec::new(),
            collider_meshes: Vec::new(),
        }
    }

    fn relative_linear_velocity(
        rigid_body: &RigidBodyData<F>,
        point: Vector<F, 3>,
    ) -> Vector<F, 3> {
        rigid_body.velocity
            + rigid_body
                .angular_velocity
                .cross(point - rigid_body.position)
    }

    pub fn update(&mut self) {
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
                rigid_body.rotation = rigid_body.rotation
                    + angular_velocity_quaternion * rigid_body.rotation * self.time_step;

                rigid_body.rotation = rigid_body.rotation.normalized();
                // println!("ROTATION: {:?}", rigid_body.rotation);
                rigid_body.velocity =
                    rigid_body.velocity + (acceleration + acceleration) * F::HALF * self.time_step;
            } else {
                rigid_body.velocity = Vector::<F, 3>::ZERO;
            }
        }

        let len = self.colliders.len();

        // Number of iterations to converge.
        for _ in 0..10 {
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
                                    let relative_velocity =
                                        rigid_body_a.velocity - rigid_body_b.velocity;
                                    if relative_velocity.length_squared() < F::GJK_EPSILON {
                                        continue;
                                    }
                                    let separation_direction = relative_velocity.normalized();

                                    println!("SEPARATION DIRECTION: {:?}", separation_direction);
                                    println!("POSITION Y HERE: {:?}", rigid_body_a.position);
                                    let (min_a, max_a) = collision::find_min_max_along_direction(
                                        a_to_world
                                            .inversed()
                                            .transform_vector(separation_direction),
                                        &mesh_a.positions,
                                    );
                                    let offset_a = a_to_world
                                        .transform_point(Vector::<F, 3>::ZERO)
                                        .dot(separation_direction);
                                    let (_, max_a) = (min_a + offset_a, max_a + offset_a);

                                    let (min_b, max_b) = collision::find_min_max_along_direction(
                                        b_to_world
                                            .inversed()
                                            .transform_vector(separation_direction),
                                        &mesh_b.positions,
                                    );
                                    let offset_b = b_to_world
                                        .transform_point(Vector::<F, 3>::ZERO)
                                        .dot(separation_direction);
                                    let (min_b, _) = (min_b + offset_b, max_b + offset_b);

                                    let separation = min_b - max_a;
                                    let point_on_separation_plane =
                                        separation_direction * (max_a + separation);

                                    println!("COLLISION!");
                                    println!(
                                        "POINT ON SEPARATION PLANE: {:?}",
                                        point_on_separation_plane
                                    );
                                    println!("SEPARATION: {:?}", separation);

                                    let (collision_points, normals) =
                                        collision::find_planar_contact_points(
                                            point_on_separation_plane,
                                            separation_direction.normalized(),
                                            &mesh_a.positions,
                                            &mesh_a.indices,
                                            &mesh_b.positions,
                                            &mesh_b.indices,
                                            a_to_world,
                                            b_to_world,
                                            a_to_world.inversed(),
                                            b_to_world.inversed(),
                                        );

                                    let mut mass_ratio_a =
                                        rigid_body_a.mass / (rigid_body_a.mass + rigid_body_b.mass);

                                    if mass_ratio_a.is_nan_numeric() {
                                        mass_ratio_a = F::ONE;
                                    }

                                    let mut mass_ratio_b =
                                        rigid_body_b.mass / (rigid_body_b.mass + rigid_body_b.mass);

                                    if mass_ratio_b.is_nan_numeric() {
                                        mass_ratio_b = F::ONE;
                                    }

                                    // Move the rigid bodies apart to the point they began colliding.
                                    if rigid_body_a.mass != F::INFINITY {
                                        rigid_body_a.position +=
                                            separation_direction * separation * mass_ratio_b;
                                    }

                                    if rigid_body_b.mass != F::INFINITY {
                                        rigid_body_b.position +=
                                            separation_direction * separation * mass_ratio_a;
                                    }
                                    for (&point, &_normal) in
                                        collision_points.iter().zip(normals.iter())
                                    {
                                        let normal = Vector::<F, 3>::Y;
                                        println!("normal: {:?}", normal);

                                        let velocity_at_point_a =
                                            Self::relative_linear_velocity(rigid_body_a, point);

                                        let velocity_at_point_b =
                                            Self::relative_linear_velocity(rigid_body_b, point);

                                        // println!("MASS RATIO: {:?}", mass_ratio);
                                        // Cushion the bounce by the bounciness of the colliding objects.
                                        let bounciness =
                                            rigid_body_a.bounciness * rigid_body_b.bounciness;

                                        let relative_velocity_at_collision =
                                            normal.dot(velocity_at_point_a - velocity_at_point_b);

                                        // The following equation is the equation for calculating the magnitude of the impulse.
                                        let numerator =
                                            -(F::ONE + bounciness) * relative_velocity_at_collision;

                                        let term0 = F::ONE / rigid_body_a.mass;
                                        let term1 = F::ONE / rigid_body_b.mass;

                                        // Todo: these are wrong.
                                        let inverse_tensor_a: Matrix<F, 3, 3> =
                                            Matrix::<F, 3, 3>::IDENTITY;
                                        let inverse_tensor_b: Matrix<F, 3, 3> =
                                            Matrix::<F, 3, 3>::IDENTITY;

                                        let ra = point - rigid_body_a.position;
                                        let rb = point - rigid_body_b.position;

                                        let term2 = normal
                                            .dot(inverse_tensor_a * (ra.cross(normal)).cross(ra));
                                        let term3 = normal
                                            .dot(inverse_tensor_b * (rb.cross(normal)).cross(rb));

                                        let magnitude = numerator / (term0 + term1 + term2 + term3);

                                        // This force can then be directly added to the colliding rigid bodies' momentums.
                                        // We store velocity and momentum = mass * velocity.
                                        // So:
                                        // momentum += force
                                        // mass * velocity += force;
                                        // velocity += force / mass;

                                        let force = normal * magnitude;
                                        let relative_velocity =
                                            rigid_body_a.velocity - rigid_body_b.velocity;

                                        let relative_velocity_along_normal =
                                            relative_velocity.dot(normal).numeric_abs();

                                        let mut impulse = normal * relative_velocity_along_normal;

                                        impulse += impulse * bounciness;

                                        if rigid_body_a.mass != F::INFINITY {
                                            rigid_body_a.velocity += force / rigid_body_a.mass;

                                            // rigid_body_a.angular_velocity +=
                                            //     inverse_tensor_a * (ra.cross(force));

                                            println!("VELOCITY A: {:?}", rigid_body_a.velocity);
                                            println!(
                                                "SEPARATING: {:?}",
                                                separation_direction * separation * mass_ratio_b
                                            );
                                            println!("NEW POSITION: {:?}", rigid_body_a.position.y);
                                        }

                                        if rigid_body_b.mass != F::INFINITY {
                                            //let response = impulse * mass_ratio;
                                            // let torque = impulse
                                            //     .cross(point - rigid_body_b.position);
                                            //
                                            // By moving the rigid bodies apart extra energy is introduced
                                            //  into the system.
                                            rigid_body_b.velocity -= force / rigid_body_b.mass;
                                            println!("VELOCITY B: {:?}", rigid_body_b.velocity);

                                            //  rigid_body_b.angular_velocity -=
                                            //      inverse_tensor_b * (rb.cross(force));
                                        }
                                    }
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
        self.collider_meshes.push(MeshData {
            positions: positions.into(),
            normals: normals.into(),
            indices: indices.into(),
        });
        MeshDataHandle(self.collider_meshes.len() - 1)
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
