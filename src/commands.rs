pub use kecs::*;

enum Command {
    DespawnEntity(Entity),
    RunSystem(System),
    SetParent {
        parent: Option<Entity>,
        child: Entity,
    },
}

#[derive(NotCloneComponent)]
pub struct Commands(Vec<Command>);

impl Commands {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.0.push(Command::DespawnEntity(entity));
    }

    pub fn spawn(&mut self, component_bundle: impl ComponentBundleTrait) {
        let mut component_bundle = Some(component_bundle);
        self.0.push(Command::RunSystem(
            (move |world: &mut World| {
                // kecs doesn't support FnOnce systems yet, so use an Option here
                // to make this closure FnMut.
                world.spawn(component_bundle.take().unwrap());
            })
            .system(),
        ))
    }

    pub fn add_world(&mut self, world: World) {
        let mut new_world = Some(world);
        self.0.push(Command::RunSystem(
            (move |world: &mut World| {
                // kecs doesn't support FnOnce systems yet, so use an Option here
                // to make this closure FnMut.
                let mut new_world = new_world.take().unwrap();
                world.add_world(&mut new_world);
            })
            .system(),
        ))
    }

    pub fn set_parent(&mut self, parent: Option<Entity>, child: Entity) {
        self.0.push(Command::SetParent { parent, child });
    }

    pub fn add_component(&mut self, entity: Entity, component: impl ComponentTrait) {
        let mut component = Some(component);
        self.0.push(Command::RunSystem(
            (move |world: &mut World| {
                // kecs doesn't support FnOnce systems yet, so use an Option here
                // to make this closure FnMut.
                let _ = world.add_component(entity, component.take().unwrap());
            })
            .system(),
        ))
    }

    pub fn remove_component<Component: ComponentTrait>(&mut self, entity: Entity) {
        self.0.push(Command::RunSystem(
            (move |world: &mut World| {
                // kecs doesn't support FnOnce systems yet, so use an Option here
                // to make this closure FnMut.
                let _ = world.remove_component::<Component>(entity);
            })
            .system(),
        ))
    }
}

pub fn apply_commands(world: &mut World) {
    let mut commands = Commands::new();
    std::mem::swap(
        &mut commands,
        world.get_single_component_mut::<Commands>().unwrap(),
    );
    for command in commands.0.drain(..) {
        match command {
            Command::DespawnEntity(entity) => {
                HierarchyNode::despawn_hierarchy(world, entity).unwrap();
            }
            Command::SetParent { parent, child } => {
                HierarchyNode::set_parent(world, parent, child).unwrap()
            }
            Command::RunSystem(mut system) => system.run(world),
        }
    }
    std::mem::swap(
        &mut commands,
        &mut world.get_single_component_mut::<Commands>().unwrap(),
    );
}
