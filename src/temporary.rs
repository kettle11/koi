use crate::*;

/// Despawned during `pre_fixed_update_systems` at the start of the next frame.
#[derive(Component, Clone)]
pub struct Temporary;

pub fn temporary_despawn_plugin() -> Plugin {
    Plugin {
        pre_fixed_update_systems: vec![despawn_temporaries.system()],
        ..Default::default()
    }
}

fn despawn_temporaries(commands: &mut Commands, temporaries: Query<&Temporary>) {
    for (entity, _) in temporaries.entities_and_components() {
        commands.despawn(*entity)
    }
}
