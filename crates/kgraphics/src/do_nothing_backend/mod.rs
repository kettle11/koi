use crate::*;
use raw_window_handle::HasRawWindowHandle;

pub struct GraphicsContext;

#[derive(Clone)]
pub struct RenderTarget;
#[derive(Clone)]
pub struct FragmentFunction;
#[derive(Clone)]
pub struct VertexFunction;
#[derive(Clone)]
pub struct DataBuffer<T> {
    phantom: std::marker::PhantomData<T>,
}
#[derive(Clone)]
pub struct IndexBuffer;
#[derive(Clone)]
pub struct Texture;

impl Texture {
    pub fn with_mip(&self, level: u8) -> Texture {
        Texture
    }
}

#[derive(Clone)]
pub struct CommandBuffer;
#[derive(Clone)]
pub struct RenderPass<'a> {
    phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone)]
pub struct IntProperty;
#[derive(Clone)]
pub struct FloatProperty;
#[derive(Clone)]
pub struct Vec2Property;
#[derive(Clone)]
pub struct Vec3Property;
#[derive(Clone)]
pub struct Vec4Property;
#[derive(Clone)]
pub struct Mat4Property;
#[derive(Clone)]
pub struct TextureProperty;
#[derive(Clone)]
pub struct CubeMapProperty;

#[derive(Clone)]
pub struct Pipeline;

impl Pipeline {
    pub fn blending(&self) -> Option<(BlendFactor, BlendFactor)> {
        None
    }
}
    

// Presently this isn't dropped appropriately.
#[derive(Debug, Clone)]
pub struct CubeMap;

impl CubeMap {
    pub fn get_face_texture(&self, face: usize) -> Texture {
        Texture
    }
}

#[derive(Clone, PartialEq, Debug, Copy, Default)]
pub struct Framebuffer;

pub struct VertexAttribute<T> {
    phantom: std::marker::PhantomData<T>,
}

impl PipelineTrait for Pipeline {
    fn get_int_property(&self, name: &str) -> Result<IntProperty, ()> {
        Ok(IntProperty)
    }
    fn get_float_property(&self, name: &str) -> Result<FloatProperty, ()> {
        Ok(FloatProperty)
    }
    fn get_vec2_property(&self, name: &str) -> Result<Vec2Property, ()> {
        Ok(Vec2Property)
    }
    fn get_vec3_property(&self, name: &str) -> Result<Vec3Property, ()> {
        Ok(Vec3Property)
    }
    fn get_vec4_property(&self, name: &str) -> Result<Vec4Property, ()> {
        Ok(Vec4Property)
    }
    fn get_mat4_property(&self, name: &str) -> Result<Mat4Property, ()> {
        Ok(Mat4Property)
    }
    fn get_texture_property(&self, name: &str) -> Result<TextureProperty, ()> {
        Ok(TextureProperty)
    }
    fn get_cube_map_property(&self, name: &str) -> Result<CubeMapProperty, ()> {
        Ok(CubeMapProperty)
    }
    fn get_vertex_attribute<T>(&self, name: &str) -> Result<VertexAttribute<T>, String> {
        Ok(VertexAttribute::<T> {
            phantom: std::marker::PhantomData,
        })
    }
}

impl RenderPassTrait for RenderPass<'_> {
    fn set_pipeline(&mut self, pipeline: &Pipeline) {}
    fn set_vertex_attribute<T>(
        &mut self,
        vertex_attribute: &VertexAttribute<T>,
        buffer: Option<&DataBuffer<T>>,
    ) {
    }

    fn set_vertex_attribute_to_constant<T>(
        &mut self,
        vertex_attribute: &VertexAttribute<T>,
        value: &[f32],
    ) {}

    fn set_float_property(&mut self, property: &FloatProperty, value: f32) {}

    fn set_int_property(&mut self, property: &IntProperty, value: i32) {}

    fn set_vec2_property(&mut self, property: &Vec2Property, value: (f32, f32)) {}

    fn set_vec3_property(&mut self, property: &Vec3Property, value: (f32, f32, f32)) {}
    fn set_vec4_property(&mut self, property: &Vec4Property, value: (f32, f32, f32, f32)) {}
    fn set_mat4_property(&mut self, property: &Mat4Property, value: &[f32; 16]) {}

    fn set_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) {}

    fn set_texture_property(
        &mut self,
        property: &TextureProperty,
        texture: Option<&Texture>,
        texture_unit: u8,
    ) {
    }
    
    fn set_cube_map_property(
        &mut self,
        property: &CubeMapProperty,
        cube_map: Option<&CubeMap>,
        texture_unit: u8,
    ) {}

    fn set_depth_mask(&mut self, depth_mask: bool) {}
    fn blit_framebuffer(
        self,
        target: &Framebuffer,
        source_x: u32,
        source_y: u32,
        source_width: u32,
        source_height: u32,
        dest_x: u32,
        dest_y: u32,
        dest_width: u32,
        dest_height: u32,
    ) {}

    fn draw_triangles(&mut self, count: u32, buffer: &IndexBuffer) {}
    fn draw_triangles_without_buffer(&mut self, count: u32) {}
}

impl CommandBufferTrait for CommandBuffer {
    /// Gets the number of actions encoded in the `CommandBuffer`
    fn len(&self) -> usize {
        0
    }

    fn begin_render_pass_with_framebuffer<'a>(
        &'a mut self,
        framebuffer: &Framebuffer,
        clear_color: Option<(f32, f32, f32, f32)>,
    ) -> RenderPass<'a> {
        RenderPass {
            phantom: std::marker::PhantomData
        }
    }

    /// If the color_texture binds to the DefaultFramebuffer then
    /// all textures will bind to the default framebuffer.
    fn begin_render_pass<'a>(
        &'a mut self,
        color_texture: Option<&Texture>,
        depth_texture: Option<&Texture>,
        stencil_texture: Option<&Texture>,
        clear_color: Option<(f32, f32, f32, f32)>,
    ) -> RenderPass<'a> {
        RenderPass {
            phantom: std::marker::PhantomData,
        }
    }
    fn present(&mut self) {}
}
impl GraphicsContextTrait for GraphicsContext {
    fn new() -> Result<Self, ()> {
        Ok(Self)
    }
    fn new_with_settings(settings: crate::GraphicsContextSettings) -> Result<Self, ()> {
        Ok(Self)
    }

    /// This must only be called once per window.
    unsafe fn get_render_target_for_window(
        &mut self,
        window: &impl HasRawWindowHandle,
        _width: u32,
        _height: u32,
    ) -> Result<RenderTarget, ()> {
        Ok(RenderTarget)
    }

    fn resize(&mut self, window: &impl HasRawWindowHandle, width: u32, height: u32) {}
    fn new_fragment_function(&mut self, source: &str) -> Result<FragmentFunction, String> {
        Ok(FragmentFunction)
    }
    fn new_vertex_function(&mut self, source: &str) -> Result<VertexFunction, String> {
        Ok(VertexFunction)
    }

    fn new_data_buffer<T>(&mut self, data: &[T]) -> Result<DataBuffer<T>, ()> {
        Ok(DataBuffer {
            phantom: std::marker::PhantomData,
        })
    }
    fn delete_data_buffer<T>(&mut self, data_buffer: DataBuffer<T>) {}

    fn new_index_buffer(&mut self, data: &[u32]) -> Result<IndexBuffer, ()> {
        Ok(IndexBuffer)
    }
    fn delete_index_buffer(&mut self, index_buffer: IndexBuffer) {}

    fn new_texture(
        &mut self,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<Texture, ()> {
        Ok(Texture)
    }

    fn update_texture(
        &mut self,
        texture: &Texture,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
        pixel_format_in: PixelFormat,
        texture_settings: TextureSettings,
    ) {
    }

    fn delete_texture(&mut self, texture: Texture) {}

    fn new_pipeline(
        &mut self,
        vertex_function: VertexFunction,
        fragment_function: FragmentFunction,
        output_pixel_format: PixelFormat,
    ) -> PipelineBuilder {
        PipelineBuilder::new(self)
    }

    fn new_command_buffer(&mut self) -> CommandBuffer {
        CommandBuffer
    }
    fn commit_command_buffer(&mut self, command_buffer: CommandBuffer) {}

    fn generate_mip_map_for_texture(&mut self, texture: &Texture) {
       
    }

    fn new_cube_map(
        &mut self,
        width: u32,
        height: u32,
        data: Option<[&[u8]; 6]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<CubeMap, ()> {
        Ok(CubeMap)
    }

    fn update_cube_map(
        &mut self,
        cube_map: &CubeMap,
        width: u32,
        height: u32,
        data: Option<[&[u8]; 6]>,
        pixel_format_in: PixelFormat,
        texture_settings: TextureSettings,
    ) {
       
    }

    fn delete_cube_map(&mut self, cube_map: CubeMap) {
       
    }

    fn generate_mip_map_for_cube_map(&mut self, cube_map: &CubeMap) {
       
    }

    fn new_framebuffer(
        &mut self,
        color_texture: Option<&Texture>,
        depth_texture: Option<&Texture>,
        stencil_texture: Option<&Texture>,
    ) -> Framebuffer {
       Framebuffer
    }

    fn delete_framebuffer(&mut self, framebuffer: Framebuffer) {
       
    }
}

impl<'a> PipelineBuilderTrait for PipelineBuilder<'a> {
    fn build(self) -> Result<Pipeline, String> {
        Ok(Pipeline)
    }
}