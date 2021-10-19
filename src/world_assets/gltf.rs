use crate::*;
use std::{convert::TryInto, path::Path};

pub(super) fn load_gltf_as_world(
    path: &str,
    gltf: &kgltf::GlTf,
    data: &Option<Vec<u8>>,
    materials: &mut Assets<Material>,
    graphics: &mut Graphics,
    meshes: &mut Assets<Mesh>,
    textures: &mut Assets<Texture>,
    mesh_primitive_data: Vec<MeshPrimitiveData>,
) -> Option<World> {
    let mut gltf_world = World::new();

    let data = data.as_ref().map(|d| &d[..]);

    for extension in &gltf.extensions_required {
        match extension.as_str() {
            "KHR_materials_unlit" => {}
            _ => panic!("Unsupported Gltf extension: {}", extension),
        }
    }

    let scene = gltf.scene.unwrap();
    let scene = &gltf.scenes[scene];

    let mut texture_load_states = vec![
        TextureLoadState {
            linear: None,
            srgb: None,
        };
        gltf.textures.len()
    ];

    let gltf_materials: Vec<_> = gltf
        .materials
        .iter()
        .map(|material| {
            let mut pbr_properties = PBRProperties::default();
            if let Some(pbr_metallic_roughness) = &material.pbr_metallic_roughness {
                let base_color = pbr_metallic_roughness.base_color_factor;

                pbr_properties.base_color =
                    Color::new(base_color[0], base_color[1], base_color[2], base_color[3]);
                pbr_properties.metallic = pbr_metallic_roughness.metallic_factor;
                pbr_properties.roughness = pbr_metallic_roughness.roughness_factor;

                pbr_properties.base_color_texture =
                    pbr_metallic_roughness.base_color_texture.as_ref().map(|t| {
                        get_texture(
                            gltf,
                            &data,
                            path,
                            textures,
                            graphics,
                            &mut texture_load_states,
                            true,
                            t.index,
                        )
                    });

                pbr_properties.metallic_roughness_texture = pbr_metallic_roughness
                    .metallic_roughness_texture
                    .as_ref()
                    .map(|t| {
                        get_texture(
                            gltf,
                            &data,
                            path,
                            textures,
                            graphics,
                            &mut texture_load_states,
                            false,
                            t.index,
                        )
                    });
            }

            pbr_properties.normal_texture = material.normal_texture.as_ref().map(|t| {
                get_texture(
                    gltf,
                    &data,
                    path,
                    textures,
                    graphics,
                    &mut texture_load_states,
                    false,
                    t.index,
                )
            });

            // Is this correct?
            pbr_properties.emissive = Color::new(
                material.emissive_factor[0],
                material.emissive_factor[1],
                material.emissive_factor[2],
                1.0,
            );

            pbr_properties.emissive_texture = material.emissive_texture.as_ref().map(|t| {
                get_texture(
                    gltf,
                    &data,
                    path,
                    textures,
                    graphics,
                    &mut texture_load_states,
                    true,
                    t.index,
                )
            });

            let unlit = material.extensions.contains_key("KHR_materials_unlit");
            let transparent = match material.alpha_mode {
                kgltf::MaterialAlphaMode::Blend => true,
                kgltf::MaterialAlphaMode::Opaque => false,
                kgltf::MaterialAlphaMode::Mask => {
                    klog::log!("NOT YET HANDLED GLTF MASK MATERIAL");
                    true
                }
            };

            let shader = if unlit {
                /*
                if transparent {
                    // Todo: Should be UNLIT_TRANSPARENT
                    Shader::UNLIT
                } else {
                    Shader::UNLIT
                }*/
                Shader::UNLIT
            } else if transparent {
                Shader::PHYSICALLY_BASED_TRANSPARENT
            } else {
                Shader::PHYSICALLY_BASED
            };

            let material = new_pbr_material(shader, pbr_properties);
            materials.add(material)
        })
        .collect();

    let mut mesh_primitives = Vec::with_capacity(mesh_primitive_data.len());

    for mesh_primitive_data in &mesh_primitive_data {
        let mut primitives = Vec::with_capacity(mesh_primitive_data.primitives.len());
        for (mesh_data, material_index) in &mesh_primitive_data.primitives {
            let new_mesh = meshes.add(Mesh::new(graphics, mesh_data.clone()));
            primitives.push((new_mesh, *material_index));
        }
        mesh_primitives.push(primitives);
    }

    for node in &scene.nodes {
        initialize_nodes(
            &mut gltf_world,
            materials,
            &gltf_materials,
            &mesh_primitives,
            &gltf.nodes,
            *node,
            None,
        )
    }

    Some(gltf_world)
}

pub(super) struct MeshPrimitiveData {
    /// The data for this mesh and its material attributes
    // The way this is structured means that multiple things that share attributes will duplicate the attribute data.
    primitives: Vec<(MeshData, Option<usize>)>,
}

pub(super) async fn load_mesh_primitive_data(
    path: &str,
    gltf: &kgltf::GlTf,
    data: Option<&[u8]>,
) -> Vec<MeshPrimitiveData> {
    let mut buffers = Vec::with_capacity(gltf.buffers.len());
    for buffer in &gltf.buffers {
        buffers.push(if let Some(uri) = &buffer.uri {
            let path = Path::new(path).parent().unwrap().join(uri);
            // klog::log!("FETCHING BUFFER!: {:?}", path);
            Some(crate::fetch_bytes(path.to_str().unwrap()).await.unwrap())
        } else {
            None
        })
    }

    let mut meshes = Vec::with_capacity(gltf.meshes.len());
    for mesh in &gltf.meshes {
        let mut primitives = Vec::with_capacity(mesh.primitives.len());

        for primitive in &mesh.primitives {
            let mut positions = None;
            let mut normals = None;
            let mut texture_coordinates = None;
            let mut colors = None;

            for (attribute, accessor_index) in &primitive.attributes {
                // https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#meshes
                let accessor_type = gltf.accessors[*accessor_index].type_.clone();
                let accessor_component_type =
                    gltf.accessors[*accessor_index].component_type.clone();

                match accessor_component_type {
                    kgltf::AccessorComponentType::Float => {}
                    _ => unimplemented!("GLTF loading does not yet handle other accessor types"),
                };

                match attribute.as_str() {
                    "POSITION" => {
                        positions =
                            Some(get_buffer::<Vec3>(gltf, &data, &buffers, *accessor_index).await);
                    }
                    "TEXCOORD_0" => {
                        texture_coordinates =
                            Some(get_buffer::<Vec2>(gltf, &data, &buffers, *accessor_index).await);
                    }
                    "NORMAL" => {
                        normals =
                            Some(get_buffer::<Vec3>(gltf, &data, &buffers, *accessor_index).await);
                    }
                    "COLOR_0" => {
                        // COLOR_0 can be different accessor types according to the spec.
                        // Here we make them always a `Vec4`
                        match accessor_type {
                            kgltf::AccessorType::Vec4 => {
                                colors = Some(
                                    get_buffer::<Vec4>(gltf, &data, &buffers, *accessor_index)
                                        .await,
                                );
                            }
                            kgltf::AccessorType::Vec3 => {
                                let colors_vec3 =
                                    get_buffer::<Vec3>(&gltf, &data, &buffers, *accessor_index)
                                        .await;
                                colors = Some(colors_vec3.iter().map(|v| v.extend(1.0)).collect());
                            }
                            _ => unimplemented!(),
                        }
                    }
                    "TANGENT" => {}
                    "TEXCOORD_1" => {}
                    "JOINTS_0" => {}
                    "WEIGHTS_0" => {}
                    _ => {} // Unimplemented
                }
            }

            let indices = get_indices(gltf, &data, &buffers, primitive.indices.unwrap()).await;

            let mesh_data = MeshData {
                positions: positions.unwrap(),
                normals: normals.unwrap_or_else(Vec::new),
                texture_coordinates: texture_coordinates.unwrap_or_else(|| Vec::new()),
                colors: colors.unwrap_or_else(Vec::new),
                indices,
            };

            primitives.push((mesh_data, primitive.material))
        }
        meshes.push(MeshPrimitiveData { primitives });
    }
    meshes
}

#[derive(Clone)]
struct TextureLoadState {
    linear: Option<Handle<Texture>>,
    srgb: Option<Handle<Texture>>,
}

// This helper function is used to load-textures late.
// Because `koi` must known the color-space of an image when loaded, but it varies depending on how the
// gltf uses the texture.
fn get_texture(
    gltf: &kgltf::GlTf,
    data: &Option<&[u8]>,
    path: &str,
    textures: &mut Assets<Texture>,
    graphics: &mut Graphics,
    texture_load_states: &mut [TextureLoadState],
    srgb: bool,
    texture_index: usize,
) -> Handle<Texture> {
    let image_index = gltf.textures[texture_index].source.unwrap();
    if srgb {
        if let Some(handle) = texture_load_states[texture_index].srgb.clone() {
            return handle;
        }
    } else if let Some(handle) = texture_load_states[texture_index].linear.clone() {
        return handle;
    }

    let image = &gltf.images[image_index];
    let new_handle = if let Some(uri) = &image.uri {
        let path = Path::new(path).parent().unwrap().join(uri);

        textures.load_with_options(
            path.to_str().unwrap(),
            TextureSettings {
                srgb,
                ..Default::default()
            },
        )
    } else {
        // This should probably instead kick of a message to decode the data on another thread
        // using the standard texture loading process.
        let buffer_view = &gltf.buffer_views[image.buffer_view.unwrap()];
        let byte_offset = buffer_view.byte_offset;
        let byte_length = buffer_view.byte_length;
        let bytes = &data.unwrap()[byte_offset..byte_offset + byte_length];
        let image_data = match image.mime_type.as_ref().unwrap() {
            kgltf::ImageMimeType::ImageJpeg => jpeg_data_from_bytes(bytes, srgb),
            kgltf::ImageMimeType::ImagePng => png_data_from_bytes(bytes, srgb),
        };

        textures.add(
            graphics
                .new_texture(
                    Some(&image_data.data),
                    image_data.width,
                    image_data.height,
                    image_data.pixel_format,
                    TextureSettings {
                        srgb,
                        ..Default::default()
                    },
                )
                .unwrap(),
        )
    };

    if srgb {
        texture_load_states[texture_index].srgb = Some(new_handle.clone())
    } else {
        texture_load_states[texture_index].linear = Some(new_handle.clone())
    }
    new_handle
}

fn initialize_nodes(
    gltf_world: &mut World,
    materials: &Assets<Material>,
    gltf_materials: &[Handle<Material>],
    mesh_primitives: &[Vec<(Handle<Mesh>, Option<usize>)>],
    nodes: &[kgltf::Node],
    node: usize,
    parent: Option<Entity>,
) {
    let node = &nodes[node];
    let transform: Transform = if let Some(matrix) = &node.matrix {
        Transform::from_mat4(matrix.try_into().unwrap())
    } else {
        Transform {
            position: node.translation.map_or(Vec3::ZERO, |t| t.into()),
            rotation: node.rotation.map_or(Quat::IDENTITY, |q| q.into()),
            scale: node.scale.map_or(Vec3::ONE, |s| s.into()),
        }
    };

    let entity = if let Some(mesh) = node.mesh {
        let mesh_primitives = &mesh_primitives[mesh];
        if mesh_primitives.len() == 1 {
            let (mesh, material_index) = &mesh_primitives[0];
            let material_handle =
                material_index.map_or_else(Handle::default, |i| gltf_materials[i].clone());
            gltf_world.spawn((
                mesh.clone(),
                material_handle,
                RenderLayers::DEFAULT,
                transform,
            ))
        } else {
            let entity_root = gltf_world.spawn((transform,));
            for (mesh, material_index) in mesh_primitives {
                let material_handle =
                    material_index.map_or_else(Handle::default, |i| gltf_materials[i].clone());
                let primitive_entity = gltf_world.spawn((
                    mesh.clone(),
                    material_handle,
                    RenderLayers::DEFAULT,
                    Transform::new(),
                ));
                HierarchyNode::set_parent(gltf_world, Some(entity_root), primitive_entity).unwrap();
            }
            entity_root
        }
    } else {
        gltf_world.spawn((transform,))
    };

    if let Some(parent) = parent {
        HierarchyNode::set_parent(gltf_world, Some(parent), entity).unwrap();
    }
    for child in &node.children {
        initialize_nodes(
            gltf_world,
            materials,
            gltf_materials,
            mesh_primitives,
            nodes,
            *child,
            Some(entity),
        );
    }
}

async fn get_indices(
    gltf: &kgltf::GlTf,
    data: &Option<&[u8]>,
    buffers: &[Option<Vec<u8>>],
    accessor: usize,
) -> Vec<[u32; 3]> {
    let accessor = &gltf.accessors[accessor];
    let count = accessor.count;

    let buffer_view = accessor.buffer_view.unwrap();
    let buffer_view = &gltf.buffer_views[buffer_view];
    let buffer = &gltf.buffers[buffer_view.buffer];

    let byte_offset = accessor.byte_offset + buffer_view.byte_offset;
    let byte_length = buffer_view.byte_length;

    let bytes = if buffer.uri.is_some() {
        &buffers[buffer_view.buffer].as_ref().unwrap()[byte_offset..byte_offset + byte_length]
    } else {
        &data.as_ref().unwrap()[byte_offset..byte_offset + byte_length]
    };

    unsafe {
        match accessor.component_type {
            kgltf::AccessorComponentType::UnsignedByte => {
                let bytes = &bytes[0..count * std::mem::size_of::<u8>()];
                let (_prefix, shorts, _suffix) = bytes.align_to::<u8>();
                shorts
                    .chunks_exact(3)
                    .map(|u| [u[0] as u32, u[1] as u32, u[2] as u32])
                    .collect()
            }
            kgltf::AccessorComponentType::UnsignedShort => {
                let bytes = &bytes[0..count * std::mem::size_of::<u16>()];
                let (_prefix, shorts, _suffix) = bytes.align_to::<u16>();
                shorts
                    .chunks_exact(3)
                    .map(|u| [u[0] as u32, u[1] as u32, u[2] as u32])
                    .collect()
            }
            kgltf::AccessorComponentType::UnsignedInt => {
                let bytes = &bytes[0..count * std::mem::size_of::<u32>()];
                let (_prefix, shorts, _suffix) = bytes.align_to::<u32>();
                shorts
                    .chunks_exact(3)
                    .map(|u| [u[0] as u32, u[1] as u32, u[2] as u32])
                    .collect()
            }
            _ => unreachable!(), // Should error instead
        }
    }
}

async fn get_buffer<T: Clone>(
    gltf: &kgltf::GlTf,
    data: &Option<&[u8]>,
    buffers: &[Option<Vec<u8>>],
    accessor: usize,
) -> Vec<T> {
    let accessor = &gltf.accessors[accessor];
    let count = accessor.count;

    let buffer_view = accessor.buffer_view.unwrap();
    let buffer_view = &gltf.buffer_views[buffer_view];
    let buffer = &gltf.buffers[buffer_view.buffer];

    let byte_offset = accessor.byte_offset + buffer_view.byte_offset;
    // let byte_length = buffer_view.byte_length;

    /*
    let bytes = if let Some(uri) = &buffer.uri {
        buffers[buffer_view.buffer].as_ref().unwrap()[byte_offset..byte_offset + byte_length];
    } else {
        &data.as_ref().unwrap()[byte_offset..byte_offset + byte_length]
    };
    */

    if buffer.uri.is_some() {
        let bytes = buffers[buffer_view.buffer].as_ref().unwrap();
        unsafe {
            bytes_to_buffer::<T>(
                &bytes[byte_offset..byte_offset + count * std::mem::size_of::<T>()],
            )
        }
    } else {
        // Use the built in data buffer
        unsafe {
            bytes_to_buffer::<T>(
                &data.as_ref().unwrap()
                    [byte_offset..byte_offset + count * std::mem::size_of::<T>()],
            )
        }
    }
}

unsafe fn bytes_to_buffer<T: Clone>(bytes: &[u8]) -> Vec<T> {
    let (_prefix, shorts, _suffix) = bytes.align_to::<T>();
    let result = shorts.into();
    result
}
