use crate::*;

#[derive(Debug, Clone)]
pub struct HierarchyNode {
    pub(crate) parent: Option<Entity>,
    pub(crate) last_child: Option<Entity>,
    pub(crate) next_sibling: Option<Entity>,
    pub(crate) previous_sibling: Option<Entity>,
}

impl HierarchyNode {
    pub fn parent(&self) -> &Option<Entity> {
        &self.parent
    }
    pub fn last_child(&self) -> &Option<Entity> {
        &self.last_child
    }
    pub fn next_sibling(&self) -> &Option<Entity> {
        &self.next_sibling
    }
    pub fn previous_sibling(&self) -> &Option<Entity> {
        &self.previous_sibling
    }

    pub fn clone_hierarchy_node(&self) -> Self {
        Self {
            parent: self.parent,
            last_child: self.last_child,
            next_sibling: self.next_sibling,
            previous_sibling: self.previous_sibling,
        }
    }
}

impl ComponentTrait for HierarchyNode {
    fn clone_components(entity_migrator: &mut EntityMigrator, items: &[Self]) -> Option<Vec<Self>> {
        Some(
            items
                .iter()
                .map(|hierarchy_node| {
                    let parent = hierarchy_node.parent.map(|e| entity_migrator.migrate(e));
                    let last_child = hierarchy_node
                        .last_child
                        .map(|e| entity_migrator.migrate(e));
                    let next_sibling = hierarchy_node
                        .next_sibling
                        .map(|e| entity_migrator.migrate(e));
                    let previous_sibling = hierarchy_node
                        .previous_sibling
                        .map(|e| entity_migrator.migrate(e));

                    Self {
                        parent,
                        last_child,
                        next_sibling,
                        previous_sibling,
                    }
                })
                .collect(),
        )
    }
}

impl HierarchyNode {
    pub fn set_parent(
        world: &mut World,
        parent: Option<Entity>,
        child: Entity,
    ) -> Result<(), KecsError> {
        let mut add_hierarchy_to_parent = false;
        let mut add_hierarchy_to_child = false;

        let previous_last_child = {
            if let Some(parent) = parent {
                // Check if a HierarchyNode exists on the parent, otherwise create one.
                if let Ok(parent_hierarchy) = world.get_component_mut::<HierarchyNode>(parent) {
                    let previous_last_child = parent_hierarchy.last_child;
                    parent_hierarchy.last_child = Some(child);
                    previous_last_child
                } else {
                    add_hierarchy_to_parent = true;
                    None
                }
            } else {
                None
            }
        };

        // Connect the previous child to the new child.
        if let Some(previous_last_child) = previous_last_child {
            let previous_last_child = world
                .get_component_mut::<HierarchyNode>(previous_last_child)
                .unwrap();

            previous_last_child.next_sibling = Some(child);
        }

        if let Ok(child_hierarchy) = world.get_component_mut::<HierarchyNode>(child) {
            // Remove the entity from its old parent, if it has one.
            if let Some(old_parent) = child_hierarchy.parent {
                Self::remove_child(world, old_parent, child)?;
            }
        }

        // Connect the child with its new siblings and parent
        // Create a HierarchyComponent if the child doesn't have one.
        if let Ok(child_hierarchy) = world.get_component_mut::<HierarchyNode>(child) {
            child_hierarchy.parent = parent;
            child_hierarchy.previous_sibling = previous_last_child;
            child_hierarchy.next_sibling = None;
        } else {
            add_hierarchy_to_child = true;
        }

        if add_hierarchy_to_parent {
            world.add_component(
                parent.unwrap(),
                HierarchyNode {
                    parent: None,
                    last_child: Some(child),
                    next_sibling: None,
                    previous_sibling: None,
                },
            )?
        }

        if add_hierarchy_to_child {
            world.add_component(
                child,
                HierarchyNode {
                    parent,
                    previous_sibling: previous_last_child,
                    next_sibling: None,
                    last_child: None,
                },
            )?;
        }

        Ok(())
    }

    /// Removes a child from the parent [Entity].
    /// If the [Child] is not a child of the parent then nothing happens and `remove_child` returns `Ok`.
    pub fn remove_child(world: &mut World, parent: Entity, child: Entity) -> Result<(), KecsError> {
        let (previous, next) = {
            let child = world.get_component_mut::<HierarchyNode>(child)?;

            if child.parent != Some(parent) {
                return Ok(());
            }

            let (previous, next) = (child.previous_sibling, child.next_sibling);
            child.previous_sibling = None;
            child.next_sibling = None;
            child.parent = None;
            (previous, next)
        };

        if let Some(previous) = previous {
            let previous = world.get_component_mut::<HierarchyNode>(previous).unwrap();
            previous.next_sibling = next;
        }

        if let Some(next) = next {
            let next = world.get_component_mut::<HierarchyNode>(next).unwrap();
            next.previous_sibling = previous;
        } else {
            // We're removing the last child, so update the parent.
            let parent = world.get_component_mut::<HierarchyNode>(parent).unwrap();

            parent.last_child = previous;
        }

        Ok(())
    }

    /// Remove an [Entity], all its components, and all of its descendent [Entity]s, from the [World].
    /// A [KecsError] is returned if the entity does not exist.
    pub fn despawn_hierarchy(world: &mut World, entity: Entity) -> Result<(), KecsError> {
        world.despawn(entity).unwrap();
        if let Ok(hierarchy_node) = world
            .get_component_mut::<HierarchyNode>(entity)
            .map(|h| h.clone_hierarchy_node())
        {
            // Despawn all children and their siblings
            let mut current_child = hierarchy_node.last_child;
            while let Some(child) = current_child {
                Self::despawn_hierarchy(world, child).unwrap();
                current_child = world
                    .get_component_mut::<HierarchyNode>(child)
                    .map(|n| n.previous_sibling)
                    .ok()
                    .flatten();
            }
        }

        Ok(())
    }
}
