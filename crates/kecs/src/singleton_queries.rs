use crate::*;
use std::sync::RwLockWriteGuard;

// Get an arbitrary instance of T from the [World].
impl<T: ComponentTrait> SystemParameterTrait for &T {
    fn get_meta_data(world: &World) -> Result<SystemParameterMetaData, KecsError> {
        let mut archetypes = Vec::new();
        let mut channels = Vec::new();

        for matching_archetype in world.storage_lookup.matching_archetype_iterator::<1>(&[(
            Some(0),
            Filter {
                component_id: get_component_id::<T>(),
                filter_type: FilterType::With,
            },
        )]) {
            archetypes.push(matching_archetype.archetype_index);
            channels.push(matching_archetype.channels[0].map(|c| (c, false)));
        }

        Ok(SystemParameterMetaData {
            archetypes,
            channels,
        })
    }
}

impl<'a, T: ComponentTrait> SystemParameterFetchTrait<'a> for &T {
    type FetchResult = RwLockReadGuard<'a, Vec<T>>;

    fn fetch(
        world: &'a World,
        meta_data: &SystemParameterMetaData,
    ) -> Result<Self::FetchResult, KecsError> {
        for (&archetype_index, channel_index) in
            meta_data.archetypes.iter().zip(meta_data.channels.iter())
        {
            if let Some((channel_index, _)) = channel_index {
                let channel =
                    world.archetypes[archetype_index].get_read_channel::<T>(*channel_index)?;
                if !channel.is_empty() {
                    return Ok(channel);
                }
            }
        }
        Err(KecsError::no_matching_component::<T>())
    }
}

impl<'b, T: ComponentTrait> AsSystemArg<'b> for RwLockReadGuard<'_, Vec<T>> {
    type Arg = &'b T;
    fn as_system_arg(&'b mut self) -> Self::Arg {
        &self[0]
    }
}

impl<T: ComponentTrait> SystemParameterTrait for &mut T {
    fn get_meta_data(world: &World) -> Result<SystemParameterMetaData, KecsError> {
        let mut archetypes = Vec::new();
        let mut channels = Vec::new();

        for matching_archetype in world.storage_lookup.matching_archetype_iterator::<1>(&[(
            Some(0),
            Filter {
                component_id: get_component_id::<T>(),
                filter_type: FilterType::With,
            },
        )]) {
            archetypes.push(matching_archetype.archetype_index);
            channels.push(matching_archetype.channels[0].map(|c| (c, true)));
        }

        Ok(SystemParameterMetaData {
            archetypes,
            channels,
        })
    }
}

impl<'a, T: ComponentTrait> SystemParameterFetchTrait<'a> for &mut T {
    type FetchResult = RwLockWriteGuard<'a, Vec<T>>;

    fn fetch(
        world: &'a World,
        meta_data: &SystemParameterMetaData,
    ) -> Result<Self::FetchResult, KecsError> {
        for (&archetype_index, channel_index) in
            meta_data.archetypes.iter().zip(meta_data.channels.iter())
        {
            if let Some((channel_index, _)) = channel_index {
                let channel =
                    world.archetypes[archetype_index].get_write_channel::<T>(*channel_index)?;
                if !channel.is_empty() {
                    return Ok(channel);
                }
            }
        }
        Err(KecsError::no_matching_component::<T>())
    }
}

impl<'b, T: 'static> AsSystemArg<'b> for RwLockWriteGuard<'_, Vec<T>> {
    type Arg = &'b mut T;
    fn as_system_arg(&'b mut self) -> Self::Arg {
        &mut self[0]
    }
}
