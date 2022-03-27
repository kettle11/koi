use crate::*;
use kgraphics::*;

mod material;
use kmath::intersections::frustum_with_bounding_box;
pub use material::*;

mod pbr_material;
pub use pbr_material::*;

mod sprite;
pub use sprite::*;

mod brdf_lookup;

use crate::graphics::texture::Texture;

struct RenderTargetTexture {
    texture: Handle<Texture>,
    pixel_format: PixelFormat,
    texture_settings: TextureSettings,
    resolve_texture: Option<Handle<Texture>>,
}

impl RenderTargetTexture {
    pub fn resize(&mut self, size: Vec2u, graphics: &mut Graphics, textures: &mut Assets<Texture>) {
        self.texture = textures.add(
            graphics
                .new_texture(
                    None,
                    size.x as u32,
                    size.y as u32,
                    self.pixel_format,
                    self.texture_settings,
                )
                .unwrap(),
        );
        if let Some(resolve_texture) = self.resolve_texture.as_mut() {
            *resolve_texture = textures.add(
                graphics
                    .new_texture(
                        None,
                        size.x as u32,
                        size.y as u32,
                        self.pixel_format,
                        TextureSettings {
                            msaa_samples: 0,
                            ..self.texture_settings
                        },
                    )
                    .unwrap(),
            )
        }
    }
}

// Todo: Framebuffer is leaked when this is dropped.
pub struct OffscreenRenderTarget {
    framebuffer: Option<NotSendSync<Framebuffer>>,
    color_texture: Option<RenderTargetTexture>,
    depth_texture: Option<RenderTargetTexture>,
    resolve_framebuffer: Option<NotSendSync<Framebuffer>>,
    inner_texture_size: Vec2u,
    used_size: Vec2u,
    needs_resolve: bool,
}

impl OffscreenRenderTarget {
    pub fn new(
        graphics: &mut Graphics,
        textures: &mut Assets<Texture>,
        initial_size: Vec2u,
        color_pixel_format_and_texture_settings: Option<(PixelFormat, TextureSettings)>,
        depth_pixel_format_and_texture_settings: Option<(PixelFormat, TextureSettings)>,
    ) -> Self {
        // If both color and depth are defined check that they have the same msaa properties.
        debug_assert!(
            (color_pixel_format_and_texture_settings.is_none()
                || depth_pixel_format_and_texture_settings.is_none())
                || color_pixel_format_and_texture_settings.map(|(_, s)| s.msaa_samples)
                    == depth_pixel_format_and_texture_settings.map(|(_, s)| s.msaa_samples)
        );
        let needs_resolve =
            color_pixel_format_and_texture_settings.map_or(false, |(_, s)| s.msaa_samples != 0);
        let mut s = OffscreenRenderTarget {
            framebuffer: None,
            color_texture: color_pixel_format_and_texture_settings.map(|(p, s)| {
                RenderTargetTexture {
                    texture: Handle::default(),
                    pixel_format: p,
                    texture_settings: s,
                    resolve_texture: if needs_resolve {
                        Some(Handle::default())
                    } else {
                        None
                    },
                }
            }),
            depth_texture: depth_pixel_format_and_texture_settings.map(|(p, s)| {
                RenderTargetTexture {
                    texture: Handle::default(),
                    pixel_format: p,
                    texture_settings: s,
                    resolve_texture: if needs_resolve {
                        Some(Handle::default())
                    } else {
                        None
                    },
                }
            }),
            resolve_framebuffer: None,
            inner_texture_size: Vec2u::ZERO,
            used_size: Vec2u::ZERO,
            needs_resolve: color_pixel_format_and_texture_settings
                .as_ref()
                .map_or(false, |f| f.1.msaa_samples != 0),
        };
        s.resize(graphics, textures, initial_size);
        s
    }
    pub fn resize(&mut self, graphics: &mut Graphics, textures: &mut Assets<Texture>, size: Vec2u) {
        // Resize
        if size
            .greater_than_per_component(self.inner_texture_size)
            .any()
        {
            let size = size.max(self.inner_texture_size);
            self.color_texture
                .as_mut()
                .map(|c| c.resize(size, graphics, textures));
            self.depth_texture
                .as_mut()
                .map(|c| c.resize(size, graphics, textures));

            if let Some(framebuffer) = self.framebuffer.take() {
                graphics.context.delete_framebuffer(framebuffer.take())
            }
            self.framebuffer = Some(NotSendSync::new(
                graphics.context.new_framebuffer(
                    self.color_texture
                        .as_ref()
                        .map(|t| &textures.get(&t.texture).0),
                    self.depth_texture
                        .as_ref()
                        .map(|t| &textures.get(&t.texture).0),
                    None,
                ),
            ));

            if self.needs_resolve {
                if let Some(framebuffer) = self.resolve_framebuffer.take() {
                    graphics.context.delete_framebuffer(framebuffer.take())
                }
                self.resolve_framebuffer = Some(NotSendSync::new(
                    graphics.context.new_framebuffer(
                        self.color_texture
                            .as_ref()
                            .map(|t| &textures.get(t.resolve_texture.as_ref().unwrap()).0),
                        self.depth_texture
                            .as_ref()
                            .map(|t| &textures.get(t.resolve_texture.as_ref().unwrap()).0),
                        None,
                    ),
                ));
            }

            self.inner_texture_size = size;
        }

        self.used_size = size;
    }

    /// This assumes that the framebuffer is currently bound.
    pub fn resolve(&self, render_pass: RenderPass) {
        if let Some(resolve_framebuffer) = self.resolve_framebuffer.as_ref() {
            render_pass.blit_framebuffer(
                &**resolve_framebuffer,
                0,
                0,
                self.used_size.x as _,
                self.used_size.y as _,
                0,
                0,
                self.used_size.x as _,
                self.used_size.y as _,
            )
        }
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &*self.framebuffer.as_ref().unwrap()
    }

    /// Gets the readable color texture.
    pub fn color_texture(&self) -> &Handle<Texture> {
        let color_texture = self.color_texture.as_ref().unwrap();
        if let Some(texture) = color_texture.resolve_texture.as_ref() {
            texture
        } else {
            &color_texture.texture
        }
    }

    /// Gets the readable depth texture.
    pub fn depth_texture(&self) -> &Handle<Texture> {
        let depth_texture = self.depth_texture.as_ref().unwrap();
        if let Some(texture) = depth_texture.resolve_texture.as_ref() {
            texture
        } else {
            &depth_texture.texture
        }
    }

    pub fn inner_texture_scale(&self) -> Vec2 {
        self.used_size
            .as_f32()
            .div_by_component(self.inner_texture_size.as_f32())
    }
}

#[derive(NotCloneComponent)]
pub struct RendererInfo {
    pub brdf_lookup_table: Handle<Texture>,
    offscreen_render_target: OffscreenRenderTarget,
}

pub fn renderer_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_renderer.system()],
        end_of_frame_systems: vec![
            prepare_shadow_casters.system(),
            render_scene.system(),
            drop_materials.system(),
        ],
        ..Default::default()
    }
}

pub fn drop_materials(materials: &mut Assets<Material>) {
    materials.drop_items(|_| {})
}

pub fn setup_renderer(world: &mut World) {
    let default_material = new_pbr_material(Shader::PHYSICALLY_BASED, PBRProperties::default());
    let mut materials = Assets::<Material>::new(default_material, MaterialAssetLoader::new());
    Material::initialize_static_materials(&mut materials);
    world.spawn((Name("Assets<Material>".into()), materials));
    let brdf_lookup_table = brdf_lookup::generate_brdf_lookup.run(world);
    let brdf_lookup_table = world
        .get_single_component_mut::<Assets<Texture>>()
        .unwrap()
        .add(brdf_lookup_table);

    let initial_size = world.get_singleton::<NotSendSync<kapp::Window>>().size();

    let renderer_info = RendererInfo {
        brdf_lookup_table,
        offscreen_render_target: (|graphics: &mut Graphics, textures: &mut Assets<Texture>| {
            OffscreenRenderTarget::new(
                graphics,
                textures,
                Vec2u::new(initial_size.0 as usize, initial_size.1 as usize),
                Some((
                    PixelFormat::RGBA16F,
                    TextureSettings {
                        msaa_samples: 4,
                        srgb: false,
                        generate_mipmaps: false,
                        ..Default::default()
                    },
                )),
                Some((
                    PixelFormat::Depth16,
                    TextureSettings {
                        srgb: false,
                        msaa_samples: 4,
                        generate_mipmaps: false,
                        ..Default::default()
                    },
                )),
            )
        })
        .run(world),
    };
    world.spawn((Name("RendererInfo".into()), renderer_info));
}

pub struct ViewInfo {
    pub projection_matrix: Mat4,
    pub view_matrix: Mat4,
    pub camera_position: Vec3,
    pub viewport: Box2,
}

struct PipelineInfo {
    model_property: Mat4Property,
    position_attribute: VertexAttribute<Vec3>,
    normal_attribute: VertexAttribute<Vec3>,
    vertex_color_attribute: VertexAttribute<Vec4>,
    texture_coordinate_attribute: VertexAttribute<Vec2>,
    base_color_property: Vec4Property,
    base_color_texture_property: TextureProperty,
    texture_coordinate_offset_property: Vec2Property,
    texture_coordinate_scale_property: Vec2Property,
    sprite_texture_unit: Option<u8>,
}

struct Renderer<'a, 'b: 'a> {
    render_pass: &'a mut RenderPass<'b>,
    camera_info: &'a [ViewInfo],
    shader_assets: &'a Assets<Shader>,
    material_assets: &'a Assets<Material>,
    mesh_assets: &'a Assets<Mesh>,
    texture_assets: &'a Assets<Texture>,
    cube_map_assets: &'a Assets<CubeMap>,
    bound_mesh: Option<&'a Handle<Mesh>>,
    bound_shader: Option<&'a Handle<Shader>>,
    material_handle: Option<&'a Handle<Material>>,
    pipeline_info: Option<PipelineInfo>,
    #[allow(unused)]
    multiview_enabled: bool,
    current_pipeline: Option<&'a Pipeline>,
    dither_scale: f32,
    just_changed_material: bool,
    brdf_lookup_texture: &'a Texture,
    color_is_set: bool,
}

impl<'a, 'b: 'a> Renderer<'a, 'b> {
    pub fn new(
        renderer_info: &'a RendererInfo,
        render_pass: &'a mut RenderPass<'b>,
        shader_assets: &'a Assets<Shader>,
        material_assets: &'a Assets<Material>,
        mesh_assets: &'a Assets<Mesh>,
        texture_assets: &'a Assets<Texture>,
        cube_map_assets: &'a Assets<CubeMap>,
        camera_info: &'a [ViewInfo],
        viewport: kmath::geometry::BoundingBox<u32, 2>,
        multiview_enabled: bool,
    ) -> Self {
        let min = viewport.min;
        let size = viewport.size();
        render_pass.set_viewport(min.x as u32, min.y as u32, size.x as u32, size.y as u32);
        let brdf_lookup_texture = texture_assets.get(&renderer_info.brdf_lookup_table);
        Self {
            render_pass,
            // camera_info,
            shader_assets,
            material_assets,
            texture_assets,
            cube_map_assets,
            mesh_assets,
            bound_mesh: None,
            bound_shader: None,
            material_handle: None,
            pipeline_info: None,
            camera_info,
            multiview_enabled,
            current_pipeline: None,
            dither_scale: 4.0,
            just_changed_material: false,
            brdf_lookup_texture,
            color_is_set: false,
        }
    }

    fn get_view_info(
        camera_global_transform: &GlobalTransform,
        offset: Mat4,
        projection_matrix: Mat4,
        viewport: Box2,
    ) -> ViewInfo {
        // Is this `offset *` correct?
        let camera_position = offset.transform_point(camera_global_transform.position);
        // let projection_matrix = camera.projection_matrix();
        let view_matrix = (camera_global_transform.model() * offset).inversed();

        ViewInfo {
            projection_matrix,
            view_matrix,
            camera_position,
            viewport,
        }
    }

    fn bind_light_info(&mut self, pipeline: &Pipeline, lights: &Lights, max_texture_unit: u8) {
        self.render_pass.set_int_property(
            &pipeline.get_int_property("p_light_count").unwrap(),
            lights.iter().count() as i32,
        );

        for (i, (transform, light, shadow_caster)) in lights.iter().enumerate() {
            if transform.position.is_nan() {
                dbg!("Light position is NaN");
                continue;
            }

            self.render_pass.set_vec3_property(
                &pipeline
                    .get_vec3_property(&format!("p_lights[{:?}].position", i))
                    .unwrap(),
                (transform.position).into(),
            );

            self.render_pass.set_vec3_property(
                &pipeline
                    .get_vec3_property(&format!("p_lights[{:?}].direction", i))
                    .unwrap(),
                (transform.forward()).into(),
            );

            self.render_pass.set_float_property(
                &pipeline
                    .get_float_property(&format!("p_lights[{:?}].ambient", i))
                    .unwrap(),
                light.ambient_light_amount,
            );

            // TODO: Make a color property and convert it into the framebuffer's color space first.
            let color_and_intensity = light
                .color
                .to_rgb_color(crate::color_spaces::LINEAR_SRGB)
                .xyz()
                * light.intensity;

            self.render_pass.set_vec3_property(
                &pipeline
                    .get_vec3_property(&format!("p_lights[{:?}].color_and_intensity", i))
                    .unwrap(),
                color_and_intensity.into(),
            );

            self.render_pass.set_int_property(
                &pipeline
                    .get_int_property(&format!("p_lights[{:?}].mode", i))
                    .unwrap(),
                match light.light_mode {
                    LightMode::Directional => 0,
                    LightMode::Point { .. } => 1,
                },
            );

            if let LightMode::Point { radius } = light.light_mode {
                self.render_pass.set_float_property(
                    &pipeline
                        .get_float_property(&format!("p_lights[{:?}].radius", i))
                        .unwrap(),
                    radius,
                );
            }

            // Set if the light has shadows enabled.
            self.render_pass.set_int_property(
                &pipeline
                    .get_int_property(&format!("p_lights[{:?}].shadows_enabled", i))
                    .unwrap(),
                if shadow_caster.is_some() { 1 } else { 0 },
            );
            self.render_pass.set_float_property(
                &pipeline
                    .get_float_property(&format!("p_lights[{:?}].ibl_shadowing", i))
                    .unwrap(),
                if let Some(shadow_caster) = shadow_caster {
                    shadow_caster.ibl_shadowing
                } else {
                    0.0
                },
            );

            // If this light casts shadows, update the shadow caster info.
            if let Some(shadow_caster) = shadow_caster {
                let mut texture_unit_offset = max_texture_unit;
                for (index, cascade) in shadow_caster.shadow_cascades.iter().enumerate() {
                    let depth_texture = self
                        .texture_assets
                        .get(&cascade.offscreen_render_target.depth_texture());

                    self.render_pass.set_texture_property(
                        &pipeline
                            .get_texture_property(&format!("p_light_shadow_maps_{:?}", index))
                            .unwrap(),
                        Some(depth_texture),
                        texture_unit_offset,
                    );
                    texture_unit_offset += 1;

                    self.render_pass.set_mat4_property(
                        &pipeline
                            .get_mat4_property(&format!("p_world_to_light_space_{:?}", index))
                            .unwrap(),
                        cascade.world_to_light_space.as_array(),
                    );
                }
            }
        }
    }

    pub fn bind_view(&mut self, camera_info: &ViewInfo, i: usize) {
        // It's not great that these properties need to be looked up each time.
        let pipeline = self.current_pipeline.unwrap();
        self.render_pass.set_mat4_property(
            &pipeline
                .get_mat4_property(&format!("p_views[{:?}]", i))
                .unwrap(),
            camera_info.view_matrix.as_array(),
        );
        self.render_pass.set_mat4_property(
            &pipeline
                .get_mat4_property(&format!("p_projections[{:?}]", i))
                .unwrap(),
            camera_info.projection_matrix.as_array(),
        );
        self.render_pass.set_vec3_property(
            &pipeline
                .get_vec3_property(&format!("p_camera_positions[{:?}]", i))
                .unwrap(),
            camera_info.camera_position.into(),
        );
    }

    pub fn change_material(
        &mut self,
        material_handle: &'a Handle<Material>,
        lights: &Lights,
        reflection_probes: &Query<(&'static GlobalTransform, &'static ReflectionProbe)>,
    ) {
        // Avoid unnecessary [Material] rebinds.
        // Todo: For now we rebind the entire material if a color variant has been set. This makes color variants less efficient.
        // A better strategy should be found.
        if Some(material_handle) != self.material_handle || self.color_is_set {
            self.color_is_set = false;
            //println!("CHANGE MATERIAL");
            self.just_changed_material = true;

            // When a pipeline change occurs a bunch of uniforms need to be rebound.
            let material = self.material_assets.get(material_handle);
            let shader = self.shader_assets.get(&material.shader);

            #[cfg(not(feature = "xr"))]
            let pipeline = &shader.pipeline;
            #[cfg(feature = "xr")]
            let pipeline = if self.multiview_enabled {
                shader.multiview_pipeline.as_ref().unwrap()
            } else {
                &shader.pipeline
            };

            // THIS IS A HACK FOR NOW
            // This should be replaced with a pipeline-specific max-texture unit
            let max_texture_unit = 5;

            // Avoid unnecessary pipeline / shader changes.
            // For now this is commented out but it could be reintroduced later.
            // This check would prevent pipleline changes if the material is different but the pipeline is the same.
            if self.bound_shader != Some(&material.shader) {
                //println!("CHANGE SHADER");

                self.current_pipeline = Some(pipeline);
                self.render_pass.set_pipeline(pipeline);

                for (i, camera_info) in self.camera_info.iter().enumerate() {
                    self.bind_view(camera_info, i);
                }

                self.render_pass.set_float_property(
                    &pipeline.get_float_property("p_dither_scale").unwrap(),
                    self.dither_scale,
                );

                // When a material is changed we lookup if the shader has some standard properties.
                let model_property = pipeline.get_mat4_property("p_model").unwrap();
                let position_attribute =
                    pipeline.get_vertex_attribute::<Vec3>("a_position").unwrap();
                let normal_attribute = pipeline.get_vertex_attribute::<Vec3>("a_normal").unwrap();
                let texture_coordinate_attribute = pipeline
                    .get_vertex_attribute::<Vec2>("a_texture_coordinate")
                    .unwrap();
                let vertex_color_attribute =
                    pipeline.get_vertex_attribute::<Vec4>("a_color").unwrap();

                // Cache properties that may be changed per Sprite.
                let base_color_texture_property = pipeline
                    .get_texture_property("p_base_color_texture")
                    .unwrap();
                let texture_coordinate_offset_property = pipeline
                    .get_vec2_property("p_texture_coordinate_offset")
                    .unwrap();
                let texture_coordinate_scale_property = pipeline
                    .get_vec2_property("p_texture_coordinate_scale")
                    .unwrap();
                let sprite_texture_unit = material
                    .texture_properties
                    .get("p_base_color_texture")
                    .map(|p| p.1);
                let base_color_property = pipeline.get_vec4_property("p_base_color").unwrap();

                // Bind light and shadow info.
                self.bind_light_info(pipeline, lights, max_texture_unit + 4);

                // Set fog values
                self.render_pass
                    .set_float_property(&pipeline.get_float_property("p_fog_start").unwrap(), 20.);
                self.render_pass
                    .set_float_property(&pipeline.get_float_property("p_fog_end").unwrap(), 8000.);
                self.render_pass.set_vec4_property(
                    &pipeline.get_vec4_property("p_fog_color").unwrap(),
                    (1.0, 1.0, 1.0, 1.0),
                );
                // Bind the reflection probe
                let (reflection_probe_diffuse, reflection_probe_specular) =
                    if let Some((_, reflection_probe)) = reflection_probes.iter().next() {
                        (
                            self.cube_map_assets
                                .get(&reflection_probe.diffuse_irradiance_map),
                            self.cube_map_assets
                                .get(&reflection_probe.specular_irradiance_map),
                        )
                    } else {
                        let default = self.cube_map_assets.get(&Handle::default());
                        (default, default)
                    };

                self.render_pass.set_cube_map_property(
                    &pipeline.get_cube_map_property("p_irradiance_map").unwrap(),
                    Some(reflection_probe_diffuse),
                    max_texture_unit + 1,
                );

                self.render_pass.set_cube_map_property(
                    &pipeline.get_cube_map_property("p_prefilter_map").unwrap(),
                    Some(reflection_probe_specular),
                    max_texture_unit + 2,
                );

                // Bind the brdf lookup table.

                self.render_pass.set_texture_property(
                    &pipeline
                        .get_texture_property("p_brdf_lookup_table")
                        .unwrap(),
                    Some(self.brdf_lookup_texture),
                    max_texture_unit + 3,
                );

                self.bound_shader = Some(&material.shader);
                self.pipeline_info = Some(PipelineInfo {
                    model_property,
                    position_attribute,
                    normal_attribute,
                    texture_coordinate_attribute,
                    vertex_color_attribute,
                    base_color_property,
                    base_color_texture_property,
                    texture_coordinate_offset_property,
                    texture_coordinate_scale_property,
                    sprite_texture_unit,
                });
            }
            self.material_handle = Some(material_handle);

            // Rebind the material properties.
            material.bind_material(
                self.render_pass,
                pipeline,
                self.texture_assets,
                self.cube_map_assets,
            );
        }
    }

    pub fn set_color(&mut self, color: Color) {
        if let Some(material_info) = &self.pipeline_info {
            let rgb_color = color.to_rgb_color(color_spaces::LINEAR_SRGB);

            self.render_pass
                .set_vec4_property(&material_info.base_color_property, rgb_color.into());
            self.color_is_set = true;
        }
    }

    pub fn prepare_sprite(&mut self, sprite: &Sprite) {
        if let Some(material_info) = &self.pipeline_info {
            if let Some(sprite_texture_unit) = material_info.sprite_texture_unit {
                let primary_texture = self.texture_assets.get(&sprite.texture_handle);

                self.render_pass.set_texture_property(
                    &material_info.base_color_texture_property,
                    Some(primary_texture),
                    sprite_texture_unit,
                );

                self.render_pass.set_vec2_property(
                    &material_info.texture_coordinate_offset_property,
                    sprite.sprite_source_bounds.min.into(),
                );
                self.render_pass.set_vec2_property(
                    &material_info.texture_coordinate_scale_property,
                    sprite.sprite_source_bounds.size().into(),
                );
            }
        }
    }

    pub fn render_mesh(&mut self, transform: &Transform, mesh_handle: &'a Handle<Mesh>) {
        // Instead of checking this here there should always be standard material properties, just
        // for a default material.
        if let Some(material_info) = &self.pipeline_info {
            let mesh = self.mesh_assets.get(mesh_handle);

            if let Some(gpu_mesh) = &mesh.gpu_mesh {
                // Only rebind the mesh attributes if the mesh has changed
                // or if the material has been changed since the mesh was bound.
                if Some(mesh_handle) != self.bound_mesh || self.just_changed_material {
                    self.render_pass.set_vertex_attribute(
                        &material_info.position_attribute,
                        Some(&gpu_mesh.positions),
                    );
                    self.render_pass.set_vertex_attribute(
                        &material_info.normal_attribute,
                        gpu_mesh.normals.as_ref(),
                    );

                    self.render_pass.set_vertex_attribute(
                        &material_info.texture_coordinate_attribute,
                        gpu_mesh.texture_coordinates.as_ref(),
                    );

                    if let Some(colors) = gpu_mesh.colors.as_ref() {
                        self.render_pass.set_vertex_attribute(
                            &material_info.vertex_color_attribute,
                            Some(colors),
                        );
                    } else {
                        self.render_pass.set_vertex_attribute_to_constant(
                            &material_info.vertex_color_attribute,
                            &[1.0, 1.0, 1.0, 1.0],
                        );
                    }

                    self.bound_mesh = Some(mesh_handle);
                }
                let model_matrix = transform.model();
                self.render_pass
                    .set_mat4_property(&material_info.model_property, model_matrix.as_array());

                if self.camera_info.len() == 1 || self.multiview_enabled {
                    self.render_pass
                        .draw_triangles(gpu_mesh.triangle_count, &gpu_mesh.index_buffer);
                } else {
                    // Render the thing for each view if we're rendering in XR
                    for camera_info in self.camera_info.iter() {
                        // This needs to be scaled by the pixels in the view.
                        let size = camera_info.viewport.size();
                        self.render_pass.set_viewport(
                            camera_info.viewport.min.x as u32,
                            camera_info.viewport.min.y as u32,
                            size.x as u32,
                            size.y as u32,
                        );
                        self.bind_view(camera_info, 0);

                        // Render the mesh
                        self.render_pass
                            .draw_triangles(gpu_mesh.triangle_count, &gpu_mesh.index_buffer);
                    }
                }
            }
        }
        self.just_changed_material = false;
    }

    pub fn render_scene(
        &mut self,
        camera: &Camera,
        camera_transform: &GlobalTransform,
        renderables: &'a Renderables,
        lights: &'a Lights,
        reflection_probes: &Query<(&'static GlobalTransform, &'static ReflectionProbe)>,
    ) {
        self.render_pass.set_depth_mask(true);

        let camera_position = camera_transform.position;
        let camera_forward = -camera_transform.forward();

        let frustum = if self.camera_info.len() > 1 {
            let mut frustum_left = Frustum::from_matrix(
                self.camera_info[0].projection_matrix * self.camera_info[0].view_matrix,
            );
            let frustum_right = Frustum::from_matrix(
                self.camera_info[1].projection_matrix * self.camera_info[1].view_matrix,
            );
            // Expand the culling frustum to accomodate both views.
            // Note: This approach won't work for headseys with a greater than 180 field of view.
            frustum_left.planes[1] = frustum_right.planes[1];
            frustum_left
        } else {
            Frustum::from_matrix(
                self.camera_info[0].projection_matrix * self.camera_info[0].view_matrix,
            )
        };

        // These should *really* be preallocated somehow.
        let mut transparent_renderables = Vec::new();
        let mut non_transparent_renderables = Vec::new();

        for renderable in renderables.iter() {
            let (transform, material_handle, mesh_handle, render_flags, _optional_sprite, _color) =
                renderable;
            let render_flags = render_flags.cloned().unwrap_or(RenderFlags::DEFAULT);

            if camera.render_flags.includes_layer(render_flags) {
                let should_render = render_flags.includes_layer(RenderFlags::IGNORE_CULLING)
                    || self
                        .mesh_assets
                        .get(mesh_handle)
                        .bounding_box
                        .map_or(true, |b| {
                            intersections::frustum_with_bounding_box(&frustum, transform.model(), b)
                        });

                if should_render {
                    let is_transparent = self
                        .shader_assets
                        .get(&self.material_assets.get(material_handle).shader)
                        .pipeline
                        .blending()
                        .is_some();

                    if is_transparent {
                        transparent_renderables.push(renderable);
                    } else {
                        non_transparent_renderables.push(renderable);
                    }
                }
            }
        }

        non_transparent_renderables.sort_by(
            |(_, material_a, mesh_a, _, _, _), (_, material_b, mesh_b, _, _, _)| {
                // Sort by material then mesh.
                // In the future sorting could occur by pipeline as well.
                let cmp = material_a.cmp(&material_b);
                match cmp {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Greater => cmp,
                    _ => mesh_a.cmp(mesh_b),
                }
            },
        );

        for renderable in non_transparent_renderables {
            let (transform, material_handle, mesh_handle, _render_flags, optional_sprite, color) =
                renderable;

            self.change_material(material_handle, lights, reflection_probes);
            if let Some(sprite) = optional_sprite {
                self.prepare_sprite(sprite);
            }
            if let Some(color) = color {
                self.set_color(*color);
            }

            self.render_mesh(transform, mesh_handle);
        }

        transparent_renderables.sort_by(|(a, ..), (b, ..)| {
            let v0 = (a.position - camera_position).dot(camera_forward);
            let v1 = (b.position - camera_position).dot(camera_forward);
            v0.partial_cmp(&v1).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Don't write to depth for transparent objects.
        // This prevents transparent objects from occluding each-other.
        // self.render_pass.set_depth_mask(false);

        for renderable in transparent_renderables.iter() {
            let (transform, material_handle, mesh_handle, _render_flags, optional_sprite, color) =
                *renderable;
            self.change_material(material_handle, lights, reflection_probes);
            if let Some(sprite) = optional_sprite {
                self.prepare_sprite(sprite);
            }

            if let Some(color) = color {
                self.set_color(*color);
            }
            self.render_mesh(transform, mesh_handle);
        }
    }
}

type Renderables<'a> = Query<
    'a,
    (
        &'static GlobalTransform,
        &'static Handle<Material>,
        &'static Handle<Mesh>,
        //Option<&Handle<Texture>>,
        Option<&'static RenderFlags>,
        Option<&'static Sprite>,
        Option<&'static Color>,
    ),
>;

type Lights<'a> = Query<
    'a,
    (
        &'static GlobalTransform,
        &'static Light,
        Option<&'static mut ShadowCaster>,
    ),
>;

pub fn prepare_shadow_casters(
    graphics: &mut Graphics,
    textures: &mut Assets<Texture>,
    mut shadow_casters: Query<&mut ShadowCaster>,
) {
    for shadow_caster in &mut shadow_casters {
        shadow_caster.prepare_shadow_casting(graphics, textures);
    }
}

pub fn render_scene<'a, 'b>(
    graphics: &mut Graphics,
    shader_assets: &Assets<Shader>,
    material_assets: &Assets<Material>,
    mesh_assets: &Assets<Mesh>,
    texture_assets: &mut Assets<Texture>,
    cube_map_assets: &Assets<CubeMap>,
    cameras: Query<(&GlobalTransform, &Camera)>,
    renderables: Renderables<'a>,
    mut lights: Lights<'b>,
    reflection_probes: Query<(&'static GlobalTransform, &'static ReflectionProbe)>,
    renderer_info: &mut RendererInfo,
) {
    let mut command_buffer = graphics.context.new_command_buffer();

    let is_primary_camera_target =
        graphics.current_camera_target == Some(graphics.primary_camera_target);

    let mut clear_color = None;

    // For now only render shadows from the primary camera's perspective.
    // This would make splitscreen shadows really messed up.
    for (camera_global_transform, camera) in &cameras {
        let is_primary_camera = ((graphics.current_camera_target.is_some()
            && graphics.current_camera_target == camera.camera_target)
            || (is_primary_camera_target && camera.camera_target == Some(CameraTarget::Primary)))
            && camera.render_flags.includes_layer(RenderFlags::DEFAULT);

        if is_primary_camera {
            render_shadow_pass(
                shader_assets,
                mesh_assets,
                &mut command_buffer,
                camera,
                camera_global_transform,
                &mut lights,
                &renderables,
            );

            // Resize of the offscreen render target to match the view size.
            renderer_info
                .offscreen_render_target
                .resize(graphics, texture_assets, {
                    let view_size = camera.get_view_size();
                    Vec2u::new(view_size.0 as _, view_size.1 as _)
                });

            clear_color = camera.clear_color;
            break;
        }
    }

    let clear_color = clear_color.map(|c| {
        // Presently the output needs to be in non-linear sRGB.
        // However that means that blending with the clear-color will be incorrect.
        // A post-processing pass is needed to convert into the appropriate output space.
        let c = c.to_rgb_color(color_spaces::ENCODED_SRGB);
        c.into()
    });

    {
        let mut render_pass = command_buffer.begin_render_pass_with_framebuffer(
            renderer_info.offscreen_render_target.framebuffer(),
            clear_color,
        );

        for (camera_global_transform, camera) in &cameras {
            // Check that the camera is setup to render to the current CameraTarget.
            let camera_should_render = (graphics.current_camera_target.is_some()
                && graphics.current_camera_target == camera.camera_target)
                || (is_primary_camera_target
                    && camera.camera_target == Some(CameraTarget::Primary));

            // Check that this camera targets the target currently being rendered.
            if camera_should_render {
                let mut camera_info = Vec::new();
                if graphics.override_views.is_empty() {
                    camera_info.push(Renderer::get_view_info(
                        camera_global_transform,
                        Mat4::IDENTITY,
                        camera.projection_matrix(),
                        Box2 {
                            min: Vec2::ZERO,
                            max: Vec2::ONE,
                        },
                    ));
                } else {
                    for view in &graphics.override_views {
                        camera_info.push(Renderer::get_view_info(
                            camera_global_transform,
                            view.offset_transform,
                            view.projection_matrix,
                            view.output_rectangle,
                        ))
                    }
                }

                /*
                #[cfg(not(feature = "xr"))]
                let multiview_enabled = false;
                #[cfg(feature = "xr")]
                let multiview_enabled = camera_info.len() > 1;
                */
                let multiview_enabled = false;

                let (width, height) = camera.get_view_size();

                let mut renderer = Renderer::new(
                    renderer_info,
                    &mut render_pass,
                    shader_assets,
                    material_assets,
                    mesh_assets,
                    texture_assets,
                    cube_map_assets,
                    &camera_info,
                    kmath::geometry::BoundingBox::<u32, 2> {
                        min: Vector::ZERO,
                        max: Vector::<u32, 2>::new(width, height),
                    },
                    multiview_enabled,
                );

                renderer.render_scene(
                    camera,
                    camera_global_transform,
                    &renderables,
                    &lights,
                    &reflection_probes,
                );
            }
        }

        renderer_info.offscreen_render_target.resolve(render_pass);

        let mut render_pass = command_buffer
            .begin_render_pass_with_framebuffer(&graphics.current_target_framebuffer, clear_color);

        let shader = shader_assets.get(&Shader::FULLSCREEN_QUAD);
        render_pass.set_pipeline(&shader.pipeline);

        render_pass.set_texture_property(
            &shader.pipeline.get_texture_property("p_texture").unwrap(),
            Some(texture_assets.get(renderer_info.offscreen_render_target.color_texture())),
            0,
        );
        render_pass.set_vec2_property(
            &shader
                .pipeline
                .get_vec2_property("p_texture_coordinate_scale")
                .unwrap(),
            renderer_info
                .offscreen_render_target
                .inner_texture_scale()
                .into(),
        );

        render_pass.draw_triangles_without_buffer(1);
    }

    command_buffer.present();
    graphics.context.commit_command_buffer(command_buffer);
}

pub fn render_depth_only(
    shaders: &Assets<Shader>,
    meshes: &Assets<Mesh>,
    command_buffer: &mut CommandBuffer,
    framebuffer: &Framebuffer,
    view_matrix: &Mat4,
    projection_matrix: &Mat4,
    viewport_size: u32,
    renderables: &Renderables,
) {
    let mut render_pass =
        command_buffer.begin_render_pass_with_framebuffer(framebuffer, Some((0.0, 0.0, 0.0, 0.0)));
    render_pass.set_viewport(0, 0, viewport_size, viewport_size);

    render_pass.set_depth_mask(true);

    let depth_shader = shaders.get(&Shader::DEPTH_ONLY);

    render_pass.set_pipeline(&depth_shader.pipeline);
    render_pass.set_mat4_property(
        &depth_shader
            .pipeline
            .get_mat4_property("p_views[0]")
            .unwrap(),
        view_matrix.as_array(),
    );
    render_pass.set_mat4_property(
        &depth_shader
            .pipeline
            .get_mat4_property("p_projections[0]")
            .unwrap(),
        projection_matrix.as_array(),
    );

    let model_property = depth_shader.pipeline.get_mat4_property("p_model").unwrap();
    let position_attribute = depth_shader
        .pipeline
        .get_vertex_attribute::<Vec3>("a_position")
        .unwrap();

    let culling_frustum = Frustum::from_matrix(*projection_matrix * *view_matrix);

    for (global_transform, _, mesh_handle, render_flags, _, _) in renderables {
        let render_flags = render_flags.cloned().unwrap_or(RenderFlags::DEFAULT);
        if render_flags.includes_layer(RenderFlags::DEFAULT)
            && !render_flags.includes_layer(RenderFlags::DO_NOT_CAST_SHADOWS)
        {
            let mesh = meshes.get(mesh_handle);
            let should_render = render_flags.includes_layer(RenderFlags::IGNORE_CULLING)
                || meshes.get(mesh_handle).bounding_box.map_or(true, |b| {
                    frustum_with_bounding_box(&culling_frustum, global_transform.model(), b)
                });

            if should_render {
                if let Some(gpu_mesh) = mesh.gpu_mesh.as_ref() {
                    render_pass
                        .set_mat4_property(&model_property, global_transform.model().as_array());
                    render_pass
                        .set_vertex_attribute(&position_attribute, Some(&gpu_mesh.positions));
                    render_pass.draw_triangles(gpu_mesh.triangle_count, &gpu_mesh.index_buffer);
                }
            }
        }
    }
}

// A shadow caster for a light.
#[derive(NotCloneComponent)]
pub struct ShadowCaster {
    pub(crate) shadow_cascades: Vec<ShadowCascadeInfo>,
    pub(crate) texture_size: u32,
    pub ibl_shadowing: f32,
}

impl ShadowCaster {
    pub fn new() -> Self {
        Self {
            shadow_cascades: Vec::new(),
            texture_size: 2048,
            ibl_shadowing: 0.0,
        }
    }

    pub fn with_ibl_shadowing(mut self, ibl_shadowing: f32) -> Self {
        self.ibl_shadowing = ibl_shadowing;
        self
    }
}

pub(crate) struct ShadowCascadeInfo {
    offscreen_render_target: OffscreenRenderTarget,
    pub(crate) world_to_light_space: Mat4,
}

impl ShadowCaster {
    fn prepare_shadow_casting(&mut self, graphics: &mut Graphics, textures: &mut Assets<Texture>) {
        if self.shadow_cascades.is_empty() {
            // Setup shadow textures
            for _ in 0..4 {
                let offscreen_render_target = OffscreenRenderTarget::new(
                    graphics,
                    textures,
                    Vec2u::new(self.texture_size as _, self.texture_size as _),
                    None,
                    Some((
                        kgraphics::PixelFormat::Depth32F,
                        TextureSettings {
                            // Nearest because this will not have mipmaps generated
                            minification_filter: kgraphics::FilterMode::Nearest,
                            magnification_filter: kgraphics::FilterMode::Nearest,
                            wrapping_horizontal: kgraphics::WrappingMode::ClampToEdge, // This should be clamp to border but that's not supported on WebGL
                            wrapping_vertical: kgraphics::WrappingMode::ClampToEdge, // This should be clamp to border but that's not supported on WebGL
                            srgb: false,
                            generate_mipmaps: false,
                            ..Default::default()
                        },
                    )),
                );

                self.shadow_cascades.push(ShadowCascadeInfo {
                    offscreen_render_target,
                    world_to_light_space: Mat4::ZERO, // This gets set later.
                });
            }
        }
    }
}

pub fn render_shadow_pass(
    shaders: &Assets<Shader>,
    meshes: &Assets<Mesh>,
    command_buffer: &mut CommandBuffer,
    camera: &Camera,
    camera_global_transform: &GlobalTransform,
    lights: &mut Lights,
    renderables: &Renderables,
) {
    let camera_view_inversed = camera_global_transform.model();

    let cascade_depths = [5., 15., 30., 60.];

    let mut z_near = camera.get_near_plane();
    let mut camera_clip_space_to_world = [Mat4::ZERO; 4];
    for (i, z_far) in cascade_depths.iter().enumerate() {
        // The +1.0 to z_far here prevents an issue where lines appear between cascades.
        let projection = camera.projection_matrix_with_z_near_and_z_far(z_near, z_far + 1.0);
        camera_clip_space_to_world[i] = camera_view_inversed * projection.inversed();
        z_near = *z_far;
    }

    // In the future this could be reduced to light's that area of influence overlaps the camera's frustum.
    for (light_global_transform, _light, shadow_caster) in lights {
        if let Some(shadow_caster) = shadow_caster {
            // Render shadow map cascades
            for (i, cascade) in shadow_caster.shadow_cascades.iter_mut().enumerate() {
                let view_matrix = light_global_transform.model().inversed();
                let camera_to_light_space = view_matrix * camera_clip_space_to_world[i];

                // Is negative z correct here?
                let corners = [
                    camera_to_light_space * (Vec4::new(-1., -1., 1., 1.)),
                    camera_to_light_space * (Vec4::new(1., -1., 1., 1.)),
                    camera_to_light_space * (Vec4::new(1., 1., 1., 1.)),
                    camera_to_light_space * (Vec4::new(-1., 1., 1., 1.)),
                    camera_to_light_space * (Vec4::new(-1., -1., -1., 1.)),
                    camera_to_light_space * (Vec4::new(1., -1., -1., 1.)),
                    camera_to_light_space * (Vec4::new(1., 1., -1., 1.)),
                    camera_to_light_space * (Vec4::new(-1., 1., -1., 1.)),
                ];

                let corners = [
                    corners[0].xyz() / corners[0].w,
                    corners[1].xyz() / corners[1].w,
                    corners[2].xyz() / corners[2].w,
                    corners[3].xyz() / corners[3].w,
                    corners[4].xyz() / corners[4].w,
                    corners[5].xyz() / corners[5].w,
                    corners[6].xyz() / corners[6].w,
                    corners[7].xyz() / corners[7].w,
                ];

                // Clamp the shadow map bounding box to texel edges to reduce shimmering
                let bounding_box = Box3::from_points(corners);
                let world_units_per_texel = bounding_box.size() / shadow_caster.texture_size as f32;
                let min = (bounding_box.min.div_by_component(world_units_per_texel))
                    .floor()
                    .mul_by_component(world_units_per_texel);
                let max = (bounding_box.max.div_by_component(world_units_per_texel))
                    .floor()
                    .mul_by_component(world_units_per_texel);
                let bounding_box = Box3 { min, max };

                // The light's matrix must enclose these.

                // It would be better to detect objects within the light's bounds and determine where the near
                // and far planes should go appropriately.
                let shadow_behind_light = 300.;
                let projection_matrix = kmath::projection_matrices::orthographic_gl(
                    bounding_box.min[0],
                    bounding_box.max[0],
                    bounding_box.min[1],
                    bounding_box.max[1],
                    -bounding_box.max[2] - shadow_behind_light, // I don't understand why these need to be negated and reversed
                    -bounding_box.min[2] + shadow_behind_light,
                );

                // println!("PROJECTION MATRIX{:#?}", projection_matrix);

                cascade.world_to_light_space = projection_matrix * view_matrix;

                render_depth_only(
                    shaders,
                    meshes,
                    command_buffer,
                    cascade.offscreen_render_target.framebuffer(),
                    &view_matrix,
                    &projection_matrix,
                    shadow_caster.texture_size,
                    renderables,
                );
            }
        }
    }
}
