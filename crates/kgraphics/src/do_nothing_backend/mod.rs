use crate::*;
use raw_window_handle::HasRawWindowHandle;

pub struct GraphicsContext;

pub struct RenderTarget;
pub struct FragmentFunction;
pub struct VertexFunction;
pub struct DataBuffer<T> {
    phantom: std::marker::PhantomData<T>,
}
pub struct IndexBuffer;
pub struct Texture;
pub struct CommandBuffer;
pub struct RenderPass<'a> {
    phantom: std::marker::PhantomData<&'a ()>,
}

pub struct IntProperty;
pub struct FloatProperty;
pub struct Vec2Property;
pub struct Vec3Property;
pub struct Vec4Property;
pub struct Mat4Property;
pub struct TextureProperty;

pub struct Pipeline;

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

    fn draw_triangles(&mut self, count: u32, buffer: Option<&IndexBuffer>) {}
}

impl CommandBufferTrait for CommandBuffer {
    fn clear(&mut self) {}

    /// Gets the number of actions encoded in the `CommandBuffer`
    fn len(&self) -> usize {
        0
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
    fn new_fragment_function(&self, source: &str) -> Result<FragmentFunction, String> {
        Ok(FragmentFunction)
    }
    fn new_vertex_function(&self, source: &str) -> Result<VertexFunction, String> {
        Ok(VertexFunction)
    }

    fn new_data_buffer<T>(&self, data: &[T]) -> Result<DataBuffer<T>, ()> {
        Ok(DataBuffer {
            phantom: std::marker::PhantomData,
        })
    }
    fn delete_data_buffer<T>(&self, data_buffer: DataBuffer<T>) {}

    fn new_index_buffer(&self, data: &[u32]) -> Result<IndexBuffer, ()> {
        Ok(IndexBuffer)
    }
    fn delete_index_buffer(&self, index_buffer: IndexBuffer) {}

    fn new_texture(
        &self,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
        pixel_format: PixelFormat,
        srgb: bool,
    ) -> Result<Texture, ()> {
        Ok(Texture)
    }

    fn update_texture(
        &self,
        texture: &Texture,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
        pixel_format_in: PixelFormat,
        srgb: bool,
    ) {
    }

    fn delete_texture(&self, texture: Texture) {}

    fn new_pipeline(
        &mut self,
        vertex_function: VertexFunction,
        fragment_function: FragmentFunction,
        output_pixel_format: PixelFormat,
    ) -> PipelineBuilder {
        PipelineBuilder::new(self)
    }

    fn new_command_buffer(&mut self) -> CommandBuffer {}
    fn commit_command_buffer(&mut self, command_buffer: CommandBuffer) {}
}
