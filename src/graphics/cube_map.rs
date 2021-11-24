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
    irradiance_convolution_shader: ShaderAndProperties,
}

impl CubeMapRenderer {
    pub fn new(graphics: &mut Graphics) -> CubeMapRenderer {
        let equirectangular_to_cubemap_shader = get_shader_and_properties(
            graphics,
            include_str!("built_in_shaders/equirectangular_to_cubemap.glsl"),
            false,
        );
        let irradiance_convolution_shader = get_shader_and_properties(
            graphics,
            include_str!("built_in_shaders/irradiance_convolution.glsl"),
            true,
        );
        CubeMapRenderer {
            equirectangular_to_cubemap_shader,
            irradiance_convolution_shader,
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
        // rd.start_frame_capture(std::ptr::null(), std::ptr::null());
        graphics.context.commit_command_buffer(command_buffer);
        // rd.end_frame_capture(std::ptr::null(), std::ptr::null());

        //  graphics.context.delete_framebuffer(framebuffer);
    }

    println!("HERE CREATING CUBEMAP");
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
    convoluted_handle: Option<Handle<CubeMap>>,
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

        // If we also want to convolute the CubeMap do so here.
        if let Some(convoluted_handle) = message.convoluted_handle {
            let size = 32;
            let convoluted_cube_map = graphics
                .new_cube_map(
                    None,
                    size,
                    size,
                    PixelFormat::RGB16F,
                    TextureSettings {
                        srgb: false,
                        minification_filter: FilterMode::Linear,
                        magnification_filter: FilterMode::Linear,
                        wrapping_horizontal: WrappingMode::ClampToEdge,
                        wrapping_vertical: WrappingMode::ClampToEdge,
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
                    .irradiance_convolution_shader,
                TextureIn::CubeMap(&cube_map),
                &convoluted_cube_map,
                size as usize,
            );

            cube_maps.replace_placeholder(&convoluted_handle, convoluted_cube_map)
        }

        cube_maps.replace_placeholder(&message.handle, cube_map);
    }
}

pub struct CubeMapOptions {
    pub texture_settings: TextureSettings,
    pub convoluted_cube_map: Option<Handle<CubeMap>>,
}

impl Default for CubeMapOptions {
    fn default() -> Self {
        Self {
            texture_settings: TextureSettings::default(),
            convoluted_cube_map: None,
        }
    }
}

impl LoadableAssetTrait for CubeMap {
    type Options = CubeMapOptions;
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
            let texture_load_data =
                texture_data_from_path(&path, &mut options.texture_settings).await;

            let _ = sender.send(CubeMapLoadMessage {
                texture_load_data,
                handle,
                texture_settings: options.texture_settings,
                convoluted_handle: options.convoluted_cube_map,
            });
        })
        .run();
    }
}

#[derive(Component, Clone)]
pub struct ReflectionProbe {
    pub irradiance_map: Handle<CubeMap>,
}
