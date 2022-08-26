use crate::*;
use raw_window_handle::*;

pub trait RenderTargetTrait {
    fn pixel_format(&self) -> PixelFormat;
    fn current_frame(&self) -> Texture;
}

#[derive(Debug, Clone, Copy)]
pub enum PropertyError {
    IncorrectType,
}

#[derive(Debug, Clone, Copy)]
pub enum GraphicsError {}

pub trait PipelineTrait {
    fn get_int_property(&self, name: &str) -> Result<IntProperty, PropertyError>;
    fn get_float_property(&self, name: &str) -> Result<FloatProperty, PropertyError>;
    fn get_vec2_property(&self, name: &str) -> Result<Vec2Property, PropertyError>;
    fn get_vec3_property(&self, name: &str) -> Result<Vec3Property, PropertyError>;
    fn get_vec4_property(&self, name: &str) -> Result<Vec4Property, PropertyError>;
    fn get_mat4_property(&self, name: &str) -> Result<Mat4Property, PropertyError>;
    fn get_texture_property(&self, name: &str) -> Result<TextureProperty, PropertyError>;
    fn get_cube_map_property(&self, name: &str) -> Result<CubeMapProperty, PropertyError>;
    fn get_uniform_block<T>(&self, name: &str) -> Result<UniformBlock<T>, String>;
    fn get_vertex_attribute<T>(&self, name: &str) -> Result<VertexAttribute<T>, String>;
}

pub trait PipelineBuilderTrait {
    fn build(self) -> Result<Pipeline, String>;
}

pub trait RenderPassTrait {
    fn set_pipeline(&mut self, pipeline: &Pipeline);
    /// Vertex attributes are arrays of data for each vertex.
    fn set_vertex_attribute<T>(
        &mut self,
        attribute: &VertexAttribute<T>,
        buffer: Option<&DataBuffer<T>>,
    );
    fn set_instance_attribute<T>(
        &mut self,
        vertex_attribute: &VertexAttribute<T>,
        buffer: Option<&DataBuffer<T>>,
    );
    fn set_vertex_attribute_to_constant<T>(
        &mut self,
        vertex_attribute: &VertexAttribute<T>,
        value: &[f32],
    );
    fn set_uniform_block<T>(
        &mut self,
        uniform_block: &UniformBlock<T>,
        buffer: Option<&DataBuffer<T>>,
    );
    fn set_float_property(&mut self, property: &FloatProperty, value: f32);

    fn set_int_property(&mut self, property: &IntProperty, value: i32);

    fn set_vec2_property(&mut self, property: &Vec2Property, value: (f32, f32));

    fn set_vec3_property(&mut self, property: &Vec3Property, value: (f32, f32, f32));
    fn set_vec4_property(&mut self, property: &Vec4Property, value: (f32, f32, f32, f32));

    fn set_mat4_property(&mut self, property: &Mat4Property, value: &[f32; 16]);

    fn set_viewport(&mut self, x: u32, y: u32, width: u32, height: u32);

    /// The texture unit should be 0 to 16
    /// Perhaps that restriction should be waved later after research.
    fn set_texture_property(
        &mut self,
        property: &TextureProperty,
        texture: Option<&Texture>,
        texture_unit: u8,
    );

    fn set_cube_map_property(
        &mut self,
        property: &CubeMapProperty,
        cube_map: Option<&CubeMap>,
        texture_unit: u8,
    );

    fn draw_triangles(&mut self, vertex_count: u32, index_buffer: &IndexBuffer);
    fn draw_triangles_without_buffer(&mut self, vertex_count: u32);
    fn draw_triangles_instanced(
        &mut self,
        vertex_count: u32,
        index_buffer: &IndexBuffer,
        instances: u32,
    );

    fn set_depth_mask(&mut self, depth_mask: bool);

    #[allow(clippy::too_many_arguments)]
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
    );
}

pub trait CommandBufferTrait {
    /// Gets the number of actions encoded in the `CommandBuffer`
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn begin_render_pass_with_framebuffer<'a>(
        &'a mut self,
        framebuffer: &Framebuffer,
        clear_color: Option<(f32, f32, f32, f32)>,
    ) -> RenderPass<'a>;

    /// If the color_texture binds to the DefaultFramebuffer then
    /// all textures will bind to the default framebuffer.
    fn begin_render_pass<'a>(
        &'a mut self,
        color_texture: Option<&Texture>,
        depth_texture: Option<&Texture>,
        stencil_texture: Option<&Texture>,
        clear_color: Option<(f32, f32, f32, f32)>,
    ) -> RenderPass<'a>;
    fn present(&mut self);
}

pub trait GraphicsContextTrait: Sized {
    fn new() -> Self;
    fn new_with_settings(settings: crate::GraphicsContextSettings) -> Self;

    /// This must only be called once per window.
    fn get_render_target_for_window(
        &mut self,
        window: &impl HasRawWindowHandle,
        _width: u32,
        _height: u32,
    ) -> RenderTarget;

    // A hack to accomodate passing an SDL window around
    #[cfg(feature = "SDL")]
    unsafe fn get_render_target_for_window_sdl(
        &mut self,
        window: kapp::WindowId,
        _width: u32,
        _height: u32,
    ) -> Result<RenderTarget, ()>;

    fn resize(&mut self, window: &impl HasRawWindowHandle, width: u32, height: u32);
    fn new_fragment_function(&mut self, source: &str) -> Result<FragmentFunction, String>;
    fn new_vertex_function(&mut self, source: &str) -> Result<VertexFunction, String>;

    fn new_data_buffer<T>(&mut self, data: &[T]) -> Result<DataBuffer<T>, GraphicsError>;
    fn delete_data_buffer<T>(&mut self, data_buffer: DataBuffer<T>);

    fn new_index_buffer(&mut self, data: &[u32]) -> Result<IndexBuffer, GraphicsError>;
    fn delete_index_buffer(&mut self, index_buffer: IndexBuffer);

    fn new_texture(
        &mut self,
        width: u32,
        height: u32,
        depth: u32,
        data: Option<&[u8]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<Texture, GraphicsError>;

    #[allow(clippy::too_many_arguments)]
    fn update_texture(
        &mut self,
        texture: &Texture,
        x: u32,
        y: u32,
        z: u32,
        width: u32,
        height: u32,
        depth: u32,
        data: Option<&[u8]>,
        pixel_format_in: PixelFormat,
        texture_settings: TextureSettings,
    );

    fn delete_texture(&mut self, texture: Texture);

    fn read_texture(&mut self, texture: &Texture, format: PixelFormat, size: usize) -> Vec<u8>;

    fn generate_mip_map_for_texture(&mut self, texture: &Texture);

    fn new_cube_map(
        &mut self,
        width: u32,
        height: u32,
        data: Option<[&[u8]; 6]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<CubeMap, GraphicsError>;

    fn update_cube_map(
        &mut self,
        cube_map: &CubeMap,
        width: u32,
        height: u32,
        data: Option<[&[u8]; 6]>,
        pixel_format_in: PixelFormat,
        texture_settings: TextureSettings,
    );

    fn delete_cube_map(&mut self, cube_map: CubeMap);

    fn generate_mip_map_for_cube_map(&mut self, cube_map: &CubeMap);

    fn new_pipeline(
        &mut self,
        vertex_function: VertexFunction,
        fragment_function: FragmentFunction,
        output_pixel_format: PixelFormat,
    ) -> PipelineBuilder;

    fn new_command_buffer(&mut self) -> CommandBuffer;
    fn commit_command_buffer(&mut self, command_buffer: CommandBuffer);
    fn new_framebuffer(
        &mut self,
        color_texture: Option<&Texture>,
        depth_texture: Option<&Texture>,
        stencil_texture: Option<&Texture>,
    ) -> Framebuffer;
    fn delete_framebuffer(&mut self, framebuffer: Framebuffer);
    fn get_multiview_supported(&self) -> MultiviewSupport {
        MultiviewSupport::None
    }
}
