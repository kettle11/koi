use crate::storage_lookup::*;
use crate::*;
use std::{
    any::Any,
    sync::{RwLock, RwLockWriteGuard},
};

pub(crate) trait ComponentChannelVecTrait: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn new_same_type(&self) -> Box<dyn ComponentChannelVecTrait>;
    fn migrate_component(&mut self, index: usize, other: &mut dyn ComponentChannelVecTrait);
    fn swap_remove(&mut self, index: usize);
    fn assign(&mut self, index: usize, component: &mut dyn AnyComponentTrait);
    fn push(&mut self, component: &mut dyn AnyComponentTrait);
    fn append_channel(&mut self, other: &mut dyn ComponentChannelVecTrait);
    fn clone_channel(
        &mut self,
        entity_migrator: &mut EntityMigrator,
    ) -> Option<Box<dyn ComponentChannelVecTrait>>;
    fn len(&mut self) -> usize;
}

impl<T: ComponentTrait> ComponentChannelVecTrait for RwLock<Vec<T>> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn new_same_type(&self) -> Box<dyn ComponentChannelVecTrait> {
        Box::new(RwLock::new(Vec::<T>::new()))
    }
    fn migrate_component(&mut self, index: usize, other: &mut dyn ComponentChannelVecTrait) {
        let data: T = self.get_mut().unwrap().swap_remove(index);
        other
            .as_any_mut()
            .downcast_mut::<RwLock<Vec<T>>>()
            .unwrap()
            .get_mut()
            .unwrap()
            .push(data)
    }
    fn swap_remove(&mut self, index: usize) {
        self.get_mut().unwrap().swap_remove(index);
    }

    fn assign(&mut self, index: usize, component: &mut dyn AnyComponentTrait) {
        self.get_mut().unwrap()[index] = component
            .as_any_mut()
            .downcast_mut::<Option<T>>()
            .unwrap()
            .take()
            .unwrap();
    }

    fn push(&mut self, component: &mut dyn AnyComponentTrait) {
        self.get_mut().unwrap().push(
            component
                .as_any_mut()
                .downcast_mut::<Option<T>>()
                .unwrap()
                .take()
                .unwrap(),
        )
    }
    fn append_channel(&mut self, other: &mut dyn ComponentChannelVecTrait) {
        let other = other
            .as_any_mut()
            .downcast_mut::<RwLock<Vec<T>>>()
            .unwrap()
            .get_mut()
            .unwrap();
        self.get_mut().unwrap().append(other);
    }

    fn clone_channel(
        &mut self,
        entity_migrator: &mut EntityMigrator,
    ) -> Option<Box<dyn ComponentChannelVecTrait>> {
        Some(Box::new(RwLock::new(T::clone_components(
            entity_migrator,
            self.get_mut().unwrap(),
        )?)))
    }
    fn len(&mut self) -> usize {
        self.get_mut().unwrap().len()
    }
}

pub(crate) struct ArchetypeChannel {
    pub(crate) component_id: ComponentId,
    pub(crate) data: Box<dyn ComponentChannelVecTrait>,
}

impl ArchetypeChannel {
    pub(crate) fn new<Component: ComponentTrait>() -> Self {
        ArchetypeChannel {
            component_id: ComponentId(TypeId::of::<Component>()),
            data: Box::new(RwLock::new(Vec::<Component>::with_capacity(1)))
                as Box<dyn ComponentChannelVecTrait>,
        }
    }

    pub(crate) fn new_same_type(&self) -> Self {
        Self {
            component_id: self.component_id,
            data: self.data.new_same_type(),
        }
    }

    pub(crate) fn clone_channel(&mut self, entity_migrator: &mut EntityMigrator) -> Option<Self> {
        Some(Self {
            component_id: self.component_id,
            data: self.data.clone_channel(entity_migrator)?,
        })
    }

    pub(crate) fn as_mut_vec<T: 'static>(&mut self) -> &mut Vec<T> {
        self.data
            .as_any_mut()
            .downcast_mut::<RwLock<Vec<T>>>()
            .unwrap()
            .get_mut()
            .unwrap()
    }
}

pub struct Archetype {
    pub(crate) entities: Vec<Entity>,
    pub(crate) channels: Vec<ArchetypeChannel>,
    pub(crate) index_in_world: usize,
}

impl Archetype {
    pub(crate) fn new(index_in_world: usize) -> Self {
        Self {
            entities: Vec::new(),
            channels: Vec::new(),
            index_in_world,
        }
    }

    pub(crate) fn get_read_channel<T: 'static>(
        &self,
        channel_index: usize,
    ) -> Result<RwLockReadGuard<Vec<T>>, KecsError> {
        self.channels[channel_index]
            .data
            .as_any()
            .downcast_ref::<RwLock<Vec<T>>>()
            .unwrap()
            .try_read()
            .map_err(|_| KecsError::ChannelExclusivelyLocked)
    }

    pub(crate) fn get_write_channel<T: 'static>(
        &self,
        channel_index: usize,
    ) -> Result<RwLockWriteGuard<Vec<T>>, KecsError> {
        self.channels[channel_index]
            .data
            .as_any()
            .downcast_ref::<RwLock<Vec<T>>>()
            .unwrap()
            .try_write()
            .map_err(|_| KecsError::ChannelExclusivelyLocked)
    }
}

pub struct World {
    pub(crate) archetypes: Vec<Archetype>,
    /// Used to look up [Archetype]s
    pub(crate) components_ids_to_archetype_index: HashMap<Vec<ComponentId>, usize>,
    pub(crate) storage_lookup: StorageLookup,
    pub(crate) entities: Entities,
}

struct RemoveInfo {
    archetype_index: usize,
    archetype_channel: usize,
    entity_index_in_archetype: usize,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            archetypes: Vec::new(),
            components_ids_to_archetype_index: HashMap::new(),
            storage_lookup: StorageLookup::new(),
            entities: Entities::new(),
        };

        // Insert the empty [Archetype]
        world.archetypes.push(Archetype::new(0));
        world.storage_lookup.new_archetype(0, &[]);
        world
            .components_ids_to_archetype_index
            .insert(Vec::new(), 0);
        world
    }

    pub fn is_empty(&self) -> bool {
        self.entities.len() != 0
    }

    /// Returns number of [Entity]s in the [World].
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub(crate) fn spawn_reserved_entities(&mut self) {
        let empty_archetype = &mut self.archetypes[0];
        while let Some(entity) = self
            .entities
            .instantiate_reserved_entity(empty_archetype.entities.len())
        {
            empty_archetype.entities.push(entity)
        }
    }

    pub fn spawn<ComponentBundle: ComponentBundleTrait>(
        &mut self,
        component_bundle: ComponentBundle,
    ) -> Entity {
        self.spawn_reserved_entities();
        // There is definitely some inefficiency here.
        // An [Entity] is added to the empty Archetype *then* transferred to another [Archetype]
        // There could be a fast path to avoid this, but for now this allows more code reuse.
        let entity = self.entities.new_entity(Some(EntityLocation {
            archetype_index: 0,
            index_within_archetype: self.archetypes[0].entities.len(),
        }));
        self.archetypes[0].entities.push(entity);
        // The only error that's possible here is EntityMissing, which should be impossible because we just created it.
        component_bundle.add_to_entity(self, entity).unwrap();
        entity
    }

    /// Remove an [Entity] from the [World].
    /// If the [Entity] is not in the [World] an [EntityMissing] error is returned.
    pub fn despawn(&mut self, entity: Entity) -> Result<(), KecsError> {
        self.spawn_reserved_entities();

        let entity_location = self.entities.free(entity)?;

        // Remove the [Entity]'s components from the [Archetype]
        let archetype = &mut self.archetypes[entity_location.archetype_index];
        for channel in &mut archetype.channels {
            channel
                .data
                .swap_remove(entity_location.index_within_archetype);
        }
        archetype
            .entities
            .swap_remove(entity_location.index_within_archetype);

        // If an [Entity]'s location within the [Archetype] was swapped, update its [EntityLocation].
        if let Some(swapped_entity) = archetype
            .entities
            .get(entity_location.index_within_archetype)
        {
            self.entities
                .get_entity_location_mut(*swapped_entity)
                .as_mut()
                .unwrap()
                .index_within_archetype = entity_location.index_within_archetype;
        }

        Ok(())
    }

    /// Reserves an [Entity] but it will not be in the [World] until
    /// it has components added.
    pub fn reserve_entity(&self) -> Entity {
        self.entities.reserve()
    }

    /// Add a single component to the [Entity].
    /// Replaces components of the same type that are already on the [Entity].
    /// Returns [KecsError::EntityMissing] if the [Entity] does not exist in the [World].
    pub fn add_component<Component: ComponentTrait>(
        &mut self,
        entity: Entity,
        component: Component,
    ) -> Result<(), KecsError> {
        self.spawn_reserved_entities();
        (component,).add_to_entity(self, entity)
    }

    /// Add all of components in the bundle to this [Entity].
    /// Replaces components of the same type that are already on the [Entity].
    /// Returns [KecsError::EntityMissing] if the [Entity] does not exist in the [World].
    pub fn add_components<Components: ComponentBundleTrait>(
        &mut self,
        entity: Entity,
        components: Components,
    ) -> Result<(), KecsError> {
        self.spawn_reserved_entities();
        components.add_to_entity(self, entity)
    }

    /// Remove a single component from this [Entity] and returns it.
    ///
    /// Returns [KecsError::EntityMissing] if the [Entity] does not exist in the [World].
    ///
    /// Returns [KecsError::NoMatchingComponent] if the [Entity] does not have the component.
    pub fn remove_component<Component: ComponentTrait>(
        &mut self,
        entity: Entity,
    ) -> Result<Component, KecsError> {
        let removing_component_id = get_component_id::<Component>();
        let RemoveInfo {
            archetype_index,
            archetype_channel,
            entity_index_in_archetype,
        } = self.remove_component_inner(
            entity,
            removing_component_id,
            std::any::type_name::<Component>(),
        )?;

        // Is this swap-removing the wrong entity?
        let removed_component = self.archetypes[archetype_index].channels[archetype_channel]
            .as_mut_vec()
            .swap_remove(entity_index_in_archetype);
        Ok(removed_component)
    }

    // The inner implementation of "remove".
    // This can prevent a large amount of monomorphized code.
    fn remove_component_inner(
        &mut self,
        entity: Entity,
        removing_component_id: ComponentId,
        component_name: &'static str,
    ) -> Result<RemoveInfo, KecsError> {
        self.spawn_reserved_entities();

        let entity_location = self
            .entities
            .get_entity_location(entity)
            .ok_or(KecsError::EntityMissing)?;

        let old_archetype = &mut self.archetypes[entity_location.archetype_index];
        let mut new_component_ids = Vec::with_capacity(old_archetype.channels.len() - 1);

        let mut removing_channel_index = None;
        for (i, channel) in old_archetype.channels.iter().enumerate() {
            if channel.component_id == removing_component_id {
                removing_channel_index = Some(i);
            } else {
                new_component_ids.push(channel.component_id);
            }
        }
        let removing_channel_index =
            removing_channel_index.ok_or(KecsError::NoMatchingComponent(component_name))?;

        let World {
            archetypes,
            components_ids_to_archetype_index,
            entities,
            storage_lookup,
            ..
        } = self;

        // Find the new [Archetype] to migrate components to.
        let new_archetype_index = components_ids_to_archetype_index
            .entry(new_component_ids)
            .or_insert_with_key(|key| {
                // Construct a new [Archetype]
                let new_archetype_index = archetypes.len();
                let mut new_archetype = Archetype::new(new_archetype_index);

                // Create new channels.
                new_archetype.channels.reserve(key.len());
                let old_archetype = &archetypes[entity_location.archetype_index];
                for (i, channel) in &mut old_archetype.channels.iter().enumerate() {
                    if removing_channel_index != i {
                        let new_channel = channel.new_same_type();
                        new_archetype.channels.push(new_channel)
                    }
                }

                archetypes.push(new_archetype);
                storage_lookup.new_archetype(new_archetype_index, key);
                new_archetype_index
            });

        let (old_archetype, new_archetype) = index_mut_twice(
            archetypes,
            entity_location.archetype_index,
            *new_archetype_index,
        );

        Self::migrate_entity(
            old_archetype,
            new_archetype,
            entities,
            entity,
            entity_location.index_within_archetype,
        );

        Ok(RemoveInfo {
            archetype_index: entity_location.archetype_index,
            archetype_channel: removing_channel_index,
            entity_index_in_archetype: entity_location.index_within_archetype,
        })
    }

    /// Migrates all possible components from the old [Archetype] to the new [Archetype]
    /// In cases where the `destination_archetype` is not a subset of `source_archetype`
    /// the destination will be left incomplete.
    pub(crate) fn migrate_entity(
        source_archetype: &mut Archetype,
        destination_archetype: &mut Archetype,
        entities: &mut Entities,
        entity: Entity,
        index_within_archetype: usize,
    ) {
        if !source_archetype.channels.is_empty() && !destination_archetype.channels.is_empty() {
            let mut source_channel_index = 0;
            let mut destination_channel_index = 0;
            let source_channel_len = source_archetype.channels.len();
            let destination_channel_len = destination_archetype.channels.len();

            while source_channel_index != source_channel_len
                && destination_channel_index != destination_channel_len
            {
                let source_channel = &mut source_archetype.channels[source_channel_index];
                let destination_channel =
                    &mut destination_archetype.channels[destination_channel_index];

                match source_channel
                    .component_id
                    .cmp(&destination_channel.component_id)
                {
                    std::cmp::Ordering::Equal => {
                        source_channel.data.migrate_component(
                            index_within_archetype,
                            &mut *destination_channel.data,
                        );
                        source_channel_index += 1;
                        destination_channel_index += 1;
                    }
                    std::cmp::Ordering::Less => {
                        source_channel_index += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        destination_channel_index += 1;
                    }
                }
            }
        }

        // Update 'entities' for both [Archetype]s
        source_archetype
            .entities
            .swap_remove(index_within_archetype);
        destination_archetype.entities.push(entity);

        // If an [Entity]'s location within the `source_archetype` was swapped, update its location.
        if let Some(swapped_entity) = source_archetype.entities.get(index_within_archetype) {
            entities
                .get_entity_location_mut(*swapped_entity)
                .as_mut()
                .unwrap()
                .index_within_archetype = index_within_archetype;
        }

        // Update the location of this [Entity]
        *entities.get_entity_location_mut(entity) = Some(EntityLocation {
            archetype_index: destination_archetype.index_in_world,
            index_within_archetype: destination_archetype.entities.len() - 1,
        });
    }

    /// This will return an error if the `Entity` does not exist or the `Entity` does not have the component.
    pub fn get_component_mut<Component: ComponentTrait>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut Component, KecsError> {
        let entity_location = self
            .entities
            .get_entity_location(entity)
            .ok_or(KecsError::EntityMissing)?;

        let removing_component_id = get_component_id::<Component>();

        let archetype = &mut self.archetypes[entity_location.archetype_index as usize];
        for channel in &mut archetype.channels {
            if channel.component_id == removing_component_id {
                let component = &mut channel.as_mut_vec()[entity_location.index_within_archetype];
                return Ok(component);
            }
        }
        Err(KecsError::no_matching_component::<Component>())
    }

    /// Gets a single instance of a component from this [World].
    /// If the component does not exist then [KecsError::NoMatchingComponent] this panics.
    /// If multple of the same component exist then an arbitrary one is returned.
    pub fn get_singleton<Component: ComponentTrait>(&mut self) -> &mut Component {
        self.get_single_component_mut().unwrap()
    }

    /// Gets a single instance of a component from this [World].
    /// If the component does not exist then [KecsError::NoMatchingComponent] is returned.
    /// If multple of the same component exist then an arbitrary one is returned.
    pub fn get_single_component_mut<Component: ComponentTrait>(
        &mut self,
    ) -> Result<&mut Component, KecsError> {
        let filters = [(
            Some(0),
            Filter {
                component_id: get_component_id::<Component>(),
                filter_type: FilterType::With,
            },
        )];
        let matching_archetype = self
            .storage_lookup
            .matching_archetype_iterator::<1>(&filters)
            .next()
            .ok_or_else(KecsError::no_matching_component::<Component>)?;
        self.archetypes[matching_archetype.archetype_index].channels
            [matching_archetype.channels[0].unwrap()]
        .as_mut_vec()
        .get_mut(0)
        .ok_or_else(KecsError::no_matching_component::<Component>)
    }

    /// Clones the components and [Entity]s of the other [World] and adds them to this [World].
    pub fn add_world(&mut self, other: &mut World) -> EntityMigrator {
        self.spawn_reserved_entities();
        World::clone_world_into_world(other, self)
    }

    /// Creates a new copy of this [World].
    /// Components that cannot be cloned will be ignored.
    /// This can result in [Entity]s in the cloned [World] with no components.
    pub fn clone_world(&mut self) -> World {
        self.spawn_reserved_entities();
        let mut new_world = World::new();
        World::clone_world_into_world(self, &mut new_world);
        new_world
    }

    /// An internal helper used by [clone_world] and [add_world]
    fn clone_world_into_world(source: &mut World, destination: &mut World) -> EntityMigrator {
        destination.spawn_reserved_entities();

        let World {
            archetypes: old_archetypes,
            entities: old_entities,
            ..
        } = source;

        let migrator_offset = destination.entities.len() as u32;
        destination
            .entities
            .reserve_space_for_entity_cloning(old_entities);

        let entity_migrator = {
            let World {
                archetypes: new_archetypes,
                components_ids_to_archetype_index: new_components_ids_to_archetype_index,
                entities: new_entities,
                storage_lookup: new_storage_lookup,
                ..
            } = destination;

            let mut entity_migrator =
                EntityMigrator::new(&old_entities.free_entities, migrator_offset);

            for old_archetype in old_archetypes {
                let mut new_channels = Vec::new();
                for channel in &mut old_archetype.channels {
                    if let Some(channel) = channel.clone_channel(&mut entity_migrator) {
                        new_channels.push(channel);
                    }
                }

                let new_component_ids: Vec<ComponentId> = new_channels
                    .iter()
                    .map(|channel| channel.component_id)
                    .collect();

                // Search the new [World] for an existing [Archetype] that matches these components.
                // If there's no matching [Archetype] create a new [Archetype].
                // Then append the cloned channels to the new [Archetype].
                // Also append the [Archetype]'s [Entity]s to the new [Archetype]
                // and update their location in [Entities]

                new_components_ids_to_archetype_index
                    .entry(new_component_ids)
                    .and_modify(|new_archetype_index| {
                        let new_archetype_index = *new_archetype_index;
                        // This case can be reached if not all channels are clone.
                        let new_archetype = &mut new_archetypes[new_archetype_index];

                        // Append channel data to this [Archetype]
                        for (desination_channel, source_channel) in new_archetype
                            .channels
                            .iter_mut()
                            .zip(new_channels.iter_mut())
                        {
                            desination_channel
                                .data
                                .append_channel(&mut *source_channel.data)
                        }

                        // Append entities to this [Archetype] and update the [Entity] location
                        new_archetype.entities.reserve(old_archetype.entities.len());
                        for old_entity in &old_archetype.entities {
                            let new_entity = entity_migrator.migrate(*old_entity);
                            *new_entities.get_entity_location_mut(new_entity) =
                                Some(EntityLocation {
                                    index_within_archetype: new_archetype.entities.len(),
                                    archetype_index: new_archetype_index,
                                });
                            new_archetype.entities.push(new_entity);
                        }
                    })
                    .or_insert_with_key(|component_ids| {
                        let new_archetype_index = new_archetypes.len();

                        // Create a new [Archetype]
                        let mut new_archetype = Archetype::new(new_archetype_index);
                        new_archetype.channels.append(&mut new_channels);

                        // Append entities to this [Archetype] and update the [Entity] location
                        new_archetype.entities.reserve(old_archetype.entities.len());
                        for old_entity in &old_archetype.entities {
                            let new_entity = entity_migrator.migrate(*old_entity);
                            *new_entities.get_entity_location_mut(new_entity) =
                                Some(EntityLocation {
                                    index_within_archetype: new_archetype.entities.len(),
                                    archetype_index: new_archetype_index,
                                });
                            new_archetype.entities.push(new_entity);
                        }

                        new_storage_lookup.new_archetype(new_archetype_index, component_ids);
                        new_archetypes.push(new_archetype);
                        new_archetype_index
                    });
            }
            entity_migrator
        };
        destination
            .entities
            .truncate_free_entities_after_cloning(old_entities);
        entity_migrator
    }

    /// Get a [Query] from the [World] without running a system
    pub fn query<'a, PARAMS: QueryParametersTrait>(
        &'a self,
    ) -> Result<<Query<'_, PARAMS> as systems::SystemParameterFetchTrait<'_>>::FetchResult, KecsError>
    where
        Query<'a, PARAMS>: SystemParameterTrait,
    {
        let meta_data = <Query<PARAMS> as SystemParameterTrait>::get_meta_data(self)?;
        <Query<PARAMS> as SystemParameterFetchTrait>::fetch(self, &meta_data)
    }
}
