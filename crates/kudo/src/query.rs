use std::{iter::Zip, sync::RwLockWriteGuard};

// ------ This section is for [Query]s which can accept multiple parameters.
use crate::*;

pub trait FilterTrait {
    fn append_filters(filters: &mut Vec<(Option<usize>, Filter)>);
}

pub struct With<T: ComponentTrait> {
    phantom: std::marker::PhantomData<fn() -> T>,
}

impl<T: ComponentTrait> FilterTrait for With<T> {
    fn append_filters(filters: &mut Vec<(Option<usize>, Filter)>) {
        filters.push((
            None,
            Filter {
                component_id: get_component_id::<T>(),
                filter_type: FilterType::With,
            },
        ))
    }
}

pub struct Without<T: ComponentTrait> {
    phantom: std::marker::PhantomData<fn() -> T>,
}

impl<T: ComponentTrait> FilterTrait for Without<T> {
    fn append_filters(filters: &mut Vec<(Option<usize>, Filter)>) {
        filters.push((
            None,
            Filter {
                component_id: get_component_id::<T>(),
                filter_type: FilterType::Without,
            },
        ))
    }
}

pub struct Query<'a, PARAMETERS: QueryParametersTrait, FILTERS: FilterTrait = ()> {
    pub(crate) fetch:
        Vec<ArchetypeBorrow<'a, <PARAMETERS as QueryParametersFetchTrait<'a>>::FetchResult>>,
    pub(crate) entities: &'a Entities,
    pub(crate) phantom: std::marker::PhantomData<fn(FILTERS)>,
}

pub(crate) struct ArchetypeBorrow<'a, T> {
    pub(crate) archetype: &'a Archetype,
    pub(crate) borrow: T,
}

impl<'a, PARAMETERS: QueryParametersTrait, FILTERS: FilterTrait> Query<'a, PARAMETERS, FILTERS> {
    pub fn iter<'b>(&'b self) -> <&'b Self as IntoIterator>::IntoIter
    where
        &'b Self: IntoIterator,
    {
        self.into_iter()
    }

    pub fn iter_mut<'b>(&'b mut self) -> <&'b mut Self as IntoIterator>::IntoIter
    where
        &'b mut Self: IntoIterator,
    {
        self.into_iter()
    }

    pub fn get_entity_components<'b>(&'b self, entity: Entity) -> Option<<<<PARAMETERS as QueryParametersFetchTrait<'a>>::FetchResult as GetIteratorsTrait<'b>>::Iterator as Iterator>::Item>{
        let entity_location = self.entities.get_entity_location(entity)?;
        let archetype_borrow_index = self
            .fetch
            .binary_search_by_key(&entity_location.archetype_index, |archetype_borrow| {
                archetype_borrow.archetype.index_in_world
            })
            .ok()?;
        Some(
            self.fetch[archetype_borrow_index]
                .borrow
                .get_components(entity_location.index_within_archetype),
        )
    }

    pub fn get_entity_components_mut<'b>(&'b mut self, entity: Entity) -> Option<<<<PARAMETERS as QueryParametersFetchTrait<'a>>::FetchResult as GetIteratorsTrait<'b>>::IteratorMut as Iterator>::Item>{
        let entity_location = self.entities.get_entity_location(entity)?;
        let archetype_borrow_index = self
            .fetch
            .binary_search_by_key(&entity_location.archetype_index, |archetype_borrow| {
                archetype_borrow.archetype.index_in_world
            })
            .ok()?;
        Some(
            self.fetch[archetype_borrow_index]
                .borrow
                .get_components_mut(entity_location.index_within_archetype),
        )
    }

    /// Produces an [Iterator] that returns the [Entity] and references to its associated components.
    pub fn entities_and_components<'b>(
        &'b self,
    ) -> ChainedIterator<
        Zip<
            std::slice::Iter<'b, Entity>,
            <<PARAMETERS as QueryParametersFetchTrait<'a>>::FetchResult as GetIteratorsTrait<'b>>::Iterator,
        >,
    >{
        ChainedIterator::new(
            self.fetch
                .iter()
                .map(|i| i.archetype.entities.iter().zip(i.borrow.get_iterator()))
                .collect(),
        )
    }

    /// Produces an [Iterator] that returns the [Entity] and mutable references to its associated components.
    pub fn entities_and_components_mut<'b>(
        &'b mut self,
    ) -> ChainedIterator<
        Zip<
            std::slice::Iter<'b, Entity>,
            <<PARAMETERS as QueryParametersFetchTrait<'a>>::FetchResult as GetIteratorsTrait<'b>>::IteratorMut,
        >,
    >{
        ChainedIterator::new(
            self.fetch
                .iter_mut()
                .map(|i| i.archetype.entities.iter().zip(i.borrow.get_iterator_mut()))
                .collect(),
        )
    }
}

pub(crate) fn get_meta_data<const CHANNEL_COUNT: usize>(
    world: &World,
    filters: &[(Option<usize>, Filter)],
    mutable: [bool; CHANNEL_COUNT],
) -> Result<SystemParameterMetaData, KudoError> {
    let mut archetypes = Vec::new();
    let mut channels = Vec::new();

    for matching_archetype in world
        .storage_lookup
        .matching_archetype_iterator::<CHANNEL_COUNT>(filters)
    {
        archetypes.push(matching_archetype.archetype_index);
        for (matching_archetype_channel, mutable) in
            matching_archetype.channels.iter().zip(mutable.iter())
        {
            channels.push(matching_archetype_channel.map(|c| (c, *mutable)));
        }
    }

    Ok(SystemParameterMetaData {
        archetypes,
        channels,
    })
}

// Manual implementations for Query<'_, A> because it's easier to type Query<'a, A> instead of
// Query<'a, (A,)> to get all entities with a single component.
impl<A: QueryParameterTrait, FILTERS: FilterTrait> SystemParameterTrait for Query<'_, A, FILTERS> {
    fn get_meta_data(world: &World) -> Result<SystemParameterMetaData, KudoError> {
        let mut filters = vec![(Some(0), A::filter())];
        FILTERS::append_filters(&mut filters);
        let mutable = [A::mutable()];
        get_meta_data(world, &filters, mutable)
    }
}

impl<'a, A: QueryParameterTrait, FILTERS: FilterTrait> SystemParameterFetchTrait<'a>
    for Query<'_, A, FILTERS>
{
    type FetchResult = Option<Query<'a, A, FILTERS>>;

    fn fetch(
        world: &'a World,
        meta_data: &SystemParameterMetaData,
    ) -> Result<Self::FetchResult, KudoError> {
        let mut fetch = Vec::new();
        for (archetype_index, channel) in meta_data.archetypes.iter().zip(meta_data.channels.iter())
        {
            let archetype = &world.archetypes[*archetype_index];
            fetch.push(ArchetypeBorrow {
                archetype,
                borrow: A::fetch(archetype, channel.map(|c| c.0))?,
            });
        }
        Ok(Some(Query {
            fetch,
            entities: &world.entities,
            phantom: std::marker::PhantomData,
        }))
    }
}

impl<A: QueryParameterTrait> QueryParametersTrait for A {}

impl<'a, A: QueryParameterFetchTrait<'a>> QueryParametersFetchTrait<'a> for A {
    type FetchResult = A::FetchResult;
}

macro_rules! query_impls {
    ( $count: tt, $( ($index: tt, $tuple:ident) ),* ) => {
        impl<$( $tuple: FilterTrait,)*> FilterTrait for ($( $tuple,)*) {
            #[allow(unused)]
            fn append_filters(filters: &mut Vec<(Option<usize>, Filter)>) {
                $(
                    $tuple::append_filters(filters);
                 )*
            }
        }

        #[allow(unused_mut, unused)]
        impl<FILTERS: FilterTrait, $( $tuple: QueryParameterTrait,)*> SystemParameterTrait for Query<'_, ($( $tuple,)*), FILTERS> {
            fn get_meta_data(world: &World) -> Result<SystemParameterMetaData, KudoError> {
                let mut filters = vec![$( (Some($index), $tuple::filter()),)*];
                FILTERS::append_filters(&mut filters);
                let mutable = [$( $tuple::mutable(),)*];
                get_meta_data(world, &filters, mutable)
            }
        }

        #[allow( unused)]
        impl<'a, FILTERS: FilterTrait, $( $tuple: QueryParameterTrait,)*> SystemParameterFetchTrait<'a> for Query<'_, ($( $tuple,)*), FILTERS> {
            type FetchResult = Option<Query<'a, ($( $tuple,)*)>>;

            fn fetch(world: &'a World, meta_data: &SystemParameterMetaData) -> Result<Self::FetchResult, KudoError> {
                let mut fetch = Vec::new();
                for (archetype_index, channels) in meta_data.archetypes.iter().zip( meta_data.channels.chunks_exact($count)) {
                    let archetype = &world.archetypes[*archetype_index];
                    fetch.push(ArchetypeBorrow {
                        archetype,
                        borrow: ($( $tuple::fetch(archetype, channels[$index].map(|c| c.0))?,)*)
                    });
                }
                Ok(Some(Query { fetch, entities: &world.entities, phantom: std::marker::PhantomData }))
            }
        }

        impl<$($tuple: QueryParameterTrait,)*> QueryParametersTrait for ($( $tuple,)*) {}

        impl<'a, $($tuple: QueryParameterFetchTrait<'a>,)*> QueryParametersFetchTrait<'a> for ($( $tuple,)*) {
            type FetchResult = ($( $tuple::FetchResult,)*);
        }
    }
}

macro_rules! query_iterator_impls {
    // These first two cases are implemented manually so skip them in this macro.
    ($count: tt, ($index0: tt, $tuple0:ident)) => {};
    ($count: tt, ($index0: tt, $tuple0:ident), ($index1: tt, $tuple1:ident)) => {};
    ($count: tt, $( ($index: tt, $tuple:ident) ),* ) => {
        #[allow(unused)]
        impl<'a, $( $tuple: GetIteratorsTrait<'a>,)*> GetIteratorsTrait<'a> for ($( $tuple,)*) {
            type Iterator = MultiIterator<($( $tuple::Iterator,)*)>;
            type IteratorMut = MultiIterator<($( $tuple::IteratorMut,)*)>;
            fn get_iterator(&'a self) -> Self::Iterator {
                MultiIterator::<($( $tuple::Iterator,)*)>::new(($( self.$index.get_iterator(),)*))
            }
            fn get_iterator_mut(&'a mut self) -> Self::IteratorMut {
                MultiIterator::<($( $tuple::IteratorMut,)*)>::new(($( self.$index.get_iterator_mut(),)*))
            }
            fn get_components(&'a self, index: usize) -> <Self::Iterator as Iterator>::Item {
                ($( self.$index.get_components(index),)*)
            }
            fn get_components_mut(&'a mut self, index: usize) -> <Self::IteratorMut as Iterator>::Item {
                ($( self.$index.get_components_mut(index),)*)
            }
        }
    };
}

impl<'a, 'b, FILTERS: FilterTrait, PARAMETERS: QueryParametersTrait> IntoIterator
    for &'b Query<'a, PARAMETERS, FILTERS>
{
    type Item = <Self::IntoIter as IntoIterator>::Item;
    type IntoIter =
        ChainedIterator<<<PARAMETERS as QueryParametersFetchTrait<'a>>::FetchResult as GetIteratorsTrait<'b>>::Iterator>;
    fn into_iter(self) -> Self::IntoIter {
        ChainedIterator::new(
            self.fetch
                .iter()
                .map(|archetype_borrow| archetype_borrow.borrow.get_iterator())
                .collect(),
        )
    }
}

impl<'a, 'b, FILTERS: FilterTrait, PARAMETERS: QueryParametersTrait> IntoIterator
    for &'b mut Query<'a, PARAMETERS, FILTERS>
{
    type Item = <Self::IntoIter as IntoIterator>::Item;
    type IntoIter =
        ChainedIterator<<<PARAMETERS as QueryParametersFetchTrait<'a>>::FetchResult as GetIteratorsTrait<'b>>::IteratorMut>;
    fn into_iter(self) -> Self::IntoIter {
        ChainedIterator::new(
            self.fetch
                .iter_mut()
                .map(|archetype_borrow| archetype_borrow.borrow.get_iterator_mut())
                .collect(),
        )
    }
}

impl<'a, A: GetIteratorsTrait<'a>> GetIteratorsTrait<'a> for (A,) {
    type Iterator = A::Iterator;
    type IteratorMut = A::IteratorMut;
    fn get_iterator(&'a self) -> Self::Iterator {
        self.0.get_iterator()
    }
    fn get_iterator_mut(&'a mut self) -> Self::IteratorMut {
        self.0.get_iterator_mut()
    }
    fn get_components(&'a self, index: usize) -> <Self::Iterator as Iterator>::Item {
        self.0.get_components(index)
    }
    fn get_components_mut(&'a mut self, index: usize) -> <Self::IteratorMut as Iterator>::Item {
        self.0.get_components_mut(index)
    }
}

impl<'a, A: GetIteratorsTrait<'a>, B: GetIteratorsTrait<'a>> GetIteratorsTrait<'a> for (A, B) {
    type Iterator = Zip<A::Iterator, B::Iterator>;
    type IteratorMut = Zip<A::IteratorMut, B::IteratorMut>;
    fn get_iterator(&'a self) -> Self::Iterator {
        self.0.get_iterator().zip(self.1.get_iterator())
    }
    fn get_iterator_mut(&'a mut self) -> Self::IteratorMut {
        self.0.get_iterator_mut().zip(self.1.get_iterator_mut())
    }
    fn get_components(&'a self, index: usize) -> <Self::Iterator as Iterator>::Item {
        (self.0.get_components(index), self.1.get_components(index))
    }
    fn get_components_mut(&'a mut self, index: usize) -> <Self::IteratorMut as Iterator>::Item {
        (
            self.0.get_components_mut(index),
            self.1.get_components_mut(index),
        )
    }
}

impl<'a, 'b, PARAMETERS: QueryParametersTrait, FILTERS: FilterTrait> AsSystemArg<'b>
    for Option<Query<'a, PARAMETERS, FILTERS>>
{
    type Arg = Query<'a, PARAMETERS, FILTERS>;
    fn as_system_arg(&'b mut self) -> Self::Arg {
        self.take().unwrap()
    }
}
pub trait QueryParametersTrait: for<'a> QueryParametersFetchTrait<'a> {}
pub trait QueryParametersFetchTrait<'a> {
    type FetchResult: for<'b> GetIteratorsTrait<'b>;
}

trait IntoIteratorTrait {}
pub trait QueryParameterTrait: for<'a> QueryParameterFetchTrait<'a> {
    fn filter() -> Filter;
    fn mutable() -> bool;
}
pub trait QueryParameterFetchTrait<'a> {
    type FetchResult: for<'b> GetIteratorsTrait<'b>;
    fn fetch(
        archetype: &'a Archetype,
        channel_index: Option<usize>,
    ) -> Result<Self::FetchResult, KudoError>;
}

pub trait GetIteratorsTrait<'a> {
    type Iterator: Iterator;
    type IteratorMut: Iterator;
    fn get_iterator(&'a self) -> Self::Iterator;
    fn get_iterator_mut(&'a mut self) -> Self::IteratorMut;
    fn get_components(&'a self, index: usize) -> <Self::Iterator as Iterator>::Item;
    fn get_components_mut(&'a mut self, index: usize) -> <Self::IteratorMut as Iterator>::Item;
}

impl<T: ComponentTrait> QueryParameterTrait for &T {
    fn filter() -> Filter {
        Filter {
            component_id: get_component_id::<T>(),
            filter_type: FilterType::With,
        }
    }
    fn mutable() -> bool {
        false
    }
}

impl<'a, T: ComponentTrait> QueryParameterFetchTrait<'a> for &T {
    type FetchResult = RwLockReadGuard<'a, Vec<T>>;
    fn fetch(
        archetype: &'a Archetype,
        channel_index: Option<usize>,
    ) -> Result<Self::FetchResult, KudoError> {
        archetype.get_read_channel(channel_index.unwrap())
    }
}

impl<'a, T: 'static> GetIteratorsTrait<'a> for RwLockReadGuard<'_, Vec<T>> {
    type Iterator = std::slice::Iter<'a, T>;
    type IteratorMut = std::slice::Iter<'a, T>;
    fn get_iterator(&'a self) -> Self::Iterator {
        self.iter()
    }
    fn get_iterator_mut(&'a mut self) -> Self::IteratorMut {
        self.iter()
    }
    fn get_components(&'a self, index: usize) -> <Self::Iterator as Iterator>::Item {
        &self[index]
    }
    fn get_components_mut(&'a mut self, index: usize) -> <Self::IteratorMut as Iterator>::Item {
        &self[index]
    }
}

impl<T: ComponentTrait> QueryParameterTrait for &mut T {
    fn filter() -> Filter {
        Filter {
            component_id: get_component_id::<T>(),
            filter_type: FilterType::With,
        }
    }
    fn mutable() -> bool {
        true
    }
}

impl<'a, T: ComponentTrait> QueryParameterFetchTrait<'a> for &mut T {
    type FetchResult = RwLockWriteGuard<'a, Vec<T>>;
    fn fetch(
        archetype: &'a Archetype,
        channel_index: Option<usize>,
    ) -> Result<Self::FetchResult, KudoError> {
        archetype.get_write_channel(channel_index.unwrap())
    }
}

impl<'a, T: 'static> GetIteratorsTrait<'a> for RwLockWriteGuard<'_, Vec<T>> {
    type Iterator = std::slice::Iter<'a, T>;
    type IteratorMut = std::slice::IterMut<'a, T>;
    fn get_iterator(&'a self) -> Self::Iterator {
        self.iter()
    }
    fn get_iterator_mut(&'a mut self) -> Self::IteratorMut {
        self.iter_mut()
    }
    fn get_components(&'a self, index: usize) -> <Self::Iterator as Iterator>::Item {
        &self[index]
    }
    fn get_components_mut(&'a mut self, index: usize) -> <Self::IteratorMut as Iterator>::Item {
        &mut self[index]
    }
}

impl<Q: QueryParameterTrait> QueryParameterTrait for Option<Q> {
    fn filter() -> Filter {
        let inner_filter = Q::filter();
        Filter {
            filter_type: FilterType::Optional,
            component_id: inner_filter.component_id,
        }
    }
    fn mutable() -> bool {
        Q::mutable()
    }
}

impl<'a, Q: QueryParameterFetchTrait<'a>> QueryParameterFetchTrait<'a> for Option<Q> {
    type FetchResult = Option<Q::FetchResult>;
    fn fetch(
        archetype: &'a Archetype,
        channel_index: Option<usize>,
    ) -> Result<Self::FetchResult, KudoError> {
        Ok(if let Some(channel_index) = channel_index {
            Some(Q::fetch(archetype, Some(channel_index))?)
        } else {
            None
        })
    }
}

impl<'a, T: GetIteratorsTrait<'a>> GetIteratorsTrait<'a> for Option<T> {
    type Iterator = OptionIterator<T::Iterator>;
    type IteratorMut = OptionIterator<T::IteratorMut>;
    fn get_iterator(&'a self) -> Self::Iterator {
        OptionIterator::new(self.as_ref().map(|s| s.get_iterator()))
    }
    fn get_iterator_mut(&'a mut self) -> Self::IteratorMut {
        OptionIterator::new(self.as_mut().map(|s| s.get_iterator_mut()))
    }
    fn get_components(&'a self, index: usize) -> <Self::Iterator as Iterator>::Item {
        self.as_ref().map(|i| i.get_components(index))
    }
    fn get_components_mut(&'a mut self, index: usize) -> <Self::IteratorMut as Iterator>::Item {
        self.as_mut().map(|i| i.get_components_mut(index))
    }
}
