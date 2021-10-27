use crate::*;
use std::sync::mpsc;

#[cfg(feature = "gltf")]
mod gltf;
#[cfg(feature = "gltf")]
use gltf::*;

pub fn world_assets_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_prefabs.system()],
        pre_fixed_update_systems: vec![load_prefabs_system.system(), delayed_spawn_system.system()],
        ..Default::default()
    }
}

fn setup_prefabs(world: &mut World) {
    let assets = Assets::<World>::new(World::new());
    world.spawn(assets);
}

fn load_prefabs_system(
    #[cfg(feature = "graphics")] graphics: &mut Graphics,
    worlds: &mut Assets<World>,
    #[cfg(feature = "graphics")] materials: &mut Assets<Material>,
    #[cfg(feature = "graphics")] meshes: &mut Assets<Mesh>,
    #[cfg(feature = "graphics")] textures: &mut Assets<Texture>,
) {
    // A Vec doesn't need to be allocated here.
    // This is just a way to not borrow the AssetLoader and Assets at
    // the same time.
    let messages: Vec<_> = worlds.asset_loader.receiver.inner().try_iter().collect();
    for PrefabLoadMessage {
        world_load_message_data,
        handle,
    } in messages.into_iter()
    {
        let world: Option<World> = match world_load_message_data {
            #[cfg(feature = "gltf")]
            PrefabLoadMessageData::GlTf {
                path,
                gltf,
                data,
                mesh_primitive_data,
            } => load_gltf_as_world(
                &path,
                &gltf,
                &data,
                materials,
                graphics,
                meshes,
                textures,
                mesh_primitive_data,
            ),
        };
        // log!("REPLACING WORLD PREFAB");

        worlds.replace_placeholder(&handle, world.unwrap());
        //log!("REPLACED PREFAB!");
    }
}

/// Spawns worlds as they load.
/// Top-level nodes will have their parents set to the [Entity] with the [Handle<World>]
/// This does not yet handle situations where the spawning worlds don't have [HierarchyNode]'s on their top-level components.
fn delayed_spawn_system(world: &mut World) {
    let mut worlds_to_add = Vec::new();

    (|spawn_when_loaded: Query<(&Handle<World>,)>, worlds: &mut Assets<World>| {
        for (entity, world) in spawn_when_loaded.entities_and_components() {
            if !worlds.is_placeholder(world) {
                let new_world = worlds.get_mut(world).clone_world();
                worlds_to_add.push((*entity, new_world));
            }
        }
    })
    .run(world);

    for (parent_entity, mut new_world) in worlds_to_add {
        let top_level_nodes: Vec<Entity> = (|transform_nodes: Query<&HierarchyNode>| {
            transform_nodes
                .entities_and_components()
                .filter_map(|(entity, hierarchy_node)| {
                    if hierarchy_node.parent().is_none() {
                        Some(*entity)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .run(&new_world);

        let entity_migrator = world.add_world(&mut new_world);
        for top_level_entity in top_level_nodes {
            let new_top_level_entity = entity_migrator.migrate(top_level_entity);
            HierarchyNode::set_parent(world, Some(parent_entity), new_top_level_entity).unwrap();
        }
        // Remove the [Handle<World>] to prevent further spawns.
        world
            .remove_component::<Handle<World>>(parent_entity)
            .unwrap();
    }
}

impl LoadableAssetTrait for World {
    type Options = ();
    type AssetLoader = WorldLoader;
}

struct PrefabLoadMessage {
    world_load_message_data: PrefabLoadMessageData,
    handle: Handle<World>,
}

enum PrefabLoadMessageData {
    #[cfg(feature = "gltf")]
    GlTf {
        path: String,
        gltf: kgltf::GlTf,
        data: Option<Vec<u8>>,
        mesh_primitive_data: Vec<MeshPrimitiveData>,
    },
}

pub struct WorldLoader {
    sender: SyncGuard<mpsc::Sender<PrefabLoadMessage>>,
    receiver: SyncGuard<mpsc::Receiver<PrefabLoadMessage>>,
}

impl AssetLoader<World> for WorldLoader {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: SyncGuard::new(sender),
            receiver: SyncGuard::new(receiver),
        }
    }

    fn load_with_options(
        &mut self,
        path: &str,
        handle: crate::Handle<World>,
        _options: <World as LoadableAssetTrait>::Options,
    ) {
        let path = path.to_owned();
        let sender = self.sender.inner().clone();

        let extension = std::path::Path::new(&path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap();
        match extension {
            #[cfg(feature = "gltf")]
            "glb" | "gltf" => {
                ktasks::spawn(async move {
                    let world_load_message_data = load_world(&path).await.unwrap();
                    sender.send(PrefabLoadMessage {
                        handle,
                        world_load_message_data,
                    })
                })
                .run();
            }
            _ => {
                panic!("Unsupported world file format")
            }
        }
    }
}

async fn load_world(path: &str) -> Option<PrefabLoadMessageData> {
    let extension = std::path::Path::new(&path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)?;

    let bytes = crate::fetch_bytes(path).await.ok()?;

    Some(match extension {
        #[cfg(feature = "gltf")]
        "glb" => {
            let glb = kgltf::GLB::from_bytes(&bytes).unwrap();
            let data = glb.binary_data.map(|d| d.into_owned());
            let mesh_primitive_data =
                load_mesh_primitive_data(path, &glb.gltf, data.as_deref()).await;

            PrefabLoadMessageData::GlTf {
                path: path.to_string(),
                gltf: glb.gltf,
                data,
                mesh_primitive_data,
            }
        }
        #[cfg(feature = "gltf")]
        "gltf" => {
            //  klog::log!("ABOUT TO DECODE GLTF");
            use kgltf::FromJson;
            let s = std::str::from_utf8(&bytes).ok()?;
            //   klog::log!("ABOUT TO DECODE GLTF0: {}", s);

            let gltf = kgltf::GlTf::from_json(s).unwrap();
            //  klog::log!("ABOUT TO DECODE GLTF1");

            let mesh_primitive_data = load_mesh_primitive_data(path, &gltf, None).await;

            //   klog::log!("DECODED GLTF, SENDING RETURN MESSAGE");
            PrefabLoadMessageData::GlTf {
                path: path.to_string(),
                gltf,
                data: None,
                mesh_primitive_data,
            }
        }
        _ => {
            klog::log!("Extension not yet implemented: {:?}", extension);
            return None;
        }
    })
}
