use crate::*;
use kgraphics::*;

mod material;
pub use material::*;

pub fn renderer_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_renderer.system()],
        end_of_frame_systems: vec![render_scene.system()],
        ..Default::default()
    }
}

pub fn setup_renderer(world: &mut World) {
    let mut materials = Assets::<Material>::new(Material::new(Handle::default()));
    Material::initialize_static_materials(&mut materials, &UNLIT_SHADER);
    world.spawn(materials);
}

struct CameraInfo {
    projection_matrix: Mat4,
    view_matrix: Mat4,
    camera_position: Vec3,
}

struct MaterialInfo<'a> {
    material_handle: &'a Handle<Material>,
    model_property: Mat4Property,
    position_attribute: VertexAttribute<Vec3>,
    normal_attribute: VertexAttribute<Vec3>,
    vertex_color_attribute: VertexAttribute<Vec4>,
    texture_coordinate_attribute: VertexAttribute<Vec2>,
}

struct Renderer<'a, 'b: 'a> {
    render_pass: &'a mut RenderPass<'b>,
    camera_info: CameraInfo,
    shader_assets: &'a Assets<Shader>,
    material_assets: &'a Assets<Material>,
    mesh_assets: &'a Assets<Mesh>,
    texture_assets: &'a Assets<Texture>,
    bound_mesh: Option<&'a Handle<Mesh>>,
    bound_shader: Option<&'a Handle<Shader>>,
    material_info: Option<MaterialInfo<'a>>,
}

impl<'a, 'b: 'a> Renderer<'a, 'b> {
    pub fn new(
        render_pass: &'a mut RenderPass<'b>,
        camera_transform: &'a Transform,
        camera: &Camera,
        shader_assets: &'a Assets<Shader>,
        material_assets: &'a Assets<Material>,
        mesh_assets: &'a Assets<Mesh>,
        texture_assets: &'a Assets<Texture>,
    ) -> Self {
        let camera_info = Self::get_camera_info(camera_transform, camera);

        let (view_width, view_height) = camera.get_view_size();
        render_pass.set_viewport(0, 0, view_width, view_height);

        Self {
            render_pass,
            camera_info,
            shader_assets,
            material_assets,
            texture_assets,
            mesh_assets,
            bound_mesh: None,
            bound_shader: None,
            material_info: None,
        }
    }

    fn get_camera_info(camera_transform: &Transform, camera: &Camera) -> CameraInfo {
        let camera_transform = camera_transform.global_transform;
        let camera_position = camera_transform.position;
        let projection_matrix = camera.projection_matrix();
        let view_matrix = camera_transform.model().inversed();

        CameraInfo {
            projection_matrix,
            view_matrix,
            camera_position,
        }
    }

    pub fn change_material(&mut self, material_handle: &'a Handle<Material>) {
        // Avoid unnecessary [Material] rebinds.
        if Some(material_handle) != self.material_info.as_ref().map(|m| m.material_handle) {
            // When a pipeline change occurs a bunch of uniforms need to be rebound.
            let material = self.material_assets.get(material_handle);
            let shader = self.shader_assets.get(&material.shader);

            // Avoid unnecessary pipeline / shader changes.
            if self.bound_shader != Some(&material.shader) {
                self.render_pass.set_pipeline(&shader.pipeline);

                self.render_pass.set_mat4_property(
                    &shader.pipeline.get_mat4_property("p_view").unwrap(),
                    self.camera_info.view_matrix.as_array(),
                );
                self.render_pass.set_mat4_property(
                    &shader.pipeline.get_mat4_property("p_projection").unwrap(),
                    self.camera_info.projection_matrix.as_array(),
                );
                self.render_pass.set_vec3_property(
                    &shader
                        .pipeline
                        .get_vec3_property("p_camera_position")
                        .unwrap(),
                    self.camera_info.camera_position.into(),
                );

                // When a material is changed we lookup if the shader has some standard properties.
                let model_property = shader.pipeline.get_mat4_property("p_model").unwrap();
                let position_attribute = shader
                    .pipeline
                    .get_vertex_attribute::<Vec3>("a_position")
                    .unwrap();
                let normal_attribute = shader
                    .pipeline
                    .get_vertex_attribute::<Vec3>("a_normal")
                    .unwrap();
                let texture_coordinate_attribute = shader
                    .pipeline
                    .get_vertex_attribute::<Vec2>("a_texture_coordinate")
                    .unwrap();
                let vertex_color_attribute = shader
                    .pipeline
                    .get_vertex_attribute::<Vec4>("a_color")
                    .unwrap();

                self.bound_shader = Some(&material.shader);
                self.material_info = Some(MaterialInfo {
                    material_handle,
                    model_property,
                    position_attribute,
                    normal_attribute,
                    texture_coordinate_attribute,
                    vertex_color_attribute,
                });
            }

            // // Rebind the material properties.
            material.bind_material(self.render_pass, &shader.pipeline, self.texture_assets);
        }
    }

    pub fn render_mesh(&mut self, transform: &Transform, mesh_handle: &'a Handle<Mesh>) {
        // Instead of checking this here there should always be standard material properties, just
        // for a default material.
        if Some(mesh_handle) != self.bound_mesh {
            if let Some(material_info) = &self.material_info {
                let mesh = self.mesh_assets.get(mesh_handle);

                if let Some(gpu_mesh) = &mesh.gpu_mesh {
                    let model_matrix = transform.model();

                    self.render_pass
                        .set_mat4_property(&material_info.model_property, model_matrix.as_array());

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
                    self.render_pass.set_vertex_attribute(
                        &material_info.vertex_color_attribute,
                        gpu_mesh.colors.as_ref(),
                    );

                    // Render the mesh
                    self.render_pass
                        .draw_triangles(gpu_mesh.triangle_count, &gpu_mesh.index_buffer);

                    self.bound_mesh = Some(mesh_handle);
                }
            }
        }
    }
}

pub fn render_scene(
    graphics: &mut Graphics,
    shader_assets: &Assets<Shader>,
    material_assets: &Assets<Material>,
    mesh_assets: &Assets<Mesh>,
    texture_assets: &Assets<Texture>,
    cameras: Query<(&Transform, &Camera)>,
    renderables: Query<(&Transform, &Handle<Material>, &Handle<Mesh>)>,
) {
    let mut command_buffer = graphics.context.new_command_buffer();
    let frame = graphics.render_target.current_frame().unwrap();

    for (camera_transform, camera) in &cameras {
        let clear_color = camera.clear_color.map(|c| {
            // Presently the output needs to be in non-linear sRGB.
            // However that means that blending with the clear-color will be incorrect.
            // A post-processing pass is needed to convert into the appropriate output space.
            let c = c.to_rgb_color(color_spaces::ENCODED_SRGB);
            (c.red, c.green, c.blue, c.alpha)
        });

        let mut render_pass =
            command_buffer.begin_render_pass(Some(&frame), Some(&frame), None, clear_color);

        {
            let mut renderer = Renderer::new(
                &mut render_pass,
                camera_transform,
                camera,
                shader_assets,
                material_assets,
                mesh_assets,
                texture_assets,
            );
            for (transform, material_handle, mesh_handle) in &renderables {
                renderer.change_material(material_handle);
                renderer.render_mesh(transform, mesh_handle);
            }
        }
    }

    graphics.context.commit_command_buffer(command_buffer);
}
