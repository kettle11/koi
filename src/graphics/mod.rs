use crate::*;
use kgraphics::*;

pub use kgraphics::Pipeline;

mod camera;
pub use camera::*;

mod render_layers;
pub use render_layers::*;

mod texture;
pub use texture::*;

mod mesh;
pub use mesh::*;

mod material;
pub use material::*;

mod mesh_primitives;
pub use mesh_primitives::*;

mod renderer;
pub use renderer::*;

// Alias this type so that it's simpler to query for it.
// On other platforms it might be possible to free `Graphics` of the `NotSendSync` requirement.
pub type Graphics = NotSendSync<GraphicsInner>;

/// Stores graphics info.
pub struct GraphicsInner {
    pub context: GraphicsContext,
    // For now there's an assumption of one window.
    pub render_target: RenderTarget,
}

pub struct PipelineSettings {
    faces_to_render: FacesToRender,
    blending: Option<(BlendFactor, BlendFactor)>,
}

impl Default for PipelineSettings {
    fn default() -> Self {
        Self {
            faces_to_render: FacesToRender::Front,
            blending: None,
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
        samples: 0,
    })
    .unwrap();

    let main_window: &kapp::Window = main_window;

    let (window_width, window_height) = main_window.size();
    context.resize(main_window, window_width, window_height);

    let render_target = unsafe {
        context
            .get_render_target_for_window(main_window, window_width, window_height)
            .unwrap()
    };

    let mut graphics = NotSendSync::new(GraphicsInner {
        context,
        render_target,
    });

    let default_mesh = graphics.new_gpu_mesh(&MeshData::default()).unwrap();
    let mut mesh_assets = Assets::<Mesh>::new(Mesh {
        gpu_mesh: Some(default_mesh),
        mesh_data: Some(MeshData::default()),
    });

    initialize_static_primitives(&mut mesh_assets, &mut graphics);

    world.spawn(graphics);
    world.spawn(mesh_assets);
}

impl GraphicsInner {
    /// koi shaders are both in the same file with #VERTEX and #FRAGMENT to annotate the vertex
    /// and fragment sections.
    pub fn new_pipeline(
        &mut self,
        source: &str,
        pipeline_settings: PipelineSettings,
    ) -> Result<Pipeline, PipelineError> {
        let mut i = source.split("#VERTEX").last().unwrap().split("#FRAGMENT");
        let vertex_source = i.next().ok_or(PipelineError::MissingVertexSection)?;
        let fragment_source = i.next().ok_or(PipelineError::MissingFragmentSection)?;

        let vertex_function = self
            .context
            .new_vertex_function(vertex_source)
            .map_err(PipelineError::VertexCompilationError)?;
        let fragment_function = self
            .context
            .new_fragment_function(fragment_source)
            .map_err(PipelineError::FragmentCompilationError)?;

        let pipeline = self
            .context
            .new_pipeline(
                vertex_function,
                fragment_function,
                /* This arbitrary pixel format is a problem */ PixelFormat::RG8Unorm,
            )
            // For now all pipelines just have alpha blending by default.
            .blending(pipeline_settings.blending)
            .faces_to_render(pipeline_settings.faces_to_render)
            .build()
            .map_err(PipelineError::PipelineCompilationError)?;
        Ok(pipeline)
    }

    pub fn new_gpu_mesh(&mut self, mesh_data: &MeshData) -> Result<GPUMesh, ()> {
        // Check that all of the indices point to valid vertices.
        // If this causes performance issues this check could be disabled in the future.
        let len = mesh_data.positions.len();
        for i in mesh_data.indices.iter() {
            if i[0] as usize > len || i[1] as usize > len || i[2] as usize > len {
                panic!(
                    "Mesh indices refer to out of bound vertices: {:?}. Vertex count: {:?}",
                    i,
                    mesh_data.positions.len()
                );
            }
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
            Some(
                self.context
                    .new_data_buffer(&mesh_data.texture_coordinates)?,
            )
        } else {
            None
        };
        let normals = if !mesh_data.normals.is_empty() {
            Some(self.context.new_data_buffer(&mesh_data.normals)?)
        } else {
            None
        };

        let colors = if !mesh_data.colors.is_empty() {
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
}

pub fn graphics_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_graphics.system()],
        draw_systems: vec![resize_window.system()],
        ..Default::default()
    }
}

pub fn resize_window(graphics: &mut Graphics, window: &NotSendSync<kapp::Window>) {
    // There are bad assumptions here about only a single window existing.
    let main_window: &NotSendSync<kapp::Window> = &window;
    let main_window: &kapp::Window = main_window;

    let (window_width, window_height) = main_window.size();

    graphics
        .context
        .resize(main_window, window_width, window_height);
}
