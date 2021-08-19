use crate::*;

pub struct PipelineBuilder<'a> {
    pub(crate) g: &'a mut GraphicsContext,
    pub(crate) vertex: Option<VertexFunction>,
    pub(crate) fragment: Option<FragmentFunction>,
    pub(crate) depth_test: DepthTest,
    pub(crate) faces_to_render: FacesToRender,
    /// Source and destination blend factors.
    pub(crate) blending: Option<(BlendFactor, BlendFactor)>,
    #[allow(unused)]
    pub(crate) output_pixel_format: PixelFormat,
    pub(crate) depth_clear_value: f32,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(graphics_context: &'a mut GraphicsContext) -> Self {
        PipelineBuilder {
            g: graphics_context,
            vertex: None,
            fragment: None,
            depth_test: crate::DepthTest::LessOrEqual,
            blending: None,
            output_pixel_format: PixelFormat::RGB8Unorm,
            faces_to_render: FacesToRender::Front,
            depth_clear_value: 1.0,
        }
    }

    pub fn faces_to_render(mut self, faces_to_render: FacesToRender) -> Self {
        self.faces_to_render = faces_to_render;
        self
    }
    pub fn depth_test(mut self, depth_test: DepthTest) -> Self {
        self.depth_test = depth_test;
        self
    }

    pub fn blending(mut self, blending: Option<(BlendFactor, BlendFactor)>) -> Self {
        self.blending = blending;
        self
    }

    /// Defaults to '1.0'
    pub fn depth_clear_value(mut self, depth_clear_value: f32) -> Self {
        self.depth_clear_value = depth_clear_value;
        self
    }
}
