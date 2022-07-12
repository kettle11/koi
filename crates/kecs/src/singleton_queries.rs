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
            let (channel_index, _) = channel_index.unwrap();
            let channel = world.archetypes[archetype_index].get_read_channel::<T>(channel_index)?;
            if !channel.is_empty() {
                return Ok(channel);
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
            let (channel_index, _) = channel_index.unwrap();
            let channel =
                world.archetypes[archetype_index].get_write_channel::<T>(channel_index)?;
            if !channel.is_empty() {
                return Ok(channel);
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

// Multi-component singleton impls

pub trait SingletonQuery: SystemParameterTrait {
    type Component: ComponentTrait;
    // Lifetimes could be inferred but are allowed for clarity
    #[allow(clippy::needless_lifetimes)]
    fn get_from_archetype<'a>(
        archetype: &'a Archetype,
        channel_index: usize,
    ) -> Result<<Self as SystemParameterFetchTrait<'a>>::FetchResult, KecsError>;
}

impl<A: ComponentTrait> SingletonQuery for &A {
    type Component = A;
    // Lifetimes could be inferred but are allowed for clarity
    #[allow(clippy::needless_lifetimes)]
    fn get_from_archetype<'a>(
        archetype: &'a Archetype,
        channel_index: usize,
    ) -> Result<<Self as SystemParameterFetchTrait<'a>>::FetchResult, KecsError> {
        let channel = archetype.get_read_channel::<A>(channel_index)?;
        if channel.is_empty() {
            Err(KecsError::no_matching_component::<A>())
        } else {
            Ok(channel)
        }
    }
}
impl<A: ComponentTrait> SingletonQuery for &mut A {
    type Component = A;

    // Lifetimes could be inferred but are allowed for clarity
    #[allow(clippy::needless_lifetimes)]
    fn get_from_archetype<'a>(
        archetype: &'a Archetype,
        channel_index: usize,
    ) -> Result<<Self as SystemParameterFetchTrait<'a>>::FetchResult, KecsError> {
        let channel = archetype.get_write_channel::<A>(channel_index)?;
        if channel.is_empty() {
            Err(KecsError::no_matching_component::<A>())
        } else {
            Ok(channel)
        }
    }
}

/*
impl<A: SingletonQuery, B: SingletonQuery> SystemParameterTrait for (A, B) {
    fn get_meta_data(world: &World) -> Result<SystemParameterMetaData, KecsError> {
        let mut archetypes = Vec::new();
        let mut channels = Vec::new();

        for matching_archetype in world.storage_lookup.matching_archetype_iterator::<2>(&[
            (
                Some(0),
                Filter {
                    component_id: get_component_id::<A::Component>(),
                    filter_type: FilterType::With,
                },
            ),
            (
                Some(1),
                Filter {
                    component_id: get_component_id::<B::Component>(),
                    filter_type: FilterType::With,
                },
            ),
        ]) {
            archetypes.push(matching_archetype.archetype_index);
            channels.push(matching_archetype.channels[0].map(|c| (c, false)));
            channels.push(matching_archetype.channels[1].map(|c| (c, false)));
        }

        Ok(SystemParameterMetaData {
            archetypes,
            channels,
        })
    }
}

impl<'a, A: SingletonQuery, B: SingletonQuery> SystemParameterFetchTrait<'a> for (A, B) {
    type FetchResult = (
        <A as SystemParameterFetchTrait<'a>>::FetchResult,
        <B as SystemParameterFetchTrait<'a>>::FetchResult,
    );

    fn fetch(
        world: &'a World,
        meta_data: &SystemParameterMetaData,
    ) -> Result<Self::FetchResult, KecsError> {
        for (&archetype_index, channel_indices) in meta_data
            .archetypes
            .iter()
            .zip(meta_data.channels.chunks_exact(2))
        {
            let (channel_a, _) = channel_indices[0].unwrap();
            let (channel_b, _) = channel_indices[1].unwrap();

            let archetype = &world.archetypes[archetype_index];
            let a = A::get_from_archetype(archetype, channel_a);
            if !a.is_ok() {
                continue;
            }
            let b = B::get_from_archetype(archetype, channel_b);
            if !b.is_ok() {
                continue;
            }
            return Ok((a.unwrap(), b.unwrap()));
        }
        // This is an inaccurate error
        Err(KecsError::EntityMissing)
    }
}

impl<'b, A: AsSystemArg<'b>, B: AsSystemArg<'b>> AsSystemArg<'b> for (A, B) {
    type Arg = (A::Arg, B::Arg);
    fn as_system_arg(&'b mut self) -> Self::Arg {
        (self.0.as_system_arg(), self.1.as_system_arg())
    }
}
*/
macro_rules! singleton_impls {
    ( $count: tt, $( ($index: tt, $tuple:ident) ),* ) => {
        impl<$( $tuple: SingletonQuery,)*> SystemParameterTrait for ($( $tuple,)*)  {
            #[allow(unused)]
            fn get_meta_data(world: &World) -> Result<SystemParameterMetaData, KecsError> {
                let mut archetypes = Vec::new();
                let mut channels = Vec::new();

                for matching_archetype in world.storage_lookup.matching_archetype_iterator::<$count>(&[
                    $((
                        Some($index),
                        Filter {
                            component_id: get_component_id::<$tuple::Component>(),
                            filter_type: FilterType::With,
                        },
                    ),
                    )*
                ]) {
                    archetypes.push(matching_archetype.archetype_index);
                    $(channels.push(matching_archetype.channels[$index].map(|c| (c, false)));)*
                }

                Ok(SystemParameterMetaData {
                    archetypes,
                    channels,
                })
            }
        }

        impl<'a, $( $tuple: SingletonQuery,)*> SystemParameterFetchTrait<'a> for ($( $tuple,)*) {
            type FetchResult = (
                $( <$tuple as SystemParameterFetchTrait<'a>>::FetchResult,)*
            );

            #[allow(unused, non_snake_case)]
            fn fetch(
                world: &'a World,
                meta_data: &SystemParameterMetaData,
            ) -> Result<Self::FetchResult, KecsError> {
                for (&archetype_index, channel_indices) in meta_data
                    .archetypes
                    .iter()
                    .zip(meta_data.channels.chunks_exact($count))
                {
                    let archetype = &world.archetypes[archetype_index];
                    let channels: [(usize, bool); $count] =  [$( channel_indices[$index].unwrap(),)*];
                    $(let $tuple = $tuple::get_from_archetype(archetype, channels[$index].0);
                    if !$tuple.is_ok() {
                        continue;
                    })*

                    return Ok(( $( $tuple.unwrap(),)*));
                }
                // This is an inaccurate error
                Err(KecsError::EntityMissing)
            }
        }

        impl<'b, $( $tuple: AsSystemArg<'b>,)*> AsSystemArg<'b> for ($( $tuple,)*) {
            type Arg =  ($( $tuple::Arg,)*);
            #[allow(clippy::unused_unit)]
            fn as_system_arg(&'b mut self) -> Self::Arg {
                ($( self.$index.as_system_arg(),)*)
            }
        }
    }
}
