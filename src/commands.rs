pub use kudo::*;

enum Command {
    DespawnEntity(Entity),
    RunSystem(Box<System>),
}

#[derive(NotCloneComponent)]
pub struct Commands(Vec<Command>);

impl Commands {
    pub fn new() -> Self {
        Self(Vec::new())
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
                HierarchyNode::despawn_hierarchy(world, entity);
            }
            Command::RunSystem(system) => {
                // todo!()
            }
        }
    }
    std::mem::swap(
        &mut commands,
        &mut world.get_single_component_mut::<Commands>().unwrap(),
    );
}
