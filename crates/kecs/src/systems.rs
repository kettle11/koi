use crate::*;

pub struct ArchetypeAccess {
    pub archetype_index: usize,
    pub channel_index: usize,
    pub mutable: bool,
}

pub struct SystemParameterMetaData {
    pub archetypes: Vec<usize>,
    pub channels: Vec<Option<(usize, bool)>>,
}

impl SystemParameterMetaData {
    pub fn append_meta_data(&self, archetype_access: &mut Vec<ArchetypeAccess>) {
        if !self.archetypes.is_empty() {
            let channel_count = self.channels.len() / self.archetypes.len();
            for (archetype_index, channels) in self
                .archetypes
                .iter()
                .zip(self.channels.chunks_exact(channel_count))
            {
                for (channel_index, mutable) in channels.iter().flatten() {
                    archetype_access.push(ArchetypeAccess {
                        archetype_index: *archetype_index,
                        channel_index: *channel_index,
                        mutable: *mutable,
                    })
                }
            }
        }
    }
}

pub trait IntoSystemTrait<PARAMETERS> {
    fn system(self) -> System;
}

pub trait RunSystemTrait<'return_lifetime, PARAMETERS, RETURN: 'return_lifetime> {
    fn try_run(self, world: &'return_lifetime World) -> Result<RETURN, KecsError>;
    // fn run(self, world: &'return_lifetime World) -> RETURN;

    #[track_caller]
    fn run(self, world: &'return_lifetime World) -> RETURN
    where
        Self: Sized,
    {
        match <Self as RunSystemTrait<'return_lifetime, PARAMETERS, RETURN>>::try_run(self, world) {
            Ok(r) => r,
            Err(e) => {
                panic!(
                    "System run error: {:?}. \nCould not run system called from: {:?}",
                    e,
                    std::panic::Location::caller()
                )
            }
        }
    }
}

/// [AsSystemArg] introduces an extra layer of indirection that allows things like
/// [RwLockReadGuard]s to exist on the stack while running systems.
pub trait AsSystemArg<'b> {
    type Arg;
    fn as_system_arg(&'b mut self) -> Self::Arg;
}

macro_rules! system_tuple_impls {
    // Don't implement this trait for systems with no parameters.
    // A little incovenient for test cases, but I can't quickly think of a compelling *real*
    // use case for them and they introduce some scheduling / trait conflict problems.
    ( $count: tt, ) => {};
    ( $count: tt, $( ($index: tt, $tuple:ident) ),* ) => {
        impl<'return_lifetime, FUNCTION, RETURN: 'return_lifetime, $( $tuple: SystemParameterTrait ),*> RunSystemTrait<'return_lifetime, ($($tuple,)*), RETURN> for FUNCTION
        where for<'a> &'a mut FUNCTION:
            FnMut( $( $tuple ),*) -> RETURN +
            FnMut( $( <<$tuple as SystemParameterFetchTrait<'return_lifetime>>::FetchResult as AsSystemArg>::Arg ),*) -> RETURN,
        {
            #[allow(non_snake_case, unused_variables, clippy::too_many_arguments)]
            fn try_run(mut self, world: &'return_lifetime World) -> Result<RETURN, KecsError> {
                #[allow(clippy::too_many_arguments)]
                fn call_inner<$($tuple,)* RETURN>(
                    mut f: impl FnMut($($tuple,)*) -> RETURN,
                    $($tuple: $tuple,)*
                ) -> RETURN {
                    f($($tuple,)*)
                }

                $(let $tuple = $tuple::get_meta_data(world)?;)*
                $(let mut $tuple = <$tuple as SystemParameterFetchTrait<'return_lifetime>>::fetch(world, &$tuple)?;)*
                $(let $tuple = $tuple.as_system_arg();)*
                let result = call_inner(&mut self, $( $tuple ),*);
                Ok(result)
            }


        }

        impl<FUNCTION: 'static + Send + Sync, $( $tuple: SystemParameterTrait ),*> IntoSystemTrait<($($tuple,)*)> for FUNCTION
            where for<'a> &'a mut FUNCTION:
                FnMut( $( $tuple ),*) +
                FnMut( $( <<$tuple as SystemParameterFetchTrait>::FetchResult as AsSystemArg>::Arg ),*),
        {
            #[allow(non_snake_case, unused_variables, unused_mut, clippy::too_many_arguments)]
            #[track_caller]
            fn system(mut self) -> System {
                // This trick to get `rustc` to accept calling the FnMut is from Bevy:
                // https://github.com/bevyengine/bevy/blob/f6dbc25bd92ea81b4c7948c6f3f41f6411e97d78/crates/bevy_ecs/src/system/function_system.rs#L432
                fn call_inner<$($tuple,)*>(
                    mut f: impl FnMut($($tuple,)*),
                    $($tuple: $tuple,)*
                ) {
                    f($($tuple,)*)
                }

                System {
                    system_inner: SystemInner::NonExclusive{
                        system: Box::new(
                            move |world: &World| {
                                let mut archetype_access = Vec::new();
                                $(let $tuple = $tuple::get_meta_data(world)?;)*
                                $($tuple.append_meta_data(&mut archetype_access);)*
                                $(let mut $tuple = <$tuple as SystemParameterFetchTrait>::fetch(world, &$tuple)?;)*
                                $(let $tuple = $tuple.as_system_arg();)*
                                call_inner(&mut self, $( $tuple ),*);
                                Ok(())
                        }),
                        meta_data: Box::new( |world: &World| {
                            let mut archetype_access = Vec::new();
                            $(let $tuple = $tuple::get_meta_data(world)?;)*
                            $($tuple.append_meta_data(&mut archetype_access);)*
                            Ok(archetype_access)
                        }),
                    },
                    #[cfg(debug_assertions)]
                    caller_location: std::panic::Location::caller()
                }
            }
        }
    };
}

pub(crate) enum SystemInner {
    Exclusive(Box<dyn FnMut(&mut World) -> Result<(), KecsError> + Send + Sync>),
    NonExclusive {
        system: Box<dyn FnMut(&World) -> Result<(), KecsError> + Send + Sync>,
        meta_data: Box<dyn Fn(&World) -> Result<Vec<ArchetypeAccess>, KecsError> + Send + Sync>,
    },
}
pub struct System {
    pub(crate) system_inner: SystemInner,
    #[cfg(debug_assertions)]
    pub(crate) caller_location: &'static std::panic::Location<'static>,
}

impl System {
    pub fn try_run(&mut self, world: &mut World) -> Result<(), KecsError> {
        match &mut self.system_inner {
            SystemInner::Exclusive(system) => system(world),
            SystemInner::NonExclusive { system, .. } => system(world),
        }
    }

    pub fn run(&mut self, world: &mut World) {
        if let Result::Err(e) = self.try_run(world) {
            #[cfg(debug_assertions)]
            println!("CALLER LOCATION: {:#?}", self.caller_location);
            panic!("{:?}", e);
        }
    }
}

pub trait SystemParameterTrait: for<'a> SystemParameterFetchTrait<'a> {
    fn get_meta_data(world: &World) -> Result<SystemParameterMetaData, KecsError>;
}

pub trait SystemParameterFetchTrait<'a> {
    type FetchResult: for<'b> AsSystemArg<'b>;

    fn fetch(
        world: &'a World,
        meta_data: &SystemParameterMetaData,
    ) -> Result<Self::FetchResult, KecsError>;
}

impl<FUNCTION: FnMut(&mut World) + 'static + Send + Sync> IntoSystemTrait<()> for FUNCTION
where
    for<'a> &'a mut FUNCTION: FnMut(&mut World),
{
    fn system(mut self) -> System {
        System {
            system_inner: SystemInner::Exclusive(Box::new(move |world: &mut World| {
                self(world);
                Ok(())
            })),
            #[cfg(debug_assertions)]
            caller_location: std::panic::Location::caller(),
        }
    }
}

/// Run a number of systems.
/// In the future this could be extended to run systems in parallel, but for now it does not.
pub fn run_systems(world: &mut World, systems: &mut [System]) {
    for system in systems {
        system.run(world);
    }
}
