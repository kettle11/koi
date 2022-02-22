use std::collections::HashMap;

pub use crate::graphics::texture::Texture;
use crate::*;
use kgraphics::*;

pub use kgraphics::{
    BlendFactor, FacesToRender, FilterMode, Framebuffer, Pipeline, TextureSettings, WrappingMode,
};

mod camera;
pub use camera::*;

mod camera_controls;
pub use camera_controls::*;

mod render_flags;
pub use render_flags::*;

mod texture;
pub use texture::*;

mod cube_map;
pub use cube_map::*;

mod mesh;
pub use mesh::*;

mod shader;
pub use shader::*;

mod mesh_primitives;
pub use mesh_primitives::*;

mod light;
pub use light::*;

mod immediate_drawer;
pub use immediate_drawer::*;

mod shader_parser;

#[cfg(feature = "renderer")]
mod renderer;
#[cfg(feature = "renderer")]
pub use renderer::*;

#[cfg(feature = "imagine_png")]
pub(crate) mod imagine_integration;

pub fn graphics_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_graphics.system()],
        pre_fixed_update_systems: vec![
            assign_current_camera_target.system(),
            check_for_dropped_graphics_assets.system(),
        ],
        draw_systems: vec![load_shaders.system(), resize_window.system()],
        end_of_frame_systems: vec![
            load_textures.system(),
            load_cube_maps.system(),
            automatic_redraw_request.system(),
        ],
        ..Default::default()
    }
}

pub fn request_window_redraw(world: &mut World) {
    (|window: &mut NotSendSync<kapp::Window>, #[cfg(feature = "xr")] xr: &crate::XR| {
        #[cfg(feature = "xr")]
        if !xr.running() {
            window.request_redraw();
        }
        #[cfg(not(feature = "xr"))]
        window.request_redraw();
    })
    .run(world);
}

/// Ensure that the primary window redraws continuously.
fn automatic_redraw_request(
    graphics: &mut Graphics,
    window: &mut NotSendSync<kapp::Window>,
    time: &mut Time,
    #[cfg(feature = "xr")] xr: &crate::XR,
) {
    if graphics.request_redraw || graphics.automatic_request_redraw {
        #[cfg(feature = "xr")]
        if !xr.running() {
            window.request_redraw();
        }
        #[cfg(not(feature = "xr"))]
        window.request_redraw();
        graphics.request_redraw = graphics.automatic_request_redraw;
    } else {
        time.discontinuity = true;
    }
}

// Alias this type so that it's simpler to query for it.
// On other platforms it might be possible to free `Graphics` of the `NotSendSync` requirement.
pub type Graphics = NotSendSync<GraphicsInner>;

/// Stores graphics info.
pub struct GraphicsInner {
    pub context: GraphicsContext,
    // For now there's an assumption of one window.
    pub render_target: RenderTarget,
    /// This target is assigned based on which source initialized this draw iteration.
    pub current_camera_target: Option<CameraTarget>,
    /// This can vary based on if a window or XR headset is primary.
    pub primary_camera_target: CameraTarget,
    /// Views the primary camera should use instead of its default view.
    /// This is used by XR devices.
    pub override_views: Vec<GraphicsViewInfo>,
    pub current_target_framebuffer: Framebuffer,
    /// Shader snippets that can be pasted into shaders.
    shader_snippets: HashMap<&'static str, &'static str>,
    #[cfg(feature = "xr")]
    multiview_support: MultiviewSupport,
    automatic_request_redraw: bool,
    request_redraw: bool,
}

#[derive(Clone, Debug)]
pub struct GraphicsViewInfo {
    pub projection_matrix: Mat4,
    /// How this view should be offset from the camera transform.
    pub offset_transform: Mat4,
    pub output_rectangle: Box2,
}

#[derive(Clone, Copy)]
pub struct PipelineSettings {
    pub faces_to_render: FacesToRender,
    pub blending: Option<(BlendFactor, BlendFactor)>,
    pub depth_test: DepthTest,
}

impl Default for PipelineSettings {
    fn default() -> Self {
        Self {
            faces_to_render: FacesToRender::Front,
            blending: None,
            depth_test: DepthTest::LessOrEqual,
        }
    }
}

#[derive(Debug)]
pub enum PipelineError {
    MissingVertexSection,
    MissingFragmentSection,
    VertexCompilationError(String),
    FragmentCompilationError(String),
    PipelineCompilationError(String),
}

fn setup_graphics(world: &mut World) {
    let main_window = world
        .get_single_component_mut::<NotSendSync<kapp::Window>>()
        .unwrap();

    let mut context = GraphicsContext::new_with_settings(GraphicsContextSettings {
        high_resolution_framebuffer: true,
        /// How many MSAA samples the window framebuffer should have
        samples: 4,
    })
    .unwrap();

    let main_window: &kapp::Window = main_window;

    let (window_width, window_height) = main_window.size();
    context.resize(main_window, window_width, window_height);

    #[cfg(not(feature = "SDL"))]
    let render_target = unsafe {
        context
            .get_render_target_for_window(main_window, window_width, window_height)
            .unwrap()
    };

    #[cfg(feature = "SDL")]
    let render_target = unsafe {
        context
            .get_render_target_for_window_sdl(main_window.id, window_width, window_height)
            .unwrap()
    };

    #[cfg(feature = "xr")]
    let multiview_support = context.get_multiview_supported();

    let mut graphics = NotSendSync::new(GraphicsInner {
        context,
        render_target,
        current_camera_target: None,
        primary_camera_target: CameraTarget::Window(main_window.id),
        override_views: Vec::new(),
        current_target_framebuffer: Framebuffer::default(),
        shader_snippets: HashMap::new(),
        #[cfg(feature = "xr")]
        multiview_support,
        automatic_request_redraw: true,
        request_redraw: true,
    });

    graphics.register_shader_snippet(
        "standard_vertex",
        include_str!("built_in_shaders/standard_vertex_snippet.glsl"),
    );

    graphics.register_shader_snippet(
        "fullscreen_vertex",
        include_str!("built_in_shaders/fullscreen_vertex_snippet.glsl"),
    );

    let default_mesh = graphics.new_gpu_mesh(&MeshData::default()).unwrap();
    let mut mesh_assets = Assets::<Mesh>::new(
        Mesh {
            gpu_mesh: Some(default_mesh),
            mesh_data: Some(MeshData::default()),

            // The default mesh is empty so give it no bounding-box.
            bounding_box: None,
        },
        MeshAssetLoader::new(),
    );

    // Initialize asset stores and their placeholders.
    let white_texture = graphics
        .new_texture(
            Some(&[255, 255, 255, 255]),
            1,
            1,
            PixelFormat::RGBA8Unorm,
            TextureSettings {
                srgb: false,
                ..Default::default()
            },
        )
        .unwrap();
    let mut texture_assets = Assets::new(white_texture, TextureAssetLoader::new());

    let default_shader = graphics
        .new_shader(
            include_str!("built_in_shaders/unlit.glsl"),
            PipelineSettings {
                faces_to_render: FacesToRender::Front,
                blending: None,
                ..Default::default()
            },
        )
        .unwrap();
    let mut shader_assets = Assets::new(default_shader, ShaderAssetLoader::new());

    initialize_static_primitives(&mut graphics, &mut mesh_assets);
    initialize_static_textures(&mut graphics, &mut texture_assets);
    initialize_static_shaders(&mut graphics, &mut shader_assets);

    let black_cube_map = graphics
        .new_cube_map(
            Some([
                &[0, 0, 0, 0],
                &[0, 0, 0, 0],
                &[0, 0, 0, 0],
                &[0, 0, 0, 0],
                &[0, 0, 0, 0],
                &[0, 0, 0, 0],
            ]),
            1,
            1,
            PixelFormat::RGBA8Unorm,
            TextureSettings {
                srgb: false,
                ..Default::default()
            },
        )
        .unwrap();

    let cube_map_assets = Assets::new(
        black_cube_map,
        NotSendSync::new(CubeMapAssetLoader::new(&mut graphics)),
    );

    world.spawn((Name("Graphics"), graphics));
    world.spawn((Name("Assets<Mesh>"), mesh_assets));
    world.spawn((Name("Assets<Texture>"), texture_assets));
    world.spawn((Name("Assets<Shader>"), shader_assets));
    world.spawn((Name("Assets<CubeMap>"), cube_map_assets));
}

fn assign_current_camera_target(graphics: &mut Graphics, events: &KappEvents) {
    match events.last() {
        Some(KappEvent::Draw { window_id }) => {
            graphics.current_camera_target = Some(CameraTarget::Window(*window_id));
            graphics.current_target_framebuffer = Framebuffer::default();
        }
        // Ignore user events because they're likely related to WebXR which may assign the CameraTarget.
        Some(KappEvent::UserEvent { .. }) => {}
        _ => panic!("Unexpected last Kapp event"),
    }
}

impl GraphicsInner {
    pub fn request_redraw(&mut self) {
        self.request_redraw = true;
    }

    pub fn set_automatic_redraw(&mut self, automatic_redraw: bool) {
        self.automatic_request_redraw = automatic_redraw;
    }

    fn create_pipeline(
        &mut self,
        source: &str,
        prepend: &str,
        pipeline_settings: PipelineSettings,
    ) -> Result<Pipeline, PipelineError> {
        let (vertex_source, fragment_source) =
            shader_parser::parse_shader(&self.shader_snippets, source, prepend);

        let vertex_function = self
            .context
            .new_vertex_function(&vertex_source)
            .map_err(PipelineError::VertexCompilationError)?;
        let fragment_function = self
            .context
            .new_fragment_function(&fragment_source)
            .map_err(PipelineError::FragmentCompilationError)?;

        self.context
            .new_pipeline(
                vertex_function,
                fragment_function,
                /* Todo: This arbitrary pixel format is a problem */ PixelFormat::RG8Unorm,
            )
            // For now all pipelines just have alpha blending by default.
            .blending(pipeline_settings.blending)
            .faces_to_render(pipeline_settings.faces_to_render)
            .build()
            .map_err(PipelineError::PipelineCompilationError)
    }

    /// koi shaders are both in the same file with #VERTEX and #FRAGMENT to annotate the vertex
    /// and fragment sections.
    pub fn new_shader(
        &mut self,
        source: &str,
        pipeline_settings: PipelineSettings,
    ) -> Result<Shader, PipelineError> {
        let pipeline = self.create_pipeline(source, "#define NUM_VIEWS 1 \n", pipeline_settings)?;

        #[cfg(feature = "xr")]
        let multiview_pipeline = match self.multiview_support {
            MultiviewSupport::None => None,
            MultiviewSupport::WithoutMsaa | MultiviewSupport::OculusWithMsaa => {
                Some(self.create_pipeline(
                    source,
                    &"#define NUM_VIEWS 2 \n #define MULTIVIEW \n",
                    pipeline_settings,
                )?)
            }
        };

        Ok(Shader {
            pipeline,
            #[cfg(feature = "xr")]
            multiview_pipeline,
        })
    }

    pub fn new_texture(
        &mut self,
        data: Option<&[u8]>,
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        texture_settings: kgraphics::TextureSettings,
    ) -> Result<Texture, ()> {
        Ok(Texture(self.context.new_texture(
            width,
            height,
            data,
            pixel_format,
            texture_settings,
        )?))
    }

    pub fn new_cube_map(
        &mut self,
        data: Option<[&[u8]; 6]>,
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        texture_settings: kgraphics::TextureSettings,
    ) -> Result<CubeMap, ()> {
        self.context
            .new_cube_map(width, height, data, pixel_format, texture_settings)
    }

    pub fn new_gpu_mesh(&mut self, mesh_data: &MeshData) -> Result<GPUMesh, ()> {
        // Check that all of the indices point to valid vertices.
        // If this causes performance issues this check could be disabled in the future.
        let len = mesh_data.positions.len();
        for i in mesh_data.indices.iter() {
            assert!(
                !(i[0] as usize >= len || i[1] as usize >= len || i[2] as usize >= len),
                "Mesh indices refer to out of bound vertices: {:?}. Vertex count: {:?}",
                i,
                mesh_data.positions.len()
            );
        }

        let triangle_count = mesh_data.indices.len() as u32;

        // Flatten the index buffer
        let index_buffer: &[u32] = unsafe {
            std::slice::from_raw_parts(
                mesh_data.indices.as_ptr() as *const u32,
                mesh_data.indices.len() * 3,
            )
        };

        let texture_coordinates = if !mesh_data.texture_coordinates.is_empty() {
            assert!(mesh_data.texture_coordinates.len() == len);
            Some(
                self.context
                    .new_data_buffer(&mesh_data.texture_coordinates)?,
            )
        } else {
            None
        };
        let normals = if !mesh_data.normals.is_empty() {
            assert!(mesh_data.normals.len() == len);
            Some(self.context.new_data_buffer(&mesh_data.normals)?)
        } else {
            None
        };

        let colors = if !mesh_data.colors.is_empty() {
            assert!(mesh_data.colors.len() == len);
            Some(self.context.new_data_buffer(&mesh_data.colors)?)
        } else {
            None
        };

        Ok(GPUMesh {
            positions: self.context.new_data_buffer(&mesh_data.positions)?,
            texture_coordinates,
            normals,
            index_buffer: self.context.new_index_buffer(index_buffer)?,
            triangle_count,
            colors,
        })
    }

    pub fn register_shader_snippet(&mut self, name: &'static str, snippet: &'static str) {
        self.shader_snippets.insert(name, snippet);
    }
}

pub fn resize_window(graphics: &mut Graphics, window: &NotSendSync<kapp::Window>) {
    // There are bad assumptions here about only a single window existing.
    let main_window: &NotSendSync<kapp::Window> = window;
    let main_window: &kapp::Window = main_window;

    let (window_width, window_height) = main_window.size();

    graphics
        .context
        .resize(main_window, window_width, window_height);
}

fn check_for_dropped_graphics_assets(
    graphics: &mut Graphics,
    meshes: &mut Assets<Mesh>,
    textures: &mut Assets<Texture>,
    cube_maps: &mut Assets<CubeMap>,
) {
    meshes.drop_items(|mesh| {
        if let Some(gpu_mesh) = mesh.gpu_mesh {
            //println!("DELETING MESH!");
            let GPUMesh {
                positions,
                normals,
                index_buffer,
                texture_coordinates,
                colors,
                triangle_count: _,
            } = gpu_mesh;
            graphics.context.delete_data_buffer(positions);
            graphics.context.delete_index_buffer(index_buffer);

            if let Some(d) = normals {
                graphics.context.delete_data_buffer(d);
            }
            if let Some(d) = texture_coordinates {
                graphics.context.delete_data_buffer(d);
            }
            if let Some(d) = colors {
                graphics.context.delete_data_buffer(d);
            }
        }
    });

    textures.drop_items(|texture| graphics.context.delete_texture(texture.0));
    cube_maps.drop_items(|cube_map| graphics.context.delete_cube_map(cube_map))
}
