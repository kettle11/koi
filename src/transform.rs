use std::ops::Mul;

use kudo::{HierarchyNode, Query};

use crate::*;

pub fn transform_plugin() -> Plugin {
    Plugin {
        pre_fixed_update_systems: vec![update_global_transforms.system()],
        draw_systems: vec![update_global_transforms.system()],
        ..Default::default()
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

    // This means that every transform, even if it's a top-level transform, will
    // store extra data.
    // #[skip]
    pub global_transform: GlobalTransform,
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalTransform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl GlobalTransform {
    pub fn model(&self) -> Mat4 {
        Mat4::from_translation_rotation_scale(self.position, self.rotation, self.scale)
    }
    pub fn from_mat4(mat4: Mat4) -> Self {
        let (position, rotation, scale) = mat4.to_translation_rotation_scale();
        Self {
            position,
            rotation,
            scale,
        }
    }
}

impl GlobalTransform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
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
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            global_transform: GlobalTransform::new(),
        }
    }

    pub fn new_with_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            global_transform: GlobalTransform {
                position,
                ..GlobalTransform::new()
            },
        }
    }

    pub fn new_with_rotation(rotation: Quat) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation,
            scale: Vec3::ONE,
            global_transform: GlobalTransform {
                rotation,
                ..GlobalTransform::new()
            },
        }
    }

    pub fn new_with_scale(scale: Vec3) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale,
            global_transform: GlobalTransform {
                scale,
                ..GlobalTransform::new()
            },
        }
    }
    pub fn new_with_scale_rotation(scale: Vec3, rotation: Quat) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation,
            scale,
            global_transform: GlobalTransform {
                scale,
                rotation,
                ..GlobalTransform::new()
            },
        }
    }

    pub fn new_looking_at(origin: Vec3, target: Vec3, up: Vec3) -> Self {
        let mut transform = Self {
            position: origin,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            global_transform: GlobalTransform {
                position: origin,
                ..GlobalTransform::new()
            },
        };
        transform.look_at(target, up);
        transform
    }

    pub fn new_with_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self {
            position,
            rotation,
            scale: Vec3::ONE,
            global_transform: GlobalTransform {
                position,
                rotation,
                ..GlobalTransform::new()
            },
        }
    }

    pub fn new_with_position_scale(position: Vec3, scale: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale,
            global_transform: GlobalTransform {
                position,
                ..GlobalTransform::new()
            },
        }
    }

    pub fn new_with_position_rotation_scale(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
            global_transform: GlobalTransform {
                position,
                rotation,
                scale,
            },
        }
    }

    pub fn from_mat4(mat4: Mat4) -> Self {
        let (position, rotation, scale) = mat4.to_translation_rotation_scale();
        Self {
            position,
            rotation,
            scale,
            global_transform: GlobalTransform::new(),
        }
    }

    pub fn model(&self) -> Mat4 {
        Mat4::from_translation_rotation_scale(self.position, self.rotation, self.scale)
    }

    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        self.rotation = Mat4::look_at(self.position, target, up)
            .inversed()
            .extract_rotation()
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

    /// Set the global position of this [Transform]
    pub fn set_global_position(&mut self, global_position: Vec3) {
        let to_parent_space = self.model() * self.global_transform.model().inversed();
        let local_position = to_parent_space.transform_point(global_position);
        self.global_transform.position = global_position;
        self.position = local_position;
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Self::Output {
        Self::from_mat4(self.model() * rhs.model())
    }
}

pub fn update_global_transforms(mut query: Query<(Option<&HierarchyNode>, &mut Transform)>) {
    // It'd be nice to find a way to avoid this allocation
    let mut parents = Vec::new();

    // This is a bit inefficient in that all hierarchies are updated, regardless of if they changed.

    for (node, transform) in &mut query {
        transform.global_transform = GlobalTransform {
            position: transform.position,
            rotation: transform.rotation,
            scale: transform.scale,
        };

        if let Some(node) = node {
            if let Some(last_child) = node.last_child() {
                if node.parent().is_none() {
                    let parent_transform = &transform.global_transform;
                    parents.push((parent_transform.clone(), *last_child));
                    continue;
                }
            }
        }
    }

    for (parent_transform, last_child) in &parents {
        let parent_matrix = parent_transform.model();
        update_descendent_transforms(&mut query, *last_child, &parent_matrix);
    }
}

fn update_descendent_transforms(
    query: &mut Query<(Option<&HierarchyNode>, &mut Transform)>,
    child_entity: Entity,
    parent_matrix: &Mat4,
) {
    if let Some((hierarchy_node, transform)) = query.get_entity_components_mut(child_entity) {
        if let Some(hierarchy_node) = hierarchy_node {
            let last_child = hierarchy_node.last_child().clone();
            let previous_sibling = hierarchy_node.previous_sibling().clone();

            let my_model_matrix = transform.model();
            let my_global_matrix = *parent_matrix * my_model_matrix;
            transform.global_transform = GlobalTransform::from_mat4(my_global_matrix);

            // Iterate through descendent transforms
            if let Some(child) = last_child {
                update_descendent_transforms(query, child, &my_global_matrix);
            }
            // Iterate through sibling transforms
            if let Some(previous_sibling) = previous_sibling {
                update_descendent_transforms(query, previous_sibling, parent_matrix);
            }
        }
    }
}
