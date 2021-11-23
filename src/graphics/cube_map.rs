use crate::*;
use kgraphics::*;
use std::sync::mpsc;

pub use kgraphics::CubeMap;

pub struct CubeMapRenderer {
    shader: Shader,
    projection_property: Mat4Property,
    view_property: Mat4Property,
    equirectangular_map_property: TextureProperty,
    position_attribute: VertexAttribute<Vec3>,
}

impl CubeMapRenderer {
    pub fn new(graphics: &mut Graphics) -> CubeMapRenderer {
        let shader = graphics
            .new_shader(
                include_str!("built_in_shaders/equirectangular_to_cubemap.glsl"),
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
        let equirectangular_map_property = shader
            .pipeline
            .get_texture_property("equirectangular_map")
            .unwrap();

        let position_attribute = shader.pipeline.get_vertex_attribute("a_position").unwrap();

        CubeMapRenderer {
            shader,
            projection_property,
            view_property,
            equirectangular_map_property,
            position_attribute,
        }
    }

    fn render_cube_map(
        &mut self,
        graphics: &mut Graphics,
        meshes: &Assets<Mesh>,
        texture: &kgraphics::Texture,
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
            // let mut rd: renderdoc::RenderDoc<renderdoc::V110> =
            //     renderdoc::RenderDoc::new().expect("Unable to connect");

            let mut command_buffer = graphics.context.new_command_buffer();

            let face_texture = cube_map.get_face_texture(i);
            let framebuffer = graphics
                .context
                .new_framebuffer(Some(&face_texture), None, None);
            {
                let mut render_pass = command_buffer
                    .begin_render_pass_with_framebuffer(&framebuffer, Some((1.0, 0.0, 0.0, 1.0)));
                render_pass.set_viewport(0, 0, size as u32, size as u32);

                render_pass.set_pipeline(&self.shader.pipeline);
                render_pass.set_mat4_property(&self.projection_property, projection.as_array());
                render_pass.set_mat4_property(&self.view_property, views[i].as_array());
                render_pass.set_texture_property(
                    &self.equirectangular_map_property,
                    Some(texture),
                    0,
                );

                render_pass
                    .set_vertex_attribute(&self.position_attribute, Some(&cube_mesh.positions));

                // Render mesh here.
                render_pass.draw_triangles(cube_mesh.triangle_count, &cube_mesh.index_buffer);
            }
            // rd.start_frame_capture(std::ptr::null(), std::ptr::null());
            graphics.context.commit_command_buffer(command_buffer);
            // rd.end_frame_capture(std::ptr::null(), std::ptr::null());

            //  graphics.context.delete_framebuffer(framebuffer);
        }

        println!("HERE CREATING CUBEMAP");
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
}

/// A system that loads textures onto the GPU
pub(crate) fn load_cube_maps(
    cube_maps: &mut Assets<CubeMap>,
    graphics: &mut Graphics,
    meshes: &Assets<Mesh>,
) {
    // A Vec doesn't need to be allocated here.
    // This is just a way to not borrow the TextureAssetLoader and Textures at
    // the same time.
    let messages: Vec<CubeMapLoadMessage> =
        cube_maps.asset_loader.receiver.inner().try_iter().collect();
    for mut message in messages.into_iter() {
        // Force ClampToEdge because other WrappingModes create a seam for CubeMaps.
        message.texture_settings.wrapping_horizontal = WrappingMode::ClampToEdge;
        message.texture_settings.wrapping_vertical = WrappingMode::ClampToEdge;

        // Create a GPU texture to process into the CubeMap
        let texture = graphics
            .new_texture(
                Some(&message.texture_load_data.data),
                message.texture_load_data.width,
                message.texture_load_data.height,
                message.texture_load_data.pixel_format,
                message.texture_settings,
            )
            .unwrap();

        let face_size = 512;
        // Hardcode the cube map's size for now.
        let cube_map = graphics
            .new_cube_map(
                None,
                face_size,
                face_size,
                message.texture_load_data.pixel_format,
                message.texture_settings,
            )
            .unwrap();

        cube_maps.asset_loader.cube_map_renderer.render_cube_map(
            graphics,
            meshes,
            &texture,
            &cube_map,
            face_size as usize,
        );
        cube_maps.replace_placeholder(&message.handle, cube_map);
    }
}

impl LoadableAssetTrait for CubeMap {
    type Options = TextureSettings;
    type AssetLoader = CubeMapAssetLoader;
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

impl AssetLoader<CubeMap> for CubeMapAssetLoader {
    fn load_with_options(
        &mut self,
        path: &str,
        handle: Handle<CubeMap>,
        mut options: <CubeMap as LoadableAssetTrait>::Options,
    ) {
        let path = path.to_owned();
        let sender = self.sender.inner().clone();

        ktasks::spawn(async move {
            let texture_load_data = texture_data_from_path(&path, &mut options).await;

            let _ = sender.send(CubeMapLoadMessage {
                texture_load_data,
                handle,
                texture_settings: options,
            });
        })
        .run();
    }
}
