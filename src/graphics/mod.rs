use crate::*;
use kgraphics::*;

pub use kgraphics::Pipeline;

mod camera;
pub use camera::*;

mod render_layers;
pub use render_layers::*;

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

#[derive(Clone, Debug, Component)]
pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub indices: Vec<[u32; 3]>,
    pub normals: Vec<Vec3>,
    pub texture_coordinates: Vec<Vec2>,
    pub colors: Vec<Vec4>,
}

#[derive(Clone)]
pub struct GPUMesh {
    pub positions: DataBuffer<Vec3>,
    pub texture_coordinates: Option<DataBuffer<Vec2>>,
    pub normals: Option<DataBuffer<Vec3>>,
    pub index_buffer: IndexBuffer,
    pub triangle_count: u32,
    pub colors: Option<DataBuffer<Vec4>>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            positions: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            texture_coordinates: Vec::new(),
            colors: Vec::new(),
        }
    }
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

    pub fn new_gpu_mesh(&mut self, mesh: &Mesh) -> Result<GPUMesh, ()> {
        // Check that all of the indices point to valid vertices.
        // If this causes performance issues this check could be disabled in the future.
        let len = mesh.positions.len();
        for i in mesh.indices.iter() {
            if i[0] as usize > len || i[1] as usize > len || i[2] as usize > len {
                panic!(
                    "Indices refer to out of bound vertices: {:?}. Vertex count: {:?}",
                    i,
                    mesh.positions.len()
                );
                //return Err(());
            }
        }

        let triangle_count = mesh.indices.len() as u32;

        // Flatten the index buffer
        let index_buffer: &[u32] = unsafe {
            std::slice::from_raw_parts(mesh.indices.as_ptr() as *const u32, mesh.indices.len() * 3)
        };

        let texture_coordinates = if !mesh.texture_coordinates.is_empty() {
            Some(self.context.new_data_buffer(&mesh.texture_coordinates)?)
        } else {
            None
        };
        let normals = if !mesh.normals.is_empty() {
            Some(self.context.new_data_buffer(&mesh.normals)?)
        } else {
            None
        };

        let colors = if !mesh.colors.is_empty() {
            Some(self.context.new_data_buffer(&mesh.colors)?)
        } else {
            None
        };

        Ok(GPUMesh {
            positions: self.context.new_data_buffer(&mesh.positions)?,
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

    let graphics = GraphicsInner {
        context,
        render_target,
    };

    world.spawn(NotSendSync::new(graphics));
}

/*
pub fn attach_gpu_mesh(graphics: &mut Graphics, meshes: Query<&Mesh, Without<Handle<GPUMesh>>>) {
}
*/

pub fn resize_window(graphics: &mut Graphics, window: &NotSendSync<kapp::Window>) {
    // There are bad assumptions here about only a single window existing.
    let main_window: &NotSendSync<kapp::Window> = &window;
    let main_window: &kapp::Window = main_window;

    let (window_width, window_height) = main_window.size();

    graphics
        .context
        .resize(main_window, window_width, window_height);
}
