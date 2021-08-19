use crate::*;

pub trait ComponentBundleTrait: 'static + Send + Sync {
    fn add_to_entity(self, world: &mut World, entity: Entity) -> Result<(), KudoError>;
}

impl<C: ComponentTrait> ComponentBundleTrait for C {
    fn add_to_entity(self, world: &mut World, entity: Entity) -> Result<(), KudoError> {
        (self,).add_to_entity(world, entity)
    }
}

/// Only implemented for Option<Component>
pub(crate) trait AnyComponentTrait: Any {
    fn new_archetype_channel(&self) -> ArchetypeChannel;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<Component: ComponentTrait> AnyComponentTrait for Option<Component> {
    fn new_archetype_channel(&self) -> ArchetypeChannel {
        ArchetypeChannel::new::<Component>()
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub(crate) fn add_components_to_entity_inner(
    world: &mut World,
    entity: Entity,
    components_and_component_ids: &mut [(&mut dyn AnyComponentTrait, ComponentId)],
) -> Result<(), KudoError> {
    let entity_location = world
        .entities
        .get_entity_location(entity)
        .ok_or(KudoError::EntityMissing)?;

    components_and_component_ids.sort_unstable_by_key(|(_, component_id)| *component_id);
    let old_archetype = &world.archetypes[entity_location.archetype_index];
    let new_component_ids = merge_sorted_iter(
        old_archetype.channels.len() + components_and_component_ids.len(),
        old_archetype.channels.iter().map(|c| c.component_id),
        components_and_component_ids.iter().map(|c| c.1),
    );

    let World {
        archetypes,
        components_ids_to_archetype_index,
        entities,
        storage_lookup,
        ..
    } = world;

    // Find or create a new [Archetype]
    let new_archetype_index = {
        let components_and_component_ids = &components_and_component_ids;
        components_ids_to_archetype_index
            .entry(new_component_ids)
            .or_insert_with_key(|key| {
                // Construct a new [Archetype]
                let new_archetype_index = archetypes.len();
                let mut new_archetype = Archetype::new(new_archetype_index);
                let old_archetype = &mut archetypes[entity_location.archetype_index];

                // Create new channels.
                new_archetype.channels.reserve(key.len());
                for channel in &mut old_archetype.channels {
                    let new_channel = channel.new_same_type();
                    new_archetype.channels.push(new_channel)
                }

                // Now insert additional channels for the new components.
                for component in components_and_component_ids.iter() {
                    new_archetype
                        .channels
                        .push(component.0.new_archetype_channel())
                }

                // Sort the [Archetype]'s channels to be in order.
                new_archetype
                    .channels
                    .sort_by_key(|channel| channel.component_id);

                archetypes.push(new_archetype);
                storage_lookup.new_archetype(new_archetype_index, key);
                new_archetype_index
            })
    };

    let (old_archetype, new_archetype) = index_mut_twice(
        archetypes,
        entity_location.archetype_index,
        *new_archetype_index,
    );

    // All existing channels are migrated to the new archetype.
    if *new_archetype_index != entity_location.archetype_index {
        // Migrate data
        World::migrate_entity(
            old_archetype,
            new_archetype,
            entities,
            entity,
            entity_location.index_within_archetype,
        );
    }

    // Add new components to [Archetype]
    let mut component_index = 0;
    for channel in new_archetype.channels.iter_mut() {
        let component_and_component_id = &mut components_and_component_ids[component_index];
        if channel.component_id == component_and_component_id.1 {
            channel.data.push(&mut *component_and_component_id.0);
            component_index += 1;
            if component_index >= components_and_component_ids.len() {
                break;
            }
        }
    }
    Ok(())
}

macro_rules! component_bundle_tuple_impls {
    ( $count: tt, $( ($index: tt, $tuple:ident) ),* ) => {
        impl< $( $tuple: ComponentTrait,)*> ComponentBundleTrait for ($( $tuple,)*)
        {
            #[allow(non_snake_case)]
            fn add_to_entity(self, world: &mut World, entity: Entity) -> Result<(), KudoError> {
                $(let mut $tuple = Some(self.$index);)*
                let mut components_and_component_ids = [$((&mut $tuple as &mut dyn AnyComponentTrait, ComponentId(TypeId::of::<$tuple>())),)*];
                add_components_to_entity_inner(world, entity, &mut components_and_component_ids)
            }
        }
    };
}

pub(crate) fn merge_sorted_iter<T: Ord>(
    capacity: usize,
    mut iter0: impl Iterator<Item = T>,
    mut iter1: impl Iterator<Item = T>,
) -> Vec<T> {
    use std::cmp::Ordering;
    let mut output = Vec::with_capacity(capacity);
    let mut item0 = iter0.next();
    let mut item1 = iter1.next();

    loop {
        output.push(match (item0.is_some(), item1.is_some()) {
            (true, true) => match item0.cmp(&item1) {
                Ordering::Less | Ordering::Equal => {
                    let item = item0.take().unwrap();
                    item0 = iter0.next();
                    item
                }
                Ordering::Greater => {
                    let item = item1.take().unwrap();
                    item1 = iter1.next();
                    item
                }
            },
            (true, false) => {
                let item = item0.take().unwrap();
                item0 = iter0.next();
                item
            }
            (false, true) => {
                let item = item1.take().unwrap();
                item1 = iter1.next();
                item
            }
            (false, false) => {
                break;
            }
        })
    }
    output
}

#[test]
fn merge_sorted() {
    let nums = [0, 1, 5];
    let other_nums = [2, 3, 4, 8];

    let result = merge_sorted_iter(
        nums.len() + other_nums.len(),
        nums.iter(),
        other_nums.iter(),
    );
    println!("RESULT: {:?}", result);
    // assert_eq!(&result, &[0, 1, 2, 3, 4, 5, 8]);
}
