use std::{any::Any, any::TypeId, collections::HashMap, sync::RwLockReadGuard};

mod world;
pub use world::*;

mod sparse_set;
mod storage_lookup;

mod chained_iterator;
pub use chained_iterator::*;

mod option_iterator;
pub use option_iterator::*;

#[macro_use]
mod multi_iterator;
pub use multi_iterator::*;

pub(crate) use storage_lookup::*;

#[macro_use]
mod systems;
pub use systems::*;

#[macro_use]
mod component_bundle;
pub use component_bundle::*;

#[macro_use]
mod query;
pub use query::*;

pub use kecs_derive::*;
/*
mod scheduler;
pub use scheduler::*;
*/

mod entities;
pub use entities::*;

pub mod hierarchy;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Hash, Eq, Clone, Copy)]
pub enum KecsError {
    NoMatchingComponent(&'static str),
    EntityMissing,
    ChannelExclusivelyLocked,
}

impl KecsError {
    fn no_matching_component<T: ComponentTrait>() -> Self {
        Self::NoMatchingComponent(std::any::type_name::<T>())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct ComponentId(TypeId);

pub(crate) fn get_component_id<T: 'static + ComponentTrait>() -> ComponentId {
    ComponentId(TypeId::of::<T>())
}

pub trait ComponentTrait: 'static + Send + Sync + Sized {
    fn get_component_id(&self) -> ComponentId {
        ComponentId(TypeId::of::<Self>())
    }

    fn clone_components(
        _entity_migrator: &mut EntityMigrator,
        _items: &[Self],
    ) -> Option<Vec<Self>> {
        None
    }
}

/// A helper to get two mutable borrows from the same slice.
pub(crate) fn index_mut_twice<T>(slice: &mut [T], first: usize, second: usize) -> (&mut T, &mut T) {
    if first < second {
        let (a, b) = slice.split_at_mut(second);
        (&mut a[first], &mut b[0])
    } else {
        let (a, b) = slice.split_at_mut(first);
        (&mut b[0], &mut a[second])
    }
}

macro_rules! tuple_impls {
    ( $count: tt, $( ($index: tt, $tuple:ident) ),*) => {
        system_tuple_impls! { $count, $( ($index, $tuple) ),*}
        component_bundle_tuple_impls! { $count, $( ($index, $tuple) ),*}
        multi_iterator_impl! { $count, $( ($index, $tuple) ),*}
        query_impls! { $count, $( ($index, $tuple) ),*}
        query_iterator_impls! { $count, $( ($index, $tuple) ),*}
    };
}

tuple_impls! {0,}
tuple_impls! { 1, (0, A) }
tuple_impls! { 2, (0, A), (1, B) }
tuple_impls! { 3, (0, A), (1, B), (2, C) }
tuple_impls! { 4, (0, A), (1, B), (2, C), (3, D)}
tuple_impls! { 5, (0, A), (1, B), (2, C), (3, D), (4, E)}
tuple_impls! { 6, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F)}
tuple_impls! { 7, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G)}
tuple_impls! { 8, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H)}
tuple_impls! { 9, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I)}
