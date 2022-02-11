use kecs::hierarchy::*;
use kecs::*;

enum Command {
    DespawnEntity(Entity),
    RunSystem(System),
    SetParent {
        parent: Option<Entity>,
        child: Entity,
    },
}

/// [Commands] is a way to enque edits of the [World] for later.
#[derive(NotCloneComponent)]
pub struct Commands(Vec<Command>);

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}

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

    /// Preserves the child global Transform if it a has a Transform component.
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

    pub fn apply(&mut self, world: &mut World) {
        for command in &mut self.0 {
            match command {
                Command::DespawnEntity(entity) => {
                    HierarchyNode::despawn_hierarchy(world, *entity).unwrap();
                }
                Command::SetParent { parent, child } => crate::set_parent(world, *parent, *child),
                Command::RunSystem(system) => system.run(world),
            }
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

pub fn apply_commands(world: &mut World) {
    let mut commands = Commands::new();
    std::mem::swap(
        &mut commands,
        world.get_single_component_mut::<Commands>().unwrap(),
    );
    commands.apply(world);
    commands.clear();

    std::mem::swap(
        &mut commands,
        world.get_single_component_mut::<Commands>().unwrap(),
    );
}
