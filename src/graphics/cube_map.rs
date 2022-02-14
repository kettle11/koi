use crate::*;
use kgraphics::*;
use std::sync::mpsc;

pub use kgraphics::CubeMap;

enum ShaderTextureProperty {
    TextureProperty(TextureProperty),
    CubeMapProperty(CubeMapProperty),
}

struct ShaderAndProperties {
    shader: Shader,
    projection_property: Mat4Property,
    view_property: Mat4Property,
    texture_property: ShaderTextureProperty,
    position_attribute: VertexAttribute<Vec3>,
}

struct SpecularIrradianceProperties {
    shader: Shader,
    projection_property: Mat4Property,
    view_property: Mat4Property,
    p_texture: CubeMapProperty,
    p_roughness: FloatProperty,
    a_position: VertexAttribute<Vec3>,
}

fn get_shader_and_properties(
    graphics: &mut Graphics,
    source: &str,
    irradiance: bool,
) -> ShaderAndProperties {
    let shader = graphics
        .new_shader(
            source,
            PipelineSettings {
                depth_test: DepthTest::LessOrEqual,
                ..PipelineSettings::default()
            },
        )
        .unwrap();
    let projection_property = shader
        .pipeline
        .get_mat4_property("p_projections[0]")
        .unwrap();
    let view_property = shader.pipeline.get_mat4_property("p_views[0]").unwrap();
    let texture_property = if !irradiance {
        ShaderTextureProperty::TextureProperty(
            shader.pipeline.get_texture_property("p_texture").unwrap(),
        )
    } else {
        ShaderTextureProperty::CubeMapProperty(
            shader.pipeline.get_cube_map_property("p_texture").unwrap(),
        )
    };

    let position_attribute = shader.pipeline.get_vertex_attribute("a_position").unwrap();

    ShaderAndProperties {
        shader,
        projection_property,
        view_property,
        texture_property,
        position_attribute,
    }
}

pub struct CubeMapRenderer {
    equirectangular_to_cubemap_shader: ShaderAndProperties,
    diffuse_irradiance_convolution_shader: ShaderAndProperties,
    specular_irradiance_shader: SpecularIrradianceProperties,
}

impl CubeMapRenderer {
    pub fn new(graphics: &mut Graphics) -> CubeMapRenderer {
        let equirectangular_to_cubemap_shader = get_shader_and_properties(
            graphics,
            include_str!("built_in_shaders/equirectangular_to_cubemap.glsl"),
            false,
        );
        let diffuse_irradiance_convolution_shader = get_shader_and_properties(
            graphics,
            include_str!("built_in_shaders/irradiance_convolution.glsl"),
            true,
        );
        let specular_irradiance_convolution_shader = graphics
            .new_shader(
                include_str!("built_in_shaders/specular_irradiance_convolution.glsl"),
                PipelineSettings {
                    depth_test: DepthTest::LessOrEqual,
                    ..PipelineSettings::default()
                },
            )
            .unwrap();
        let specular_irradiance_shader = SpecularIrradianceProperties {
            projection_property: specular_irradiance_convolution_shader
                .pipeline
                .get_mat4_property("p_projections[0]")
                .unwrap(),
            view_property: specular_irradiance_convolution_shader
                .pipeline
                .get_mat4_property("p_views[0]")
                .unwrap(),
            p_roughness: specular_irradiance_convolution_shader
                .pipeline
                .get_float_property("p_roughness")
                .unwrap(),
            p_texture: specular_irradiance_convolution_shader
                .pipeline
                .get_cube_map_property("p_texture")
                .unwrap(),
            a_position: specular_irradiance_convolution_shader
                .pipeline
                .get_vertex_attribute("a_position")
                .unwrap(),
            shader: specular_irradiance_convolution_shader,
        };
        CubeMapRenderer {
            equirectangular_to_cubemap_shader,
            diffuse_irradiance_convolution_shader,
            specular_irradiance_shader,
        }
    }
}

enum TextureIn<'a> {
    Texture(&'a kgraphics::Texture),
    CubeMap(&'a CubeMap),
}
fn render_cube_map(
    graphics: &mut Graphics,
    meshes: &Assets<Mesh>,
    shader: &ShaderAndProperties,
    texture: TextureIn,
    cube_map: &CubeMap,
    size: usize,
) {
    let cube_mesh = meshes.get(&Mesh::CUBE_MAP_CUBE).gpu_mesh.as_ref().unwrap();

    let projection: Mat4 =
        kmath::projection_matrices::perspective_gl(90.0_f32.to_radians(), 1.0, 0.1, 10.);

    // I assume the -Y here is to flip the image as well.
    let views = [
        Mat4::looking_at(Vec3::ZERO, Vec3::X, -Vec3::Y),
        Mat4::looking_at(Vec3::ZERO, -Vec3::X, -Vec3::Y),
        Mat4::looking_at(Vec3::ZERO, Vec3::Y, Vec3::Z),
        Mat4::looking_at(Vec3::ZERO, -Vec3::Y, -Vec3::Z),
        Mat4::looking_at(Vec3::ZERO, Vec3::Z, -Vec3::Y),
        Mat4::looking_at(Vec3::ZERO, -Vec3::Z, -Vec3::Y),
    ];

    for i in 0..6 {
        let mut command_buffer = graphics.context.new_command_buffer();

        let face_texture = cube_map.get_face_texture(i);
        let framebuffer = graphics
            .context
            .new_framebuffer(Some(&face_texture), None, None);
        {
            let mut render_pass = command_buffer
                .begin_render_pass_with_framebuffer(&framebuffer, Some((1.0, 0.0, 0.0, 1.0)));
            render_pass.set_viewport(0, 0, size as u32, size as u32);

            render_pass.set_pipeline(&shader.shader.pipeline);
            render_pass.set_mat4_property(&shader.projection_property, projection.as_array());
            render_pass.set_mat4_property(&shader.view_property, views[i].as_array());
            match &shader.texture_property {
                ShaderTextureProperty::TextureProperty(p) => {
                    let t = match texture {
                        TextureIn::Texture(t) => t,
                        _ => unreachable!(),
                    };
                    render_pass.set_texture_property(p, Some(t), 0);
                }
                ShaderTextureProperty::CubeMapProperty(p) => {
                    let t = match texture {
                        TextureIn::CubeMap(t) => t,
                        _ => unreachable!(),
                    };
                    render_pass.set_cube_map_property(p, Some(t), 0);
                }
            }

            render_pass
                .set_vertex_attribute(&shader.position_attribute, Some(&cube_mesh.positions));

            // Render mesh here.
            render_pass.draw_triangles(cube_mesh.triangle_count, &cube_mesh.index_buffer);
        }
        graphics.context.commit_command_buffer(command_buffer);
        graphics.context.delete_framebuffer(framebuffer);

        // Probably should generate mipmaps here to reduce artifacts
    }
}

fn render_specular_irradiance_cube_map(
    graphics: &mut Graphics,
    meshes: &Assets<Mesh>,
    shader: &SpecularIrradianceProperties,
    environment_texture: &CubeMap,
    cube_map: &CubeMap,
) {
    let size = 128.0;

    let cube_mesh = meshes.get(&Mesh::CUBE_MAP_CUBE).gpu_mesh.as_ref().unwrap();

    let projection: Mat4 =
        kmath::projection_matrices::perspective_gl(90.0_f32.to_radians(), 1.0, 0.1, 10.);

    // I assume the -Y here is to flip the image as well.
    let views = [
        Mat4::looking_at(Vec3::ZERO, Vec3::X, -Vec3::Y),
        Mat4::looking_at(Vec3::ZERO, -Vec3::X, -Vec3::Y),
        Mat4::looking_at(Vec3::ZERO, Vec3::Y, Vec3::Z),
        Mat4::looking_at(Vec3::ZERO, -Vec3::Y, -Vec3::Z),
        Mat4::looking_at(Vec3::ZERO, Vec3::Z, -Vec3::Y),
        Mat4::looking_at(Vec3::ZERO, -Vec3::Z, -Vec3::Y),
    ];

    let mip_map_levels = 5;
    for mip in 0..mip_map_levels {
        let mip_width = (size * (0.5f32).powf(mip as f32)) as u32;
        let mip_height = mip_width;

        let roughness = mip as f32 / (mip_map_levels - 1) as f32;

        for i in 0..6 {
            let mut command_buffer = graphics.context.new_command_buffer();

            let face_texture = cube_map.get_face_texture(i).with_mip(mip);
            let framebuffer = graphics
                .context
                .new_framebuffer(Some(&face_texture), None, None);
            {
                let mut render_pass = command_buffer
                    .begin_render_pass_with_framebuffer(&framebuffer, Some((1.0, 0.0, 0.0, 1.0)));
                render_pass.set_viewport(0, 0, mip_width, mip_height);

                render_pass.set_pipeline(&shader.shader.pipeline);
                render_pass.set_mat4_property(&shader.projection_property, projection.as_array());
                render_pass.set_mat4_property(&shader.view_property, views[i].as_array());
                render_pass.set_float_property(&shader.p_roughness, roughness);
                render_pass.set_cube_map_property(&shader.p_texture, Some(environment_texture), 0);
                render_pass.set_vertex_attribute(&shader.a_position, Some(&cube_mesh.positions));

                // Render mesh here.
                render_pass.draw_triangles(cube_mesh.triangle_count, &cube_mesh.index_buffer);
            }

            graphics.context.commit_command_buffer(command_buffer);
            graphics.context.delete_framebuffer(framebuffer);
        }
    }
}

pub fn bind_view(
    render_pass: &mut RenderPass,
    pipeline: &Pipeline,
    camera_info: &ViewInfo,
    i: usize,
) {
    // It's not great that these properties need to be looked up each time.
    render_pass.set_mat4_property(
        &pipeline
            .get_mat4_property(&format!("p_views[{:?}]", i))
            .unwrap(),
        camera_info.view_matrix.as_array(),
    );
    render_pass.set_mat4_property(
        &pipeline
            .get_mat4_property(&format!("p_projections[{:?}]", i))
            .unwrap(),
        camera_info.projection_matrix.as_array(),
    );
    render_pass.set_vec3_property(
        &pipeline
            .get_vec3_property(&format!("p_camera_positions[{:?}]", i))
            .unwrap(),
        camera_info.camera_position.into(),
    );
}

struct CubeMapLoadMessage {
    handle: Handle<CubeMap>,
    texture_load_data: TextureLoadData,
    texture_settings: TextureSettings,
    diffuse_and_specular_irradiance_cubemaps: Option<(Handle<CubeMap>, Handle<CubeMap>)>,
}

/// A system that loads textures onto the GPU
pub fn load_cube_maps(
    cube_maps: &mut Assets<CubeMap>,
    graphics: &mut Graphics,
    meshes: &Assets<Mesh>,
) {
    // A Vec doesn't need to be allocated here.
    // This is just a way to not borrow the TextureAssetLoader and Textures at
    // the same time.
    let messages: Vec<CubeMapLoadMessage> =
        cube_maps.asset_loader.receiver.inner().try_iter().collect();
    for message in messages.into_iter() {
        // Force ClampToEdge because other WrappingModes create a seam for CubeMaps.

        let mut texture_settings = TextureSettings {
            wrapping_horizontal: WrappingMode::ClampToEdge,
            wrapping_vertical: WrappingMode::ClampToEdge,
            minification_filter: FilterMode::Linear,
            magnification_filter: FilterMode::Linear,
            generate_mipmaps: false,
            ..message.texture_settings
        };

        let pixel_format = message.texture_load_data.pixel_format;
        // Create a GPU texture to process into the CubeMap
        let texture = new_texture_from_texture_load_data(
            graphics,
            message.texture_load_data,
            texture_settings,
        );

        // This needs to be true otherwise artifacts are introduced into the CubeMap.
        // Why?
        texture_settings.generate_mipmaps = true;
        let face_size = 512;
        // Hardcode the cube map's size for now.
        let cube_map = graphics
            .new_cube_map(None, face_size, face_size, pixel_format, texture_settings)
            .unwrap();

        render_cube_map(
            graphics,
            meshes,
            &cube_maps
                .asset_loader
                .cube_map_renderer
                .equirectangular_to_cubemap_shader,
            TextureIn::Texture(&texture),
            &cube_map,
            face_size as usize,
        );
        graphics.context.generate_mip_map_for_cube_map(&cube_map);

        // Manually free the texture here.
        graphics.context.delete_texture(texture.0);

        // If we also want to convolute the CubeMap do so here.
        if let Some((diffuse_handle, specular_handle)) =
            message.diffuse_and_specular_irradiance_cubemaps
        {
            let size = 32;
            let diffuse_irradiance_cubemap = graphics
                .new_cube_map(
                    None,
                    size,
                    size,
                    PixelFormat::RGBA16F,
                    TextureSettings {
                        srgb: false,
                        minification_filter: FilterMode::Linear,
                        magnification_filter: FilterMode::Linear,
                        wrapping_horizontal: WrappingMode::ClampToEdge,
                        wrapping_vertical: WrappingMode::ClampToEdge,
                        generate_mipmaps: false,
                        ..Default::default()
                    },
                )
                .unwrap();

            // Convolute here.
            render_cube_map(
                graphics,
                meshes,
                &cube_maps
                    .asset_loader
                    .cube_map_renderer
                    .diffuse_irradiance_convolution_shader,
                TextureIn::CubeMap(&cube_map),
                &diffuse_irradiance_cubemap,
                size as usize,
            );

            cube_maps.replace_placeholder(&diffuse_handle, diffuse_irradiance_cubemap);

            let specular_irradiance_cubemap = graphics
                .new_cube_map(
                    None,
                    128,
                    128,
                    PixelFormat::RGBA16F,
                    TextureSettings {
                        srgb: false,
                        minification_filter: FilterMode::Linear,
                        magnification_filter: FilterMode::Linear,
                        wrapping_horizontal: WrappingMode::ClampToEdge,
                        wrapping_vertical: WrappingMode::ClampToEdge,
                        generate_mipmaps: true,
                        ..Default::default()
                    },
                )
                .unwrap();

            render_specular_irradiance_cube_map(
                graphics,
                meshes,
                &cube_maps
                    .asset_loader
                    .cube_map_renderer
                    .specular_irradiance_shader,
                &cube_map,
                &specular_irradiance_cubemap,
            );

            cube_maps.replace_placeholder(&specular_handle, specular_irradiance_cubemap);
        }

        cube_maps.replace_placeholder(&message.handle, cube_map);
    }
}

pub struct CubeMapOptions {
    pub texture_settings: TextureSettings,
    pub diffuse_and_specular_irradiance_cubemaps: Option<(Handle<CubeMap>, Handle<CubeMap>)>,
}

impl Default for CubeMapOptions {
    fn default() -> Self {
        Self {
            texture_settings: TextureSettings::default(),
            diffuse_and_specular_irradiance_cubemaps: None,
        }
    }
}

impl LoadableAssetTrait for CubeMap {
    type Options = CubeMapOptions;
    type AssetLoader = NotSendSync<CubeMapAssetLoader>;
}

pub struct CubeMapAssetLoader {
    sender: SyncGuard<mpsc::Sender<CubeMapLoadMessage>>,
    receiver: SyncGuard<mpsc::Receiver<CubeMapLoadMessage>>,
    cube_map_renderer: CubeMapRenderer,
}

impl CubeMapAssetLoader {
    pub fn new(graphics: &mut Graphics) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: SyncGuard::new(sender),
            receiver: SyncGuard::new(receiver),
            cube_map_renderer: CubeMapRenderer::new(graphics),
        }
    }
}

impl AssetLoader<CubeMap> for NotSendSync<CubeMapAssetLoader> {
    fn load_with_options(
        &mut self,
        path: &str,
        handle: Handle<CubeMap>,
        mut options: <CubeMap as LoadableAssetTrait>::Options,
    ) {
        let path = path.to_owned();
        let sender = self.sender.inner().clone();

        ktasks::spawn(async move {
            let texture_load_data =
                texture_data_from_path(&path, &mut options.texture_settings).await;

            let _ = sender.send(CubeMapLoadMessage {
                texture_load_data,
                handle,
                texture_settings: options.texture_settings,
                diffuse_and_specular_irradiance_cubemaps: options
                    .diffuse_and_specular_irradiance_cubemaps,
            });
        })
        .run();
    }
}

#[derive(Component, Clone)]
pub struct ReflectionProbe {
    pub source: Handle<CubeMap>,
    pub diffuse_irradiance_map: Handle<CubeMap>,
    pub specular_irradiance_map: Handle<CubeMap>,
}

impl Assets<CubeMap> {
    pub fn load_reflection_probe(&mut self, path: &str) -> ReflectionProbe {
        let diffuse_irradiance_map = self.new_handle();
        let specular_irradiance_map = self.new_handle();

        let source = self.load_with_options(
            path,
            CubeMapOptions {
                diffuse_and_specular_irradiance_cubemaps: Some((
                    diffuse_irradiance_map.clone(),
                    specular_irradiance_map.clone(),
                )),
                ..Default::default()
            },
        );

        ReflectionProbe {
            source,
            diffuse_irradiance_map,
            specular_irradiance_map,
        }
    }
}

pub fn spawn_skybox(world: &mut World, path: &str) {
    let (reflection_probe, skybox_material) =
        (|cube_maps: &mut Assets<CubeMap>, materials: &mut Assets<Material>| {
            let reflection_probe = cube_maps.load_reflection_probe(path);

            let mut material = Material::new(Shader::SKY_BOX);
            material.set_cube_map("p_environment_map", reflection_probe.source.clone());
            (reflection_probe, materials.add(material))
        })
        .run(world);

    world.spawn((
        Name("Sky box"),
        Transform::new(),
        Mesh::CUBE_MAP_CUBE,
        Color::WHITE,
        crate::Texture::WHITE,
        skybox_material,
        RenderFlags::DO_NOT_CAST_SHADOWS
            .with_layer(RenderFlags::IGNORE_CULLING)
            .with_layer(RenderFlags::DEFAULT),
    ));
    world.spawn((Transform::new(), reflection_probe));
}

pub fn spawn_skybox_without_image_based_lighting(world: &mut World, path: &str) {
    let skybox_material = (|cube_maps: &mut Assets<CubeMap>, materials: &mut Assets<Material>| {
        let skybox_cube_map = cube_maps.load_with_options(
            path,
            CubeMapOptions {
                diffuse_and_specular_irradiance_cubemaps: None,
                ..Default::default()
            },
        );

        let mut material = Material::new(Shader::SKY_BOX);
        material.set_cube_map("p_environment_map", skybox_cube_map);
        materials.add(material)
    })
    .run(world);

    world.spawn((
        Name("Sky box"),
        Transform::new(),
        Mesh::CUBE_MAP_CUBE,
        Color::WHITE,
        crate::Texture::WHITE,
        skybox_material,
        RenderFlags::DO_NOT_CAST_SHADOWS
            .with_layer(RenderFlags::IGNORE_CULLING)
            .with_layer(RenderFlags::DEFAULT),
    ));
}

pub fn spawn_reflection_probe(world: &mut World, path: &str) {
    let (reflection_probe, skybox_material) =
        (|cube_maps: &mut Assets<CubeMap>, materials: &mut Assets<Material>| {
            let reflection_probe = cube_maps.load_reflection_probe(path);

            let mut material = Material::new(Shader::SKY_BOX);
            material.set_cube_map("p_environment_map", reflection_probe.source.clone());
            (reflection_probe, materials.add(material))
        })
        .run(world);

    world.spawn((Transform::new(), reflection_probe));
}
