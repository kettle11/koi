use std::ops::Mul;

use crate::*;

use kecs::Query;

pub fn transform_plugin() -> Plugin {
    Plugin {
        pre_fixed_update_systems: vec![
            update_root_global_transforms.system(),
            apply_commands.system(),
            update_global_transforms.system(),
            apply_commands.system(),
        ],
        draw_systems: vec![
            update_root_global_transforms.system(),
            apply_commands.system(),
            update_global_transforms.system(),
            apply_commands.system(),
        ],
        ..Default::default()
    }
}

#[derive(Clone, Copy, Debug, Component)]
pub struct GlobalTransform(Transform);

impl Deref for GlobalTransform {
    type Target = Transform;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Component)]
pub struct Transform {
    /// Position relative to parent
    pub position: Vec3,
    /// Rotation relative to parent
    pub rotation: Quat,
    /// Scale relative to parent
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn new_looking_at(origin: Vec3, target: Vec3, up: Vec3) -> Self {
        let transform = Self {
            position: origin,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        };
        transform.look_at(target, up)
    }

    pub fn from_mat4(mat4: Mat4) -> Self {
        let (position, rotation, scale) = mat4.to_translation_rotation_scale();
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn model(&self) -> Mat4 {
        Mat4::from_translation_rotation_scale(self.position, self.rotation, self.scale)
    }

    /// This doesn't correctly respect global vs local transforms.
    #[must_use]
    pub fn look_at(mut self, target: Vec3, up: Vec3) -> Self {
        let rotation = Mat4::look_at(self.position, target, up)
            .inversed()
            .extract_rotation();
        self.rotation = rotation;
        self
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.rotation.rotate_vector3(Vec3::X)
    }

    #[inline]
    pub fn left(&self) -> Vec3 {
        self.rotation.rotate_vector3(-Vec3::X)
    }

    #[inline]
    pub fn up(&self) -> Vec3 {
        self.rotation.rotate_vector3(Vec3::Y)
    }

    #[inline]
    pub fn down(&self) -> Vec3 {
        self.rotation.rotate_vector3(-Vec3::Y)
    }

    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.rotation.rotate_vector3(-Vec3::Z)
    }

    #[inline]
    pub fn back(&self) -> Vec3 {
        self.rotation.rotate_vector3(Vec3::Z)
    }

    pub fn transform_bounding_box(&self, bounding_box: BoundingBox<f32, 3>) -> BoundingBox<f32, 3> {
        let model = self.model();
        let min = model.transform_point(bounding_box.min);
        let max = model.transform_point(bounding_box.max);
        BoundingBox { min, max }
    }

    /*
    /// Set the global position of this [Transform]
    pub fn set_global_position(&mut self, global_transform: &Transform, global_position: Vec3) {
        let to_parent_space = self.model() * self.global_transform.model().inversed();
        let local_position = to_parent_space.transform_point(global_position);
        self.global_transform.position = global_position;
        self.position = local_position;
    }
    */
}

impl Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Self::Output {
        Self::from_mat4(self.model() * rhs.model())
    }
}

/// Add [GlobalTransform]s to all root nodes without them.
fn update_root_global_transforms(
    commands: &mut Commands,
    mut query: Query<(
        &Transform,
        Option<&mut GlobalTransform>,
        Option<&mut HierarchyNode>,
    )>,
) {
    for (entity, (local_transform, global_transform, hierarchy_node)) in
        query.entities_and_components_mut()
    {
        if hierarchy_node.map_or(true, |h| h.parent().is_none()) {
            let new_global_transform = GlobalTransform(*local_transform);
            if let Some(global_transform) = global_transform {
                *global_transform = new_global_transform;
            } else {
                commands.add_component(*entity, new_global_transform)
            }
        }
    }
}

pub fn update_global_transforms(
    commands: &mut Commands,
    mut query: Query<(&HierarchyNode, Option<&Transform>)>,
) {
    // It'd be nice to find a way to avoid this allocation
    let mut parents = Vec::new();

    // This is a bit inefficient in that all hierarchies are updated, regardless of if they changed.
    for (node, local_transform) in &query {
        if let Some(last_child) = node.last_child() {
            if node.parent().is_none() {
                if let Some(local_transform) = local_transform {
                    let global_transform = GlobalTransform(*local_transform);
                    parents.push((global_transform, *last_child));
                } else {
                    parents.push((GlobalTransform(Transform::new()), *last_child));
                }
            }
        }
    }

    for (parent_transform, last_child) in &parents {
        let parent_matrix = parent_transform.model();
        update_descendent_transforms(commands, &mut query, *last_child, &parent_matrix);
    }
}

fn update_descendent_transforms(
    commands: &mut Commands,
    query: &mut Query<(&HierarchyNode, Option<&Transform>)>,
    child_entity: Entity,
    parent_matrix: &Mat4,
) {
    if let Some((hierarchy_node, local_transform)) = query.get_entity_components_mut(child_entity) {
        let last_child = *hierarchy_node.last_child();
        let previous_sibling = *hierarchy_node.previous_sibling();

        let my_model_matrix = if let Some(local_transform) = local_transform {
            local_transform.model()
        } else {
            *parent_matrix
        };
        let my_global_matrix = *parent_matrix * my_model_matrix;

        commands.add_component(
            child_entity,
            GlobalTransform(Transform::from_mat4(my_global_matrix)),
        );

        // Iterate through descendent transforms
        if let Some(child) = last_child {
            update_descendent_transforms(commands, query, child, &my_global_matrix);
        }
        // Iterate through sibling transforms
        if let Some(previous_sibling) = previous_sibling {
            update_descendent_transforms(commands, query, previous_sibling, parent_matrix);
        }
    }
}

/// Parents to the parent and preserves the child's world-space transform.
/// NOTE: This will ignore the child location transform unless it's become part of the global transform.
/// Probably that should be changed in the future.
pub fn set_parent(world: &mut World, parent: Option<Entity>, child: Entity) {
    // This is very inefficient to do here, it updates *ALL* transforms again.
    // It should be removed in favor of only updating the transform for the thing being reparanted.
    // update_global_transforms.run(world);

    HierarchyNode::set_parent(world, parent, child).unwrap();

    let mut parent_mat = Mat4::IDENTITY;
    if let Some(parent) = parent {
        if let Ok(parent_transform) = world.get_component_mut::<GlobalTransform>(parent) {
            parent_mat = parent_transform.model();
        }
    }

    if let Ok(child_transform) = world.get_component_mut::<GlobalTransform>(child) {
        let child_model = child_transform.model();
        let child_relative = child_model.inversed() * parent_mat;

        *child_transform = GlobalTransform(Transform::from_mat4(child_relative));
    }
}
