use crate::KecsError;
use core::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entity {
    index: u32,
    generation: u32,
}

/// Where the [Entity] is stored in the [World]
#[derive(Debug, Clone, Copy)]
pub struct EntityLocation {
    pub archetype_index: usize,
    pub index_within_archetype: usize,
}
/// Manages the mapping of [Entity]s to [Archetype]s.
#[derive(Debug)]
pub(crate) struct Entities {
    // If the [EntityLocation] is [None] then the [Entity] is despawned.
    generation_and_entity_location: Vec<(u32, Option<EntityLocation>)>,
    pub(crate) free_entities: Vec<Entity>,
    // If this is negative then there are no free_indices left.
    pub(crate) available_free_indices: AtomicI64,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            generation_and_entity_location: Vec::new(),
            free_entities: Vec::new(),
            available_free_indices: AtomicI64::new(0),
        }
    }

    /// Readies this [Entities] for a [World]'s [Entity]'s to be added.
    pub fn reserve_space_for_entity_cloning(&mut self, other: &Entities) {
        self.generation_and_entity_location
            .reserve(other.generation_and_entity_location.len() - other.free_entities.len());

        let entities_to_reserve = other.generation_and_entity_location.len()
            - other.free_entities.len()
            - self.free_entities.len();

        self.generation_and_entity_location
            .extend((0..entities_to_reserve).map(|_| (0, None)));
    }

    pub fn truncate_free_entities_after_cloning(&mut self, other: &Entities) {
        self.free_entities.truncate(
            self.free_entities
                .len()
                .min(other.generation_and_entity_location.len() - other.free_entities.len()),
        );
    }

    /// Creates a new [Entity] and sets its [EntityLocation]
    pub fn new_entity(&mut self, entity_location: Option<EntityLocation>) -> Entity {
        let entity = if let Some(free_entity) = self.free_entities.pop() {
            // Generation does not need to be incremented because it's incremented during `free`.
            self.generation_and_entity_location[free_entity.index as usize] =
                (free_entity.generation, entity_location);
            free_entity
        } else {
            self.generation_and_entity_location
                .push((0, entity_location));
            Entity {
                index: self.generation_and_entity_location.len() as u32 - 1,
                generation: 0,
            }
        };
        entity
    }

    pub fn get_entity_location(&self, entity: Entity) -> Option<EntityLocation> {
        let (generation, location) = &self.generation_and_entity_location[entity.index as usize];
        if *generation != entity.generation {
            None
        } else {
            *location
        }
    }

    /// Gets the [EntityLocation] without checking the [Entity]'s generation.
    pub fn get_entity_location_mut(&mut self, entity: Entity) -> &mut Option<EntityLocation> {
        &mut self.generation_and_entity_location[entity.index as usize].1
    }

    /// Frees an [Entity] to allow its index to be reused.
    pub fn free(&mut self, entity: Entity) -> Result<EntityLocation, KecsError> {
        let (generation, entity_location) =
            &mut self.generation_and_entity_location[entity.index as usize];
        if *generation == entity.generation {
            if let Some(entity_location) = entity_location.take() {
                self.free_entities.push(Entity {
                    index: entity.index,
                    generation: entity.generation + 1,
                });
                *self.available_free_indices.get_mut() += 1;
                return Ok(entity_location);
            }
        }
        Err(KecsError::EntityMissing)
    }

    /// Reserves an [Entity] and its [EntityLocation] will be assigned later.
    /// This is used to create [Entity] handles without exclusive access to the [World].
    pub fn reserve(&self) -> Entity {
        let available_index = self.available_free_indices.fetch_sub(1, Ordering::Relaxed);
        if available_index > 0 {
            self.free_entities[(available_index - 1) as usize]
        } else {
            Entity {
                index: (self.generation_and_entity_location.len() as i64 - available_index) as u32,
                generation: 0,
            }
        }
    }

    /// The [World] calls this repeatedly from [spawn_reserved_entities] to instantiate and get reserved [Entity]s
    pub fn instantiate_reserved_entity(&mut self, index_within_archetype: usize) -> Option<Entity> {
        let available_free_indices = *self.available_free_indices.get_mut();

        if available_free_indices < self.free_entities.len() as i64 {
            if available_free_indices < 0 {
                *self.available_free_indices.get_mut() += 1;
            }
            Some(self.new_entity(Some(EntityLocation {
                archetype_index: 0,
                index_within_archetype,
            })))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.generation_and_entity_location.len()
    }
}

pub struct EntityMigrator<'a> {
    free_entities: &'a [Entity],
    offset: u32,
}
impl<'a> EntityMigrator<'a> {
    pub fn new(free_entities: &'a [Entity], offset: u32) -> Self {
        Self {
            free_entities,
            offset,
        }
    }

    pub fn migrate(&self, old_entity: Entity) -> Entity {
        let new_entity = if old_entity.index < self.free_entities.len() as u32 {
            self.free_entities[self.free_entities.len() - old_entity.index as usize]
        } else {
            Entity {
                generation: 0,
                index: self.offset + (old_entity.index - self.free_entities.len() as u32),
            }
        };
        new_entity
    }
}
