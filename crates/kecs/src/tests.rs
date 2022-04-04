use crate::*;

#[derive(Clone, Component)]
struct A;

#[derive(Clone, Component)]
struct B;

#[derive(Clone, Component)]
struct C;

#[test]
fn spawn() {
    let mut world = World::new();
    world.spawn(A);
    assert_eq!(world.archetypes.len(), 2);
    world.spawn(B);
    assert_eq!(world.archetypes.len(), 3);
    world.spawn((A, B));
    assert_eq!(world.archetypes.len(), 4);
    world.spawn((B, A));
    assert_eq!(world.archetypes.len(), 4);
}

#[test]
fn despawn() {
    let mut world = World::new();
    let entity_a = world.spawn(A);
    world.spawn(B);
    world.spawn((A, B));
    world.spawn((B, A));

    world.despawn(entity_a).unwrap();

    (|query: Query<&A>| {
        assert_eq!(query.into_iter().count(), 2);
    })
    .run(&world);
}

#[test]
fn single_query() {
    impl ComponentTrait for i32 {}
    let mut world = World::new();
    world.spawn((2,));
    (|i: &i32| assert_eq!(*i, 2)).run(&world);
    (|i: &mut i32| assert_eq!(*i, 2)).run(&world);
}

#[test]
fn multicomponent_singleton_query() {
    let mut world = World::new();
    world.spawn((A, B));
    (|(a, b): (&A, &B)| {}).run(&world);
}

#[test]
fn multi_query() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn((A, B));
    (|_: Query<&A>| {}).run(&world);
}

// This fails for now because queries with just Option filters aren't supported yet.
/*
#[test]
fn multi_option_query() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn((A, B));
    // world.spawn(B);

    (|a: Query<(Option<&A>,)>| {
        assert_eq!(a.into_iter().count(), 2);
    })
    .run(&world)
   ;
}
*/

#[test]
fn multi_option_query1() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn(B);

    (|a: Query<(Option<&A>, &B)>| {
        assert_eq!(a.into_iter().count(), 1);
    })
    .run(&world);
}

#[test]
fn multi_query_iter() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn((A, B));
    (|q: Query<&A>| {
        for _ in &q {}
    })
    .run(&world);

    (|mut q: Query<&mut A>| {
        for _ in &mut q {}
    })
    .run(&world);
}

#[test]
fn system_add() {
    let mut world = World::new();
    world.spawn(A);

    (|_: &A| {}).run(&world);

    let mut x = 0;
    (move |_: &A| {
        x += 1;
        println!("X: {:?}", x);
    })
    .run(&world);
}

#[test]
fn exclusive_system() {
    let mut world = World::new();
    world.spawn(A);

    fn test_exclusive_system(_: &mut World) {}
    let _ = test_exclusive_system.system();
    let _ = (|_: &mut World| {}).system();
}

#[test]
fn get_components() {
    let mut world = World::new();
    let entity_a = world.spawn(A);
    let entity_b = world.spawn(B);

    (move |query: Query<&A>| {
        query.get_entity_components(entity_a).unwrap();
        assert!(query.get_entity_components(entity_b).is_none());
    })
    .run(&world);
}

#[test]
fn query_with_one_component() {
    let mut world = World::new();
    let _ = world.spawn(A);
    (move |query: Query<&A>| {
        assert_eq!(query.into_iter().count(), 1);
    })
    .run(&world);
}

#[test]
fn add_component0() {
    let mut world = World::new();
    let entity = world.spawn(A);
    world.add_component(entity, B).unwrap();

    (move |query: Query<(&A, &B)>| {
        assert_eq!(query.into_iter().count(), 1);
    })
    .run(&world);
}

#[test]
fn add_component1() {
    let mut world = World::new();
    let entity_a = world.spawn(A);
    let _ = world.spawn((A, B));

    world.add_component(entity_a, B).unwrap();

    (move |query: Query<(&A, &B)>| {
        assert_eq!(query.into_iter().count(), 2);
    })
    .run(&world);
}

#[test]
fn remove_component0() {
    let mut world = World::new();
    let entity = world.spawn((A, B));
    world.remove_component::<B>(entity).unwrap();

    (move |query: Query<&A>| {
        assert_eq!(query.into_iter().count(), 1);
    })
    .run(&world);
}

#[test]
fn remove_component1() {
    let mut world = World::new();
    let entity_a = world.spawn((A, B));
    let _ = world.spawn(A);

    world.remove_component::<B>(entity_a).unwrap();

    (move |query: Query<&A>| {
        assert_eq!(query.into_iter().count(), 2);
    })
    .run(&world);
}

#[test]
fn remove_component2() {
    let mut world = World::new();
    let entity = world.spawn(A);
    world.remove_component::<A>(entity).unwrap();
    world.add_component(entity, B).unwrap();

    (move |query: Query<&B>| {
        assert_eq!(query.into_iter().count(), 1);
    })
    .run(&world);
}

#[test]
fn mutable_closure() {
    let mut world = World::new();
    let _ = world.spawn(A);
    let _ = world.spawn(A);

    let mut i = 0;

    {
        (|query: Query<&A>| {
            for _ in &query {
                i += 1;
            }
        })
        .run(&world);
    }
    assert!(i == 2);
}

#[test]
fn system_with_return() {
    let mut world = World::new();
    let _ = world.spawn(A);
    let _ = (|_: Query<&A>| 100).run(&world);
}

#[test]
fn clone_world() {
    let mut world = World::new();
    let _ = world.spawn(A);
    let cloned_world = world.clone_world();
    let _ = (|_: &A| {}).run(&cloned_world);
}

#[test]
fn add_world_to_world0() {
    let mut world = World::new();
    let _ = world.spawn(A);
    let _ = world.spawn(B);

    let mut cloned_world = world.clone_world();
    world.add_world(&mut cloned_world);

    (|query: Query<&A>| {
        assert_eq!(query.into_iter().count(), 2);
    })
    .run(&world);
}

#[test]
fn add_world_to_world1() {
    let mut world = World::new();
    let _ = world.spawn(A);
    let _ = world.spawn(B);

    let mut world_b = World::new();
    world_b.spawn((A, B));

    world.add_world(&mut world_b);

    (|query: Query<&A>| {
        assert_eq!(query.into_iter().count(), 2);
    })
    .run(&world);
}

#[test]
fn no_matching_component() {
    let world = World::new();
    assert_eq!(
        (|_: &A| {}).try_run(&world),
        Err(KecsError::no_matching_component::<A>())
    );
}

#[test]
fn get_component_mut() {
    let mut world = World::new();
    let a = world.spawn(A);
    let _ = world.get_component_mut::<A>(a);
}

#[test]
fn extra_filters() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn((A, B));
    (|query: Query<&A, Without<B>>| {
        assert_eq!(query.into_iter().count(), 1);
    })
    .run(&world);
}

#[test]
fn extra_filters1() {
    // type Q = Query<'static, (&'static A, &'static B), Without<C>>;
    let mut world = World::new();
    world.spawn(A);
    world.spawn((A, B));
    world.spawn((A, B, C));

    (|query: Query<(&A, &B), Without<C>>| {
        assert_eq!(query.into_iter().count(), 1);
    })
    .run(&world);
}

#[test]
fn query0() {
    let mut world = World::new();
    // world.spawn(A);
    world.spawn((A, C));
    (|query: Query<(&A, Option<&B>, Option<&C>)>| {
        let components = query.iter().next().unwrap();
        assert!(components.2.is_some());
        assert_eq!(query.into_iter().count(), 1);
    })
    .run(&world);
}

#[test]
fn query1() {
    let mut world = World::new();
    // world.spawn(A);

    world.spawn(A);
    world.spawn(B);

    (|query: Query<Option<&A>>| assert!(query.iter().next().is_some())).run(&world);
}

#[test]
fn add_duplicate_component() {
    let mut world = World::new();
    // world.spawn(A);

    let entity = world.spawn(A);
    world.add_component(entity, B).unwrap();
    world.add_component(entity, B).unwrap();
}

#[test]
fn disoint_queries() {
    let mut world = World::new();
    // world.spawn(A);

    let entity = world.spawn((A, B));
    world.spawn(A);

    world.add_component(entity, B).unwrap();
    world.add_component(entity, B).unwrap();

    (|query0: Query<&mut A, With<B>>, _: Query<&mut A, Without<B>>| {
        assert!(query0.iter().count() == 1);
    })
    .run(&world);

    assert!(!(|query0: Query<&mut A, With<B>>, _: Query<&A>| {
        assert!(query0.iter().count() == 1);
    })
    .try_run(&world)
    .is_ok());
}

#[test]
fn empty_query() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn(A);

    (|query: Query<()>| {
        assert!(query.iter().count() == 2);
    })
    .run(&world);
}

#[test]
fn optional_query() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn(A);

    (|query: Query<Option<&A>>| {
        for _ in query.iter() {
            println!("HERE");
        }
        assert!(query.iter().count() == 2);
    })
    .run(&world);
}
/*
#[test]
fn componentless_query() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn((A, B));
    (|query: Query<(), With<A>>| {}).run(&world).unwrap();

    println!(
        "{}",
        std::any::type_name::<<Query<(), With<A>> as SystemParameterFetchTrait>::>(),
    );
}
*/

#[test]
fn despawn_and_add_world() {
    let mut world = World::new();
    let entity_a = world.spawn(A);
    let mut world_b = world.clone_world();

    world.despawn(entity_a).unwrap();
    world.add_world(&mut world_b);
}
