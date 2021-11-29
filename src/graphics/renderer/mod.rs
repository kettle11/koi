use crate::*;
use kgraphics::*;

mod material;
pub use material::*;

mod pbr_material;
pub use pbr_material::*;

mod sprite;
pub use sprite::*;

use crate::graphics::texture::Texture;

pub fn renderer_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_renderer.system()],
        end_of_frame_systems: vec![render_scene.system()],
        ..Default::default()
    }
}

pub fn setup_renderer(world: &mut World) {
    let default_material = new_pbr_material(Shader::PHYSICALLY_BASED, PBRProperties::default());
    let mut materials = Assets::<Material>::new(default_material, MaterialAssetLoader::new());
    Material::initialize_static_materials(&mut materials);
    world.spawn(materials);
}

pub struct ViewInfo {
    pub projection_matrix: Mat4,
    pub view_matrix: Mat4,
    pub camera_position: Vec3,
    pub viewport: Box2,
}

struct MaterialInfo<'a> {
    material_handle: &'a Handle<Material>,
    model_property: Mat4Property,
    position_attribute: VertexAttribute<Vec3>,
    normal_attribute: VertexAttribute<Vec3>,
    vertex_color_attribute: VertexAttribute<Vec4>,
    texture_coordinate_attribute: VertexAttribute<Vec2>,
    base_color_property: Vec4Property,
    base_color_texture_property: TextureProperty,
    texture_coordinate_offset_property: Vec2Property,
    texture_coordinate_scale_property: Vec2Property,
    sprite_texture_unit: u8,
}

pub type RendererQuery<'a> = (
    &'a mut Graphics,
    &'a Assets<Shader>,
    &'a Assets<Material>,
    &'a Assets<Mesh>,
    &'a Assets<Texture>,
);

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
    material_info: Option<MaterialInfo<'a>>,
    #[allow(unused)]
    multiview_enabled: bool,
    current_pipeline: Option<&'a Pipeline>,
    dither_scale: f32,
    just_changed_material: bool,
}

impl<'a, 'b: 'a> Renderer<'a, 'b> {
    pub fn new(
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
        // let camera_info = Self::get_camera_info(camera_transform, camera);
        // let (view_width, view_height) = camera.get_view_size();
        let min = viewport.min;
        let size = viewport.size();
        render_pass.set_viewport(min.x as u32, min.y as u32, size.x as u32, size.y as u32);

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
            material_info: None,
            camera_info,
            multiview_enabled,
            current_pipeline: None,
            dither_scale: 4.0,
            just_changed_material: false,
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
        let view_matrix = (offset * camera_global_transform.model()).inversed();

        ViewInfo {
            projection_matrix,
            view_matrix,
            camera_position,
            viewport,
        }
    }

    fn bind_light_info(
        &mut self,
        pipeline: &Pipeline,
        lights: &Query<'_, (&'static GlobalTransform, &'static Light)>,
    ) {
        self.render_pass.set_int_property(
            &pipeline.get_int_property("p_light_count").unwrap(),
            lights.iter().count() as i32,
        );
        for (i, (transform, light)) in lights.iter().enumerate() {
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

            // Set if the light has shadows enabled.
            // Harcoded to false for now
            self.render_pass.set_int_property(
                &pipeline
                    .get_int_property(&format!("p_lights[{:?}].shadows_enabled", i))
                    .unwrap(),
                0,
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
        lights: &Query<'_, (&'static GlobalTransform, &'static Light)>,
        reflection_probes: &Query<(&'static GlobalTransform, &'static ReflectionProbe)>,
    ) {
        // Avoid unnecessary [Material] rebinds.
        if Some(material_handle) != self.material_info.as_ref().map(|m| m.material_handle) {
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

            // Avoid unnecessary pipeline / shader changes.
            if self.bound_shader != Some(&material.shader) {
                self.current_pipeline = Some(pipeline);
                self.render_pass.set_pipeline(pipeline);

                if self.multiview_enabled || self.camera_info.len() == 1 {
                    for (i, camera_info) in self.camera_info.iter().enumerate() {
                        self.bind_view(camera_info, i);
                    }
                }

                self.render_pass.set_float_property(
                    &shader
                        .pipeline
                        .get_float_property("p_dither_scale")
                        .unwrap(),
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
                    .map(|p| p.1)
                    .unwrap_or(material.max_texture_unit);
                let base_color_property = pipeline.get_vec4_property("p_base_color").unwrap();

                self.bind_light_info(pipeline, lights);

                // Bind the reflection probe
                let reflection_probe_cube_map = self.cube_map_assets.get(
                    reflection_probes
                        .iter()
                        .next()
                        .map_or(&Handle::default(), |p| &p.1.diffuse_irradiance_map),
                );
                self.render_pass.set_cube_map_property(
                    &pipeline.get_cube_map_property(&"p_irradiance_map").unwrap(),
                    Some(reflection_probe_cube_map),
                    material.max_texture_unit + 1,
                );

                self.bound_shader = Some(&material.shader);
                self.material_info = Some(MaterialInfo {
                    material_handle,
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
        if let Some(material_info) = &self.material_info {
            let rgb_color = color.to_rgb_color(color_spaces::LINEAR_SRGB);

            self.render_pass
                .set_vec4_property(&material_info.base_color_property, rgb_color.into());
        }
    }

    pub fn prepare_sprite(&mut self, sprite: &Sprite) {
        if let Some(material_info) = &self.material_info {
            let primary_texture = self.texture_assets.get(&sprite.texture_handle);
            self.render_pass.set_texture_property(
                &material_info.base_color_texture_property,
                Some(primary_texture),
                material_info.sprite_texture_unit,
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

    pub fn render_mesh(&mut self, transform: &Transform, mesh_handle: &'a Handle<Mesh>) {
        // Instead of checking this here there should always be standard material properties, just
        // for a default material.
        if let Some(material_info) = &self.material_info {
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

                if self.camera_info.len() == 1 {
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
        renderables: &'a Renderables,
        lights: &Query<'_, (&'static GlobalTransform, &'static Light)>,
        reflection_probes: &Query<(&'static GlobalTransform, &'static ReflectionProbe)>,
    ) {
        for (transform, material_handle, mesh_handle, render_layer, optional_sprite, color) in
            renderables.iter()
        {
            let render_layer = render_layer.cloned().unwrap_or(RenderLayers::DEFAULT);
            if camera.render_layers.includes_layer(render_layer) {
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
}

type Renderables<'a> = Query<
    'a,
    (
        &'static GlobalTransform,
        &'static Handle<Material>,
        &'static Handle<Mesh>,
        //Option<&Handle<Texture>>,
        Option<&'static RenderLayers>,
        Option<&'static Sprite>,
        Option<&'static Color>,
    ),
>;

pub fn render_scene<'a>(
    graphics: &mut Graphics,
    shader_assets: &Assets<Shader>,
    material_assets: &Assets<Material>,
    mesh_assets: &Assets<Mesh>,
    texture_assets: &Assets<Texture>,
    cube_map_assets: &Assets<CubeMap>,
    cameras: Query<(&GlobalTransform, &Camera)>,
    renderables: Renderables<'a>,
    lights: Query<(&'static GlobalTransform, &'static Light)>,
    reflection_probes: Query<(&'static GlobalTransform, &'static ReflectionProbe)>,
) {
    let mut command_buffer = graphics.context.new_command_buffer();

    let is_primary_camera_target =
        graphics.current_camera_target == Some(graphics.primary_camera_target);

    if let Some((_, camera)) = cameras.iter().next() {
        let clear_color = camera.clear_color.map(|c| {
            // Presently the output needs to be in non-linear sRGB.
            // However that means that blending with the clear-color will be incorrect.
            // A post-processing pass is needed to convert into the appropriate output space.
            let c = c.to_rgb_color(color_spaces::ENCODED_SRGB);
            c.into()
        });

        let mut render_pass = command_buffer
            .begin_render_pass_with_framebuffer(&graphics.current_target_framebuffer, clear_color);

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

                // #[cfg(not(feature = "xr"))]
                let multiview_enabled = false;
                // #[cfg(feature = "xr")]
                // let multiview_enabled = camera_info.len() > 1;

                let (width, height) = camera.get_view_size();

                let mut renderer = Renderer::new(
                    &mut render_pass,
                    shader_assets,
                    material_assets,
                    mesh_assets,
                    texture_assets,
                    cube_map_assets,
                    // &lights,
                    &camera_info,
                    kmath::geometry::BoundingBox::<u32, 2> {
                        min: Vector::ZERO,
                        max: Vector::<u32, 2>::new(width, height),
                    },
                    multiview_enabled,
                );

                renderer.render_scene(camera, &renderables, &lights, &reflection_probes);
            }
        }
    }

    graphics.context.commit_command_buffer(command_buffer);
}
