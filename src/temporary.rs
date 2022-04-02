use crate::*;

/// Despawned during `pre_fixed_update_systems` at the start of the next frame.
#[derive(Component, Clone)]
pub struct Temporary(pub usize);

pub fn temporary_despawn_plugin() -> Plugin {
    Plugin {
        pre_fixed_update_systems: vec![despawn_temporaries.system()],
        ..Default::default()
    }
}

fn despawn_temporaries(commands: &mut Commands, mut temporaries: Query<&mut Temporary>) {
    for (entity, temporary) in temporaries.entities_and_components_mut() {
        if temporary.0 == 0 {
            commands.despawn(*entity)
        } else {
            temporary.0 -= 1;
        }
    }
}
