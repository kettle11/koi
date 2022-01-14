use crate::*;

mod command_buffer;
pub use command_buffer::*;

mod gl_native;
use gl_native::*;

// Included for GLContext
use crate::{BlendFactor, DepthTest, FacesToRender, PixelFormat};
use kapp::*;
use raw_window_handle::*;
use std::collections::HashMap;

mod bump_allocator;
use bump_allocator::*;

pub use gl::gl_native::Framebuffer;
pub struct GraphicsContext {
    old_command_buffers: Vec<CommandBuffer>,
    gl_context: GLContext,
    gl: gl_native::GL,
}
pub struct VertexFunction {
    shader: gl_native::Shader,
}

pub struct FragmentFunction {
    shader: gl_native::Shader,
}

#[derive(Clone)]
pub struct Pipeline {
    program: gl_native::Program,
    vertex_attributes: HashMap<String, VertexAttributeInfo>,
    uniforms: HashMap<String, Uniform>,
    depth_test: DepthTest,
    faces_to_render: FacesToRender,
    blending: Option<(BlendFactor, BlendFactor)>,
    //depth_clear_value: f32,
}

impl Pipeline {
    pub fn blending(&self) -> Option<(BlendFactor, BlendFactor)> {
        self.blending
    }
}

/// OpenGL doesn't handle multiple render targets correctly.
pub struct RenderTarget {
    pixel_format: PixelFormat,
}

impl RenderTargetTrait for RenderTarget {
    fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }

    fn current_frame(&self) -> Result<Texture, ()> {
        Ok(Texture {
            texture_type: TextureType::DefaultFramebuffer,
            mip: 0,
        })
    }
}

#[derive(Clone)]
pub struct DataBuffer<T> {
    buffer: gl_native::Buffer,
    phantom: std::marker::PhantomData<T>,
}

#[derive(Clone)]
pub struct IndexBuffer {
    buffer: gl_native::Buffer,
}

#[derive(Debug, Clone)]
enum TextureType {
    Texture(gl_native::TextureNative),
    RenderBuffer(RenderBufferNative),
    CubeMap {
        face: u8,
        texture_native: gl_native::TextureNative,
    },
    DefaultFramebuffer,
}

#[derive(Debug)]
pub struct Texture {
    texture_type: TextureType,
    mip: u8,
}

impl Texture {
    pub fn with_mip(&self, level: u8) -> Texture {
        Texture {
            texture_type: self.texture_type.clone(),
            mip: level,
        }
    }
}

// Presently this isn't dropped appropriately.
#[derive(Debug, Clone)]
pub struct CubeMap {
    texture: gl_native::TextureNative,
}

impl CubeMap {
    pub fn get_face_texture(&self, face: usize) -> Texture {
        assert!(face < 6);
        Texture {
            texture_type: TextureType::CubeMap {
                face: face as u8,
                texture_native: self.texture,
            },
            mip: 0,
        }
    }
}

#[derive(Clone)]
struct Uniform {
    uniform_type: u32,
    // Is this actually the size in bytes?
    // size_bytes: i32,
    location: gl_native::UniformLocation,
}

#[derive(Clone, Debug)]
struct VertexAttributeInfo {
    // attribute_type: u32, // A GL num
    byte_size: u32,
    index: u32,
}

#[derive(Clone)]
pub struct VertexAttribute<T> {
    info: Option<VertexAttributeInfo>,
    phantom: std::marker::PhantomData<T>,
}

#[derive(Clone, PartialEq)]
pub struct FloatProperty {
    location: Option<gl_native::UniformLocation>,
}

impl FloatProperty {
    pub fn exists(&self) -> bool {
        self.location.is_some()
    }
}

#[derive(Clone, PartialEq)]
pub struct IntProperty {
    location: Option<gl_native::UniformLocation>,
}

impl IntProperty {
    pub fn exists(&self) -> bool {
        self.location.is_some()
    }
}

#[derive(Clone, PartialEq)]
pub struct Vec2Property {
    location: Option<gl_native::UniformLocation>,
}

impl Vec2Property {
    pub fn exists(&self) -> bool {
        self.location.is_some()
    }
}

#[derive(Clone, PartialEq)]
pub struct Vec3Property {
    location: Option<gl_native::UniformLocation>,
}

impl Vec3Property {
    pub fn exists(&self) -> bool {
        self.location.is_some()
    }
}

#[derive(Clone, PartialEq)]
pub struct Vec4Property {
    location: Option<gl_native::UniformLocation>,
}

impl Vec4Property {
    pub fn exists(&self) -> bool {
        self.location.is_some()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextureProperty {
    location: Option<gl_native::UniformLocation>,
}

impl TextureProperty {
    pub fn exists(&self) -> bool {
        self.location.is_some()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CubeMapProperty {
    location: Option<gl_native::UniformLocation>,
}

impl CubeMapProperty {
    pub fn exists(&self) -> bool {
        self.location.is_some()
    }
}

#[derive(Clone, PartialEq)]
pub struct Mat4Property {
    location: Option<gl_native::UniformLocation>,
}

impl Mat4Property {
    pub fn exists(&self) -> bool {
        self.location.is_some()
    }
}

impl Pipeline {
    fn get_property(
        &self,
        name: &str,
        type_: GLenum,
    ) -> Result<Option<gl_native::UniformLocation>, ()> {
        if let Some(uniform) = self.uniforms.get(name) {
            if uniform.uniform_type == type_.0 {
                Ok(Some(uniform.location))
            } else {
                Err(())
            }
        } else {
            Ok(None)
        }
    }
}

impl PipelineTrait for Pipeline {
    fn get_int_property(&self, name: &str) -> Result<IntProperty, ()> {
        Ok(IntProperty {
            location: self.get_property(name, GL_INT)?,
        })
    }

    fn get_float_property(&self, name: &str) -> Result<FloatProperty, ()> {
        Ok(FloatProperty {
            location: self.get_property(name, GL_FLOAT)?,
        })
    }

    fn get_vec2_property(&self, name: &str) -> Result<Vec2Property, ()> {
        Ok(Vec2Property {
            location: self.get_property(name, GL_FLOAT_VEC2)?,
        })
    }

    fn get_vec3_property(&self, name: &str) -> Result<Vec3Property, ()> {
        Ok(Vec3Property {
            location: self.get_property(name, GL_FLOAT_VEC3)?,
        })
    }

    fn get_vec4_property(&self, name: &str) -> Result<Vec4Property, ()> {
        Ok(Vec4Property {
            location: self.get_property(name, GL_FLOAT_VEC4)?,
        })
    }

    fn get_mat4_property(&self, name: &str) -> Result<Mat4Property, ()> {
        Ok(Mat4Property {
            location: self.get_property(name, GL_FLOAT_MAT4)?,
        })
    }

    fn get_texture_property(&self, name: &str) -> Result<TextureProperty, ()> {
        Ok(TextureProperty {
            location: self.get_property(name, GL_SAMPLER_2D)?,
        })
    }

    fn get_cube_map_property(&self, name: &str) -> Result<CubeMapProperty, ()> {
        Ok(CubeMapProperty {
            location: self.get_property(name, GL_SAMPLER_CUBE)?,
        })
    }

    fn get_vertex_attribute<T>(&self, name: &str) -> Result<VertexAttribute<T>, String> {
        if let Some(attribute) = self.vertex_attributes.get(name) {
            if attribute.byte_size == std::mem::size_of::<T>() as u32 {
                Ok(VertexAttribute {
                    info: Some(attribute.clone()),
                    phantom: std::marker::PhantomData,
                })
            } else {
                Err(format!(
                    "Vertex attribute size mismatch for {:?}. /n Shader: {:?}, Rust: {:?}",
                    name,
                    attribute.byte_size,
                    std::mem::size_of::<T>()
                ))
            }
        } else {
            Ok(VertexAttribute {
                info: None,
                phantom: std::marker::PhantomData,
            })
        }
    }
}

use crate::pipeline_builder::*;

impl<'a> PipelineBuilderTrait for PipelineBuilder<'a> {
    fn build(self) -> Result<Pipeline, String> {
        let program = self.g.new_program(
            self.vertex.as_ref().unwrap(),
            self.fragment.as_ref().unwrap(),
        )?;

        /*
        let mut stride = 0;
        for attribute in &self.vertex_attributes {
            stride += attribute.components_count * std::mem::size_of::<f32>();
        }
        */

        let mut uniforms = HashMap::new();
        let mut vertex_attributes = HashMap::new();
        unsafe {
            let uniform_count = self.g.gl.get_active_uniforms(program);
            for i in 0..uniform_count {
                let uniform = self.g.gl.get_active_uniform(program, i).unwrap();
                let uniform_location = self.g.gl.get_uniform_location(program, &uniform.name);

                uniforms.insert(
                    uniform.name,
                    Uniform {
                        uniform_type: uniform.uniform_type.0,
                        // size_bytes: uniform.size_members,
                        location: uniform_location.unwrap(),
                    },
                );
            }

            let vertex_attribute_count = self.g.gl.get_active_attributes(program);
            for i in 0..vertex_attribute_count {
                let attribute = self.g.gl.get_active_attribute(program, i).unwrap();

                // Notably the attribute location index is *not* the index passed into `GetActiveAttrib`
                let attribute_location = self.g.gl.get_attribute_location(program, &attribute.name);
                let byte_size = match attribute.attribute_type {
                    GL_FLOAT => 4,
                    GL_FLOAT_VEC2 => 8,
                    GL_FLOAT_VEC3 => 12,
                    GL_FLOAT_VEC4 => 16,
                    GL_FLOAT_MAT4 => 64,
                    _ => continue,
                };

                vertex_attributes.insert(
                    attribute.name,
                    VertexAttributeInfo {
                        // attribute_type: attribute.attribute_type.0,
                        byte_size,
                        index: attribute_location as u32,
                    },
                );
            }
        }

        Ok(Pipeline {
            program,
            vertex_attributes,
            uniforms,
            depth_test: self.depth_test,
            faces_to_render: self.faces_to_render,
            blending: self.blending,
            // depth_clear_value: self.depth_clear_value,
        })
    }
}

impl GraphicsContextTrait for GraphicsContext {
    fn new_with_settings(settings: crate::GraphicsContextSettings) -> Result<Self, ()> {
        unsafe {
            let mut gl_context_builder = GLContext::new();
            gl_context_builder.high_resolution_framebuffer(settings.high_resolution_framebuffer);
            gl_context_builder.samples(settings.samples);

            #[cfg(target_arch = "wasm32")]
            gl_context_builder.webgl2();

            let gl_context = gl_context_builder.build().unwrap();

            #[cfg(target_arch = "wasm32")]
            let gl: GL = GL::new(); //glow::Context::from_webgl2_context(gl_context.webgl2_context().unwrap());

            #[cfg(not(target_arch = "wasm32"))]
            let gl: GL = GL {
                gl: GlFns::load_from(&|s| {
                    let s = std::ffi::CStr::from_ptr(s as *const i8);
                    gl_context.get_proc_address(s.to_str().unwrap())
                })
                .unwrap(),
            };

            // A vertex array object must be bound for OpenGL 4+
            // But is it a problem that everything is binding into this object?
            // Should this object just be bound and unbound as required?
            // A random comment on the internet indicates that it may be faster to not use VAOs
            // if most meshes share the exact same layout. There are fewer calls of overhead.
            let vertex_array_object = gl.create_vertex_array().unwrap();

            gl.bind_vertex_array(Some(vertex_array_object));

            gl.enable(GL_TEXTURE_CUBE_MAP_SEAMLESS);

            Ok(GraphicsContext {
                gl_context,
                gl,
                old_command_buffers: Vec::new(),
            })
        }
    }

    fn new() -> Result<Self, ()> {
        Self::new_with_settings(crate::GraphicsContextSettings {
            high_resolution_framebuffer: false,
            samples: 0,
        })
    }

    /// This must only be called once per window.
    unsafe fn get_render_target_for_window(
        &mut self,
        window: &impl HasRawWindowHandle,
        _width: u32,
        _height: u32,
    ) -> Result<RenderTarget, ()> {
        self.gl_context.set_window(Some(window)).unwrap();

        // This is obviously incorrect. This should be fixed.
        let pixel_format = PixelFormat::RGBA8Unorm;

        Ok(RenderTarget { pixel_format })
    }

    #[cfg(feature = "SDL")]
    /// This must only be called once per window.
    unsafe fn get_render_target_for_window_sdl(
        &mut self,
        window: kapp::WindowId,
        _width: u32,
        _height: u32,
    ) -> Result<RenderTarget, ()> {
        self.gl_context.set_window_with_window_id(window).unwrap();
        // This is obviously incorrect. This should be fixed.
        let pixel_format = PixelFormat::RGBA8Unorm;
        Ok(RenderTarget { pixel_format })
    }

    fn resize(&mut self, _window: &impl HasRawWindowHandle, width: u32, height: u32) {
        self.gl_context.resize();
        unsafe {
            // This perhaps incorrectly assumes there's only one viewport.
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    fn new_fragment_function(&mut self, source: &str) -> Result<FragmentFunction, String> {
        Ok(FragmentFunction {
            shader: self.compile_shader(GL_FRAGMENT_SHADER, source)?,
        })
    }

    fn new_vertex_function(&mut self, source: &str) -> Result<VertexFunction, String> {
        Ok(VertexFunction {
            shader: self.compile_shader(GL_VERTEX_SHADER, source)?,
        })
    }

    fn new_data_buffer<T>(&mut self, data: &[T]) -> Result<DataBuffer<T>, ()> {
        unsafe {
            let buffer = self.gl.create_buffer().unwrap();
            self.gl.bind_buffer(GL_ARRAY_BUFFER, Some(buffer));
            self.gl
                .buffer_data_u8_slice(GL_ARRAY_BUFFER.0, slice_to_bytes(data), GL_STATIC_DRAW.0);
            Ok(DataBuffer {
                buffer,
                phantom: std::marker::PhantomData,
            })
        }
    }

    fn delete_data_buffer<T>(&mut self, data_buffer: DataBuffer<T>) {
        unsafe { self.gl.delete_buffer(data_buffer.buffer) }
    }

    fn new_index_buffer(&mut self, data: &[u32]) -> Result<IndexBuffer, ()> {
        unsafe {
            let buffer = self.gl.create_buffer().unwrap();
            self.gl.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, Some(buffer));
            self.gl.buffer_data_u8_slice(
                GL_ELEMENT_ARRAY_BUFFER.0,
                slice_to_bytes(data),
                GL_STATIC_DRAW.0,
            );
            Ok(IndexBuffer { buffer })
        }
    }

    fn delete_index_buffer(&mut self, index_buffer: IndexBuffer) {
        unsafe { self.gl.delete_buffer(index_buffer.buffer) }
    }

    fn new_texture(
        &mut self,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<Texture, ()> {
        unsafe {
            if texture_settings.msaa_samples == 0 {
                let texture = self.gl.create_texture().unwrap();
                let texture = Texture {
                    texture_type: TextureType::Texture(texture),
                    mip: 0,
                };
                self.update_texture(
                    &texture,
                    width,
                    height,
                    data,
                    pixel_format,
                    texture_settings,
                );
                Ok(texture)
            } else {
                let renderbuffer = self.gl.create_renderbuffer();

                let (_pixel_format, inner_pixel_format, _type_) =
                    crate::gl_shared::pixel_format_to_gl_format_and_inner_format_and_type(
                        pixel_format,
                        texture_settings.srgb,
                    );
                self.gl.renderbuffer_storage_multisample(
                    GL_RENDERBUFFER,
                    texture_settings.msaa_samples as i32,
                    inner_pixel_format as i32,
                    width as i32,
                    height as i32,
                );

                Ok(Texture {
                    texture_type: TextureType::RenderBuffer(renderbuffer),
                    mip: 0,
                })
            }
        }
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
        let (target, texture) = match texture.texture_type {
            TextureType::Texture(t) => (GL_TEXTURE_2D, t),
            TextureType::CubeMap {
                face,
                texture_native,
            } => (
                GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + face as u32),
                texture_native,
            ),
            TextureType::RenderBuffer { .. } => {
                panic!("For now textures with MSAA cannot be updated by a call to `update_texture`")
            }
            TextureType::DefaultFramebuffer => panic!("Cannot update default framebuffer"),
        };
        unsafe {
            // Convert data to linear instead of sRGB if needed and flip the image vertically.
            /* let data = prepare_image(
                pixel_format_in,
                texture_settings.srgb,
                width as usize,
                height as usize,
                data,
            );*/

            let (pixel_format, inner_pixel_format, type_) =
                crate::gl_shared::pixel_format_to_gl_format_and_inner_format_and_type(
                    pixel_format_in,
                    texture_settings.srgb,
                );

            self.gl.bind_texture(target, Some(texture));
            self.gl.tex_image_2d(
                target,
                0,                         /* mip level */
                inner_pixel_format as i32, // Internal format, how the GPU stores these pixels.
                width as i32,
                height as i32,
                0,                    /* border: must be 0 */
                GLenum(pixel_format), // This doesn't necessarily need to match the internal_format
                GLenum(type_),
                data,
            );

            let minification_filter = minification_filter_to_gl_enum(
                texture_settings.minification_filter,
                texture_settings.mipmap_filter,
                texture_settings.generate_mipmaps,
            );
            let magnification_filter =
                magnification_filter_to_gl_enum(texture_settings.magnification_filter);

            self.gl
                .tex_parameter_i32(target, GL_TEXTURE_MIN_FILTER, minification_filter as i32);

            self.gl
                .tex_parameter_i32(target, GL_TEXTURE_MAG_FILTER, magnification_filter as i32);

            let wrapping_horizontal = wrapping_to_gl_enum(texture_settings.wrapping_horizontal);
            let wrapping_vertical = wrapping_to_gl_enum(texture_settings.wrapping_vertical);

            self.gl
                .tex_parameter_i32(target, GL_TEXTURE_WRAP_S, wrapping_horizontal as i32);
            self.gl
                .tex_parameter_i32(target, GL_TEXTURE_WRAP_T, wrapping_vertical as i32);

            if texture_settings.generate_mipmaps {
                self.gl.generate_mipmap(target);
            }
        }
    }

    fn delete_texture(&mut self, texture: Texture) {
        unsafe {
            match texture.texture_type {
                TextureType::Texture(t) => self.gl.delete_texture(t),
                TextureType::CubeMap { .. } => return,
                TextureType::RenderBuffer(renderbuffer) => {
                    self.gl.delete_renderbuffer(renderbuffer);
                }
                TextureType::DefaultFramebuffer => panic!("Cannot delete default framebuffer"),
            };
        }
    }

    fn generate_mip_map_for_texture(&mut self, texture: &Texture) {
        let (target, texture) = match texture.texture_type {
            TextureType::Texture(t) => (GL_TEXTURE_2D, t),
            TextureType::CubeMap {
                face,
                texture_native,
            } => (
                GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + face as u32),
                texture_native,
            ),
            TextureType::RenderBuffer { .. } => {
                panic!("Cannot generate mipmaps for textures with MSAA samples")
            }
            TextureType::DefaultFramebuffer => {
                panic!("Cannot default mipmaps for default framebuffer")
            }
        };
        unsafe {
            self.gl.bind_texture(target, Some(texture));
            self.gl.generate_mipmap(target);
        }
    }

    fn new_cube_map(
        &mut self,
        width: u32,
        height: u32,
        data: Option<[&[u8]; 6]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<CubeMap, ()> {
        unsafe {
            let texture = self.gl.create_texture().unwrap();
            let cube_map = CubeMap { texture };
            self.update_cube_map(
                &cube_map,
                width,
                height,
                data,
                pixel_format,
                texture_settings,
            );
            Ok(cube_map)
        }
    }

    fn update_cube_map(
        &mut self,
        cube_map: &CubeMap,
        width: u32,
        height: u32,
        data: Option<[&[u8]; 6]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) {
        let (pixel_format, inner_pixel_format, type_) =
            crate::gl_shared::pixel_format_to_gl_format_and_inner_format_and_type(
                pixel_format,
                texture_settings.srgb,
            );
        unsafe {
            self.gl
                .bind_texture(GL_TEXTURE_CUBE_MAP, Some(cube_map.texture));
            for i in 0..6 {
                self.gl.tex_image_2d(
                    GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + i as u32),
                    0,                         /* mip level */
                    inner_pixel_format as i32, // Internal format, how the GPU stores these pixels.
                    width as i32,
                    height as i32,
                    0,                    /* border: must be 0 */
                    GLenum(pixel_format), // This doesn't necessarily need to match the internal_format
                    GLenum(type_),
                    data.map(|d| d[i]),
                );
            }

            let minification_filter = minification_filter_to_gl_enum(
                texture_settings.minification_filter,
                texture_settings.mipmap_filter,
                texture_settings.generate_mipmaps,
            );
            let magnification_filter =
                magnification_filter_to_gl_enum(texture_settings.magnification_filter);

            self.gl.tex_parameter_i32(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_MIN_FILTER,
                minification_filter as i32,
            );
            self.gl.tex_parameter_i32(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_MAG_FILTER,
                magnification_filter as i32,
            );

            let wrapping_horizontal = wrapping_to_gl_enum(texture_settings.wrapping_horizontal);
            let wrapping_vertical = wrapping_to_gl_enum(texture_settings.wrapping_vertical);

            self.gl.tex_parameter_i32(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_WRAP_S,
                wrapping_horizontal as i32,
            );
            self.gl.tex_parameter_i32(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_WRAP_T,
                wrapping_vertical as i32,
            );

            if texture_settings.generate_mipmaps {
                self.gl.generate_mipmap(GL_TEXTURE_CUBE_MAP);
            }
        }
    }

    fn delete_cube_map(&mut self, cube_map: CubeMap) {
        unsafe { self.gl.delete_texture(cube_map.texture) }
    }

    fn generate_mip_map_for_cube_map(&mut self, texture: &CubeMap) {
        unsafe {
            self.gl
                .bind_texture(GL_TEXTURE_CUBE_MAP, Some(texture.texture));
            self.gl.generate_mipmap(GL_TEXTURE_CUBE_MAP);
        }
    }

    fn new_command_buffer(&mut self) -> CommandBuffer {
        self.old_command_buffers
            .pop()
            .unwrap_or_else(CommandBuffer::new)
    }

    fn commit_command_buffer(&mut self, mut command_buffer: CommandBuffer) {
        // let mut temp_framebuffer = None;
        unsafe {
            use CommandBufferAction::*;
            for command in command_buffer.actions.iter().cloned() {
                match command {
                    Clear((r, g, b, a)) => {
                        self.gl.clear_color(r, g, b, a);
                        self.gl.clear(GL_COLOR_BUFFER_BIT.0 | GL_DEPTH_BUFFER_BIT.0);
                    }
                    BindFramebuffer(framebuffer) => {
                        self.gl.bind_framebuffer(GL_FRAMEBUFFER, framebuffer);
                    }
                    ChangePipeline(pipeline) => {
                        // Requiring a clone of the pipeline all over the place is not good.
                        self.gl.use_program(Some(pipeline.program));

                        // GL_ALWAYS will still write to the depth buffer, just the value is ignored.
                        // So depth testing is enabled even for always.
                        self.gl.enable(GL_DEPTH_TEST);

                        match pipeline.depth_test {
                            crate::DepthTest::AlwaysPass => {
                                self.gl.depth_func(GL_ALWAYS);
                            }
                            crate::DepthTest::Less => {
                                self.gl.depth_func(GL_LESS);
                            }
                            crate::DepthTest::Greater => {
                                self.gl.depth_func(GL_GREATER);
                            }
                            crate::DepthTest::LessOrEqual => {
                                self.gl.depth_func(GL_LEQUAL);
                            }
                            crate::DepthTest::GreaterOrEqual => {
                                self.gl.depth_func(GL_GEQUAL);
                            }
                        };

                        match pipeline.faces_to_render {
                            FacesToRender::Front => {
                                self.gl.enable(GL_CULL_FACE);
                                self.gl.cull_face(GL_BACK)
                            }
                            FacesToRender::Back => {
                                self.gl.enable(GL_CULL_FACE);
                                self.gl.cull_face(GL_FRONT)
                            }
                            FacesToRender::FrontAndBack => {
                                self.gl.disable(GL_CULL_FACE);
                            }
                            FacesToRender::None => {
                                self.gl.enable(GL_CULL_FACE);
                                self.gl.cull_face(GL_FRONT_AND_BACK)
                            }
                        };

                        if let Some((source_blend_factor, destination_blend_factor)) =
                            pipeline.blending
                        {
                            fn blending_to_gl(blending: BlendFactor) -> GLenum {
                                match blending {
                                    BlendFactor::OneMinusSourceAlpha => GL_ONE_MINUS_SRC_ALPHA,
                                    BlendFactor::SourceAlpha => GL_SRC_ALPHA,
                                }
                            }

                            self.gl.enable(GL_BLEND);
                            self.gl.blend_func(
                                blending_to_gl(source_blend_factor),
                                blending_to_gl(destination_blend_factor),
                            );
                        } else {
                            self.gl.disable(GL_BLEND);
                        }

                        self.gl.gl.ClearDepth(1.0);
                    }
                    SetVertexAttribute((attribute, buffer)) => {
                        if buffer.is_none() {
                            self.gl.disable_vertex_attrib_array(attribute.index);
                        } else {
                            self.gl.bind_buffer(GL_ARRAY_BUFFER, buffer);
                            self.gl.vertex_attrib_pointer_f32(
                                attribute.index,                // Index
                                attribute.byte_size as i32 / 4, // Number of components. It's assumed that components are always 32 bit.
                                GL_FLOAT,
                                false,
                                0, // 0 means to assume tightly packed
                                0, // Offset
                            );

                            self.gl.enable_vertex_attrib_array(attribute.index);
                        }
                    }
                    SetVertexAttributeToConstant {
                        attribute,
                        length,
                        x,
                        y,
                        z,
                        w,
                    } => {
                        let values = [x, y, z, w];
                        self.gl.disable_vertex_attrib_array(attribute.index);
                        self.gl.vertex_attrib(attribute.index, length, &values);
                    }
                    SetIndexBuffer(buffer) => {
                        self.gl
                            .bind_buffer(GL_ELEMENT_ARRAY_BUFFER, Some(buffer.buffer));
                    }
                    SetIntUniform((location, handle)) => {
                        let data = command_buffer.uniforms.get_any(handle);
                        self.gl.uniform_1_i32(Some(location), *data);
                    }
                    SetFloatUniform((location, handle)) => {
                        let data = command_buffer.uniforms.get_any(handle);
                        self.gl.uniform_1_f32(Some(location), *data);
                    }
                    SetVec2Uniform((location, handle)) => {
                        let data: &(f32, f32) = command_buffer.uniforms.get_any(handle);
                        self.gl.uniform_2_f32(Some(location), data.0, data.1);
                    }
                    SetVec3Uniform((location, handle)) => {
                        let data: &(f32, f32, f32) = command_buffer.uniforms.get_any(handle);
                        self.gl
                            .uniform_3_f32(Some(location), data.0, data.1, data.2);
                    }
                    SetVec4Uniform((location, handle)) => {
                        let data: &(f32, f32, f32, f32) = command_buffer.uniforms.get_any(handle);
                        self.gl
                            .uniform_4_f32(Some(location), data.0, data.1, data.2, data.3);
                    }
                    SetMat4Uniform((location, handle)) => {
                        let data: &[f32; 16] = command_buffer.uniforms.get_any(handle);
                        self.gl
                            .uniform_matrix_4_f32_slice(Some(location), false, data);
                    }
                    SetTextureUnit((uniform_location, unit, texture)) => {
                        self.gl.uniform_1_i32(Some(uniform_location), unit as i32);
                        self.gl.active_texture(GL_TEXTURE0.0 + unit as u32);
                        self.gl.bind_texture(GL_TEXTURE_2D, texture);
                    }
                    SetTextureUnitToCubeMap((uniform_location, unit, texture)) => {
                        self.gl.uniform_1_i32(Some(uniform_location), unit as i32);
                        self.gl.active_texture(GL_TEXTURE0.0 + unit as u32);
                        self.gl.bind_texture(GL_TEXTURE_CUBE_MAP, texture);
                    }
                    SetViewport((x, y, width, height)) => {
                        self.gl
                            .viewport(x as i32, y as i32, width as i32, height as i32);
                    }
                    DrawTriangles(count) => {
                        self.gl
                            .draw_elements(GL_TRIANGLES, (count * 3) as i32, GL_UNSIGNED_INT, 0);
                    }
                    DrawTriangleArrays(count) => {
                        self.gl.draw_arrays(GL_TRIANGLES, 0, (count * 3) as i32);
                    }
                    SetDepthMask(value) => self.gl.set_depth_mask(value),
                    BlitFramebuffer {
                        target,
                        source_x,
                        source_y,
                        source_width,
                        source_height,
                        dest_x,
                        dest_y,
                        dest_width,
                        dest_height,
                    } => {
                        self.gl.bind_framebuffer(GL_DRAW_FRAMEBUFFER, target);
                        self.gl.blit_framebuffer(
                            source_x,
                            source_y,
                            source_width,
                            source_height,
                            dest_x,
                            dest_y,
                            dest_width,
                            dest_height,
                        )
                    }
                    Present => {
                        self.gl_context.swap_buffers();
                    }
                }
            }

            command_buffer.clear();
            self.old_command_buffers.push(command_buffer);
        }
    }

    fn new_pipeline(
        &mut self,
        vertex_function: VertexFunction,
        fragment_function: FragmentFunction,
        output_pixel_format: PixelFormat,
    ) -> PipelineBuilder {
        let mut pipeline_builder = PipelineBuilder::new(self);
        pipeline_builder.vertex = Some(vertex_function);
        pipeline_builder.fragment = Some(fragment_function);
        pipeline_builder.output_pixel_format = output_pixel_format;
        pipeline_builder
    }

    fn new_framebuffer(
        &mut self,
        color_texture: Option<&Texture>,
        depth_texture: Option<&Texture>,
        stencil_texture: Option<&Texture>,
    ) -> Framebuffer {
        fn bind_target_and_native_texture(
            gl: &mut gl_native::GL,
            attachment: GLenum,
            texture: Option<&Texture>,
        ) {
            let texture_type = texture.map(|t| &t.texture_type);
            let level = texture.map_or(0, |t| t.mip as i32);
            unsafe {
                match texture_type {
                    Some(TextureType::Texture(t)) => gl.framebuffer_texture_2d(
                        GL_FRAMEBUFFER,
                        attachment,
                        GL_TEXTURE_2D,
                        Some(*t),
                        level,
                    ),
                    Some(TextureType::CubeMap {
                        face,
                        texture_native,
                    }) => gl.framebuffer_texture_2d(
                        GL_FRAMEBUFFER,
                        attachment,
                        GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + *face as u32),
                        Some(*texture_native),
                        level,
                    ),
                    Some(TextureType::RenderBuffer(renderbuffer)) => gl.framebuffer_renderbuffer(
                        GL_FRAMEBUFFER,
                        attachment,
                        GL_RENDERBUFFER,
                        *renderbuffer,
                    ),
                    Some(TextureType::DefaultFramebuffer) => {
                        panic!("Cannot update default framebuffer")
                    }
                    None => gl.framebuffer_texture_2d(
                        GL_FRAMEBUFFER,
                        attachment,
                        GL_TEXTURE_2D,
                        None,
                        0,
                    ),
                }
            }
        }
        unsafe {
            let framebuffer = self.gl.create_framebuffer().unwrap();

            self.gl.bind_framebuffer(GL_FRAMEBUFFER, framebuffer);

            bind_target_and_native_texture(&mut self.gl, GL_COLOR_ATTACHMENT0, color_texture);
            bind_target_and_native_texture(&mut self.gl, GL_DEPTH_ATTACHMENT, depth_texture);
            bind_target_and_native_texture(&mut self.gl, GL_STENCIL_ATTACHMENT, stencil_texture);

            // For some reason on macOS if the default framebuffer is not rebind here Mac OpenGL driver code segfaults.
            self.gl
                .bind_framebuffer(GL_FRAMEBUFFER, Framebuffer::default());

            framebuffer
        }
    }

    fn delete_framebuffer(&mut self, framebuffer: Framebuffer) {
        unsafe {
            self.gl.delete_framebuffer(framebuffer);
        }
    }
}

impl GraphicsContext {
    fn new_program(
        &self,
        vertex_function: &VertexFunction,
        fragment_function: &FragmentFunction,
    ) -> Result<Program, String> {
        unsafe {
            let program = self.gl.create_program().unwrap();
            self.gl.attach_shader(program, vertex_function.shader);
            self.gl.attach_shader(program, fragment_function.shader);
            self.gl.link_program(program);

            if !self.gl.get_program_link_status(program) {
                Err(self.gl.get_program_info_log(program))
            } else {
                Ok(program)
            }
        }
    }

    fn compile_shader(&self, shader_type: GLenum, source: &str) -> Result<Shader, String> {
        #[cfg(target_arch = "wasm32")]
        let version = if shader_type == GL_FRAGMENT_SHADER {
            "#version 300 es\n precision mediump float;"
        } else {
            "#version 300 es"
        };
        #[cfg(all(not(target_arch = "wasm32")))]
        let version = "#version 410";

        let source = &format!("{}\n{}", version, source);
        unsafe {
            let shader = self.gl.create_shader(shader_type).unwrap();
            self.gl.shader_source(shader, source);
            self.gl.compile_shader(shader);

            if !self.gl.get_shader_compile_status(shader) {
                Err(self.gl.get_shader_info_log(shader))
            } else {
                Ok(shader)
            }
        }
    }
}

unsafe fn slice_to_bytes<T>(t: &[T]) -> &[u8] {
    let ptr = t.as_ptr() as *const u8;
    let size = std::mem::size_of::<T>() * t.len();
    std::slice::from_raw_parts(ptr, size)
}
