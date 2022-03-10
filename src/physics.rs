use crate::*;

type FloatType = f32;

pub fn physics_plguin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_physics.system()],
        fixed_update_systems: vec![
            update_physics_0.system(),
            update_physics_1.system(),
            update_physics_2.system(),
        ],
        ..Default::default()
    }
}

pub fn setup_physics(world: &mut World) {
    world.spawn((
        Name("PhysicsWorld".into()),
        PhysicsWorld {
            world: kphysics::PhysicsWorld::new(),
            paused: false,
        },
    ));
}

pub struct PhysicsWorldHandle(usize);

#[derive(Component, Clone)]
pub struct RigidBody {
    pub mass: FloatType,
    pub bounciness: FloatType,
    pub gravity_multiplier: FloatType,
    pub velocity: Vec3,
    pub rigid_body_handle: Option<kphysics::RigidBodyDataHandle>,
    /// Linear force that is applied during the physics update
    pub linear_force_to_apply: Vec3,
    /// Angular torque force that is applied during the physics update
    pub torque_to_apply: Vec3,
}

impl RigidBody {
    pub fn new(mass: FloatType) -> Self {
        Self {
            mass,
            bounciness: 0.8,
            gravity_multiplier: 1.0,
            velocity: Vec3::ZERO,
            rigid_body_handle: None,
            linear_force_to_apply: Vec3::ZERO,
            torque_to_apply: Vec3::ZERO,
        }
    }

    /// This won't work during the frame the object is initialized as
    /// its corresponding physics object won't be initialized yet.
    pub fn apply_linear_impulse(&mut self, linear_force: Vec3) {
        self.linear_force_to_apply += linear_force;
    }

    /// This won't work during the frame the object is initialized as
    /// its corresponding physics object won't be initialized yet.
    pub fn apply_torque(&mut self, torque: Vec3) {
        self.torque_to_apply += torque;
    }

    /// Position is relative to this `RigidBody`'s center.
    pub fn apply_force_at_position(&mut self, force: Vec3, position: Vec3) {
        let linear_force = force;
        let torque = force.cross(position);

        println!("TORQUE: {:?}", torque);
        self.apply_linear_impulse(linear_force);
        self.apply_torque(-torque);
    }
}

#[derive(Component, Clone)]
pub struct Collider {
    /// The `RigidBody` this `Collider` will have an effect on.
    /// If set to `None` this will default to the `Entity` this is attached to it.
    pub rigid_body_entity: Option<Entity>,
    // /// A handle to the PhysicsWorld this RigidBody is active within.
    // /// This should be the same as the attached RigidBody.
    // pub physics_world_index: PhysicsWorldHandle,
    collider_handle: Option<kphysics::ColliderDataHandle>,
}

impl Collider {
    pub fn new() -> Self {
        Self {
            rigid_body_entity: None,
            collider_handle: None,
        }
    }
}

#[derive(Component, Clone)]
pub struct PhysicsWorld {
    world: kphysics::PhysicsWorld<FloatType>,
    pub paused: bool,
}

impl Deref for PhysicsWorld {
    type Target = kphysics::PhysicsWorld<FloatType>;
    fn deref(&self) -> &Self::Target {
        &self.world
    }
}

impl DerefMut for PhysicsWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world
    }
}

/// These systems are split apart because their borrows overlap.
/// This first system updates the physics simulation's `RigidBody` data.
pub fn update_physics_0(
    mut rigid_bodies: Query<(&mut RigidBody, &mut Transform)>,
    physics_world: &mut PhysicsWorld,
) {
    // Synchronize all `RigidBody` values with the `PhysicsWorld`.
    for (entity, (rigid_body, rigid_body_transform)) in rigid_bodies.entities_and_components_mut() {
        let associated_entity = kphysics::AssociatedEntity {
            index: entity.index(),
            generation: entity.generation(),
        };

        if let Some(rigid_body_handle) = &rigid_body.rigid_body_handle {
            let rigid_body_data = physics_world.get_rigid_body_data_mut(*rigid_body_handle);
            rigid_body_data.mass = rigid_body.mass;
            rigid_body_data.position = rigid_body_transform.position;
            rigid_body_data.rotation = rigid_body_transform.rotation;
            rigid_body_data.velocity = rigid_body.velocity;
            rigid_body_data.associated_entity = associated_entity;
        } else {
            let new_rigid_body_data = kphysics::RigidBodyData {
                mass: rigid_body.mass,
                bounciness: rigid_body.bounciness,
                position: rigid_body_transform.position,
                rotation: rigid_body_transform.rotation,
                velocity: rigid_body.velocity,
                angular_velocity: Vec3::ZERO,
                gravity_multiplier: rigid_body.gravity_multiplier,
                associated_entity,
            };
            rigid_body.rigid_body_handle = Some(physics_world.new_rigid_body(new_rigid_body_data));
        };

        // Apply any accumulated linear forces.
        physics_world.apply_linear_force(
            rigid_body.rigid_body_handle.unwrap(),
            rigid_body.linear_force_to_apply,
        );
        rigid_body.linear_force_to_apply = Vec3::ZERO;

        // Apply any accumulated torque.
        physics_world.apply_torque(
            rigid_body.rigid_body_handle.unwrap(),
            rigid_body.torque_to_apply,
        );
        rigid_body.torque_to_apply = Vec3::ZERO;
    }
}

/// Update the physic simulation's `Collider` data.
pub fn update_physics_1(
    rigid_bodies: Query<&RigidBody>,
    mut colliders: Query<(&mut Collider, &mut Transform, &Handle<Mesh>)>,
    meshes: &Assets<Mesh>,
    physics_world: &mut PhysicsWorld,
) {
    // Synchronize all `Collider` values with the `PhysicsWorld`.
    // Connect `Collider`s to `RigidBody`s.
    for (entity, (collider, collider_transform, mesh_handle)) in
        colliders.entities_and_components_mut()
    {
        let associated_entity = kphysics::AssociatedEntity {
            index: entity.index(),
            generation: entity.generation(),
        };

        // Default to checking self for the `RigidBody` if the `rigid_body_entity` is `None`
        let target_entity = collider.rigid_body_entity.as_ref().unwrap_or(entity);
        let attached_rigid_body = rigid_bodies
            .get_entity_components(*target_entity)
            .map(|c| c.rigid_body_handle.unwrap());

        if let Some(collider_handle) = &collider.collider_handle {
            let collider_data = physics_world.get_collider_data_mut(*collider_handle);
            collider_data.associated_entity = associated_entity;
            collider_data.attached_rigid_body = attached_rigid_body;
        } else {
            let mesh_data = meshes.get(mesh_handle);

            let mesh_collider_handle = physics_world.add_mesh_data(
                &mesh_data.mesh_data.as_ref().unwrap().positions,
                &mesh_data.mesh_data.as_ref().unwrap().normals,
                &mesh_data.mesh_data.as_ref().unwrap().indices,
            );
            let collider_data = kphysics::ColliderData {
                associated_entity,
                attached_rigid_body,
                offset_from_rigid_body: Vec3::ZERO, // this will be updated in a follow-up step.
                mesh_index: mesh_collider_handle,
            };
            collider.collider_handle = Some(physics_world.new_collider(collider_data));
        };

        // The position isn't just set because internally the `PhysicsWorld`
        // needs to calculate the offset from the `Collider` to the `RigidBody`
        physics_world.update_collider_position_and_scale(
            collider.collider_handle.unwrap(),
            collider_transform.position,
            collider_transform.scale,
        );
    }
}

/// Run the physics simulation and update the ECS world with the results.
pub fn update_physics_2(
    mut rigid_bodies: Query<(&mut RigidBody, &mut Transform)>,
    physics_world: &mut PhysicsWorld,
) {
    if !physics_world.paused {
        physics_world.update();

        for (rigid_body, rigid_body_transform) in &mut rigid_bodies {
            let rigid_body_data =
                physics_world.get_rigid_body_data(rigid_body.rigid_body_handle.unwrap());
            rigid_body_transform.position = rigid_body_data.position;
            rigid_body_transform.rotation = rigid_body_data.rotation;
            rigid_body.velocity = rigid_body_data.velocity;
        }
    }
}
