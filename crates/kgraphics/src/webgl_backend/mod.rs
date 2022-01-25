use crate::*;
use kwasm::*;
use raw_window_handle::HasRawWindowHandle;
use std::collections::HashMap;

pub struct GraphicsContext {
    old_command_buffers: Vec<CommandBuffer>,
    js: WebGLJS,
}

pub struct RenderTarget {
    pixel_format: PixelFormat,
}

#[derive(Debug, Clone)]
pub struct Framebuffer(Option<JSObjectDynamic>);

impl Framebuffer {
    pub unsafe fn from_js_object(js_object_dynamic: JSObjectDynamic) -> Self {
        Self(Some(js_object_dynamic))
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Debug, Clone)]
enum TextureType {
    Texture(JSObjectDynamic),
    CubeMap {
        face: u8,
        texture_native: JSObjectDynamic,
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
    texture: JSObjectDynamic,
}

impl CubeMap {
    pub fn get_face_texture(&self, face: usize) -> Texture {
        assert!(face < 6);
        Texture {
            texture_type: TextureType::CubeMap {
                face: face as u8,
                texture_native: self.texture.clone(),
            },
            mip: 0,
        }
    }
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
struct Uniform {
    uniform_type: u32,
    // size_bytes: u32,
    location: JSObjectDynamic,
}
#[derive(Debug)]
pub struct FragmentFunction {
    js_object: JSObjectDynamic,
}
#[derive(Debug)]
pub struct VertexFunction {
    js_object: JSObjectDynamic,
}
#[derive(Debug, Clone)]
pub struct DataBuffer<T> {
    js_object: JSObjectDynamic,
    phantom: std::marker::PhantomData<T>,
}
#[derive(Debug, Clone)]
pub struct IndexBuffer(JSObjectDynamic);

#[repr(u8)]
enum Command {
    Clear = 0,
    BindFramebuffer = 1,
    ChangePipeline = 2,
    SetVertexAttribute = 3,
    SetVertexAttributeToConstant = 4,
    // UNUSED
    SetFloatUniform = 5,
    SetIntUniform = 6,
    SetVec2Uniform = 7,
    SetVec3Uniform = 8,
    SetVec4Uniform = 9,
    SetMat4Uniform = 10,
    SetTextureUniform = 11,
    SetViewport = 12,
    DrawTriangles = 13,
    // Present = 14,
    SetCubeMapUniform = 15,
    SetDepthMask = 16,
}

pub struct CommandBuffer {
    commands: Vec<Command>,
    f32_data: Vec<f32>,
    u32_data: Vec<u32>,
}

pub struct RenderPass<'a> {
    command_buffer: &'a mut CommandBuffer,
}

pub struct IntProperty(JSObjectDynamic);
pub struct FloatProperty(JSObjectDynamic);
pub struct Vec2Property(JSObjectDynamic);
pub struct Vec3Property(JSObjectDynamic);
pub struct Vec4Property(JSObjectDynamic);
pub struct Mat4Property(JSObjectDynamic);
pub struct TextureProperty(JSObjectDynamic);
pub struct CubeMapProperty(JSObjectDynamic);

#[derive(Clone, Copy)]
pub struct VertexAttributeInfo {
    index: u32,
    byte_size: u32,
}

#[derive(Clone)]
pub struct Pipeline {
    program: JSObjectDynamic,
    vertex_attributes: HashMap<String, VertexAttributeInfo>,
    uniforms: HashMap<String, Uniform>,
    depth_test: DepthTest,
    faces_to_render: FacesToRender,
    blending: Option<(BlendFactor, BlendFactor)>,
    depth_clear_value: f32,
}

impl Pipeline {
    pub fn blending(&self) -> Option<(BlendFactor, BlendFactor)> {
        self.blending
    }
}

#[derive(Clone)]
pub struct VertexAttribute<T> {
    info: Option<VertexAttributeInfo>,
    phantom: std::marker::PhantomData<T>,
}

// These implementations are safe because their inner [JSObjectDynamic]
// can only be used by [GraphicsContext] which is not `Send`.
unsafe impl Send for Uniform {}
unsafe impl Send for FragmentFunction {}
unsafe impl Send for VertexFunction {}
unsafe impl<T> Send for DataBuffer<T> {}
unsafe impl Send for IndexBuffer {}
unsafe impl Send for Pipeline {}
unsafe impl Send for Texture {}

unsafe impl Sync for Uniform {}
unsafe impl Sync for FragmentFunction {}
unsafe impl Sync for VertexFunction {}
unsafe impl<T> Sync for DataBuffer<T> {}
unsafe impl Sync for IndexBuffer {}
unsafe impl Sync for Pipeline {}
unsafe impl Sync for Texture {}

impl Pipeline {
    fn get_property(&self, name: &str, type_: u32) -> Result<JSObjectDynamic, ()> {
        if let Some(uniform) = self.uniforms.get(name) {
            if uniform.uniform_type == type_ {
                Ok(uniform.location.clone())
            } else {
                Err(())
            }
        } else {
            Ok(JSObject::null())
        }
    }
}

impl PipelineTrait for Pipeline {
    fn get_int_property(&self, name: &str) -> Result<IntProperty, ()> {
        Ok(IntProperty(self.get_property(name, INT)?))
    }

    fn get_float_property(&self, name: &str) -> Result<FloatProperty, ()> {
        Ok(FloatProperty(self.get_property(name, FLOAT)?))
    }

    fn get_vec2_property(&self, name: &str) -> Result<Vec2Property, ()> {
        Ok(Vec2Property(self.get_property(name, FLOAT_VEC2)?))
    }

    fn get_vec3_property(&self, name: &str) -> Result<Vec3Property, ()> {
        Ok(Vec3Property(self.get_property(name, FLOAT_VEC3)?))
    }

    fn get_vec4_property(&self, name: &str) -> Result<Vec4Property, ()> {
        Ok(Vec4Property(self.get_property(name, FLOAT_VEC4)?))
    }

    fn get_mat4_property(&self, name: &str) -> Result<Mat4Property, ()> {
        Ok(Mat4Property(self.get_property(name, FLOAT_MAT4)?))
    }

    fn get_texture_property(&self, name: &str) -> Result<TextureProperty, ()> {
        Ok(TextureProperty(self.get_property(name, SAMPLER_2D)?))
    }

    fn get_cube_map_property(&self, name: &str) -> Result<CubeMapProperty, ()> {
        Ok(CubeMapProperty(self.get_property(name, SAMPLER_CUBE)?))
    }

    fn get_vertex_attribute<T>(&self, name: &str) -> Result<VertexAttribute<T>, String> {
        if let Some(attribute) = self.vertex_attributes.get(name) {
            if attribute.byte_size == std::mem::size_of::<T>() as u32 {
                return Ok(VertexAttribute {
                    info: Some(attribute.clone()),
                    phantom: std::marker::PhantomData,
                });
            } else {
                return Err(format!(
                    "Vertex attribute size mismatch for {:?}. /n Shader: {:?}, Rust: {:?}",
                    name,
                    attribute.byte_size,
                    std::mem::size_of::<T>()
                ));
            }
        }

        Ok(VertexAttribute::<T> {
            info: None,
            phantom: std::marker::PhantomData,
        })
    }
}

impl RenderPassTrait for RenderPass<'_> {
    fn set_pipeline(&mut self, pipeline: &Pipeline) {
        fn blending_to_gl(blending: BlendFactor) -> c_uint {
            match blending {
                BlendFactor::OneMinusSourceAlpha => ONE_MINUS_SRC_ALPHA,
                BlendFactor::SourceAlpha => SRC_ALPHA,
            }
        }

        let (source_blend_factor, destination_blend_factor) = match pipeline.blending {
            Some((source_blend_factor, destination_blend_factor)) => (
                blending_to_gl(source_blend_factor),
                blending_to_gl(destination_blend_factor),
            ),
            None => (0, 0),
        };

        self.command_buffer.commands.push(Command::ChangePipeline);
        self.command_buffer.u32_data.extend_from_slice(&[
            pipeline.program.index(),
            match pipeline.depth_test {
                crate::DepthTest::AlwaysPass => ALWAYS,
                crate::DepthTest::Less => LESS,
                crate::DepthTest::Greater => GREATER,
                crate::DepthTest::LessOrEqual => LEQUAL,
                crate::DepthTest::GreaterOrEqual => GEQUAL,
            },
            match pipeline.faces_to_render {
                FacesToRender::Front => BACK,
                FacesToRender::Back => FRONT,
                FacesToRender::FrontAndBack => 0,
                FacesToRender::None => FRONT_AND_BACK,
            },
            source_blend_factor,
            destination_blend_factor,
        ]);
        self.command_buffer
            .f32_data
            .push(pipeline.depth_clear_value);
    }

    fn set_vertex_attribute<T>(
        &mut self,
        vertex_attribute: &VertexAttribute<T>,
        buffer: Option<&DataBuffer<T>>,
    ) {
        if let Some(info) = vertex_attribute.info {
            self.command_buffer
                .commands
                .push(Command::SetVertexAttribute);

            self.command_buffer.u32_data.extend_from_slice(&[
                info.index,
                info.byte_size / 4, // Number of components
                buffer.map_or(0, |b| b.js_object.index()),
            ]);
        }
    }

    fn set_vertex_attribute_to_constant<T>(
        &mut self,
        vertex_attribute: &VertexAttribute<T>,
        value: &[f32],
    ) {
        if let Some(info) = vertex_attribute.info.clone() {
            self.command_buffer
                .commands
                .push(Command::SetVertexAttributeToConstant);
            self.command_buffer
                .u32_data
                .extend_from_slice(&[info.index, value.len() as u32]);
            self.command_buffer.f32_data.extend_from_slice(&value);
        }
    }

    fn set_float_property(&mut self, property: &FloatProperty, value: f32) {
        if !property.0.is_null() {
            self.command_buffer.commands.push(Command::SetFloatUniform);
            self.command_buffer.u32_data.push(property.0.index());
            self.command_buffer.f32_data.push(value);
        }
    }

    fn set_int_property(&mut self, property: &IntProperty, value: i32) {
        if !property.0.is_null() {
            self.command_buffer.commands.push(Command::SetIntUniform);
            self.command_buffer.u32_data.push(property.0.index());
            self.command_buffer.u32_data.push(value as u32);
        }
    }

    fn set_vec2_property(&mut self, property: &Vec2Property, value: (f32, f32)) {
        if !property.0.is_null() {
            self.command_buffer.commands.push(Command::SetVec2Uniform);
            self.command_buffer.u32_data.push(property.0.index());
            self.command_buffer
                .f32_data
                .extend_from_slice(&[value.0, value.1]);
        }
    }

    fn set_vec3_property(&mut self, property: &Vec3Property, value: (f32, f32, f32)) {
        if !property.0.is_null() {
            self.command_buffer.commands.push(Command::SetVec3Uniform);
            self.command_buffer.u32_data.push(property.0.index());
            self.command_buffer
                .f32_data
                .extend_from_slice(&[value.0, value.1, value.2]);
        }
    }

    fn set_vec4_property(&mut self, property: &Vec4Property, value: (f32, f32, f32, f32)) {
        if !property.0.is_null() {
            self.command_buffer.commands.push(Command::SetVec4Uniform);
            self.command_buffer.u32_data.push(property.0.index());
            self.command_buffer
                .f32_data
                .extend_from_slice(&[value.0, value.1, value.2, value.3]);
        }
    }
    fn set_mat4_property(&mut self, property: &Mat4Property, value: &[f32; 16]) {
        if !property.0.is_null() {
            self.command_buffer.commands.push(Command::SetMat4Uniform);
            self.command_buffer.u32_data.push(property.0.index());
            self.command_buffer.f32_data.extend_from_slice(value);
        }
    }

    fn set_texture_property(
        &mut self,
        property: &TextureProperty,
        texture: Option<&Texture>,
        texture_unit: u8,
    ) {
        debug_assert!(texture_unit < 16);

        if !property.0.is_null() {
            self.command_buffer
                .commands
                .push(Command::SetTextureUniform);
            let texture_index = texture.map_or(0, |t| match &t.texture_type {
                TextureType::Texture(t) => t.index(),
                TextureType::CubeMap { .. } => {
                    todo!()
                }
                TextureType::DefaultFramebuffer => panic!("Cannot update default framebuffer"),
            });
            self.command_buffer.u32_data.extend_from_slice(&[
                property.0.index(),
                texture_index,
                texture_unit as u32,
            ]);
        }
    }

    fn set_cube_map_property(
        &mut self,
        property: &CubeMapProperty,
        texture: Option<&CubeMap>,
        texture_unit: u8,
    ) {
        debug_assert!(texture_unit < 16);

        if !property.0.is_null() {
            self.command_buffer
                .commands
                .push(Command::SetCubeMapUniform);
            self.command_buffer.u32_data.extend_from_slice(&[
                property.0.index(),
                texture.map_or(0, |t| t.texture.index()),
                texture_unit as u32,
            ]);
        }
    }

    fn set_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.command_buffer.commands.push(Command::SetViewport);
        self.command_buffer
            .u32_data
            .extend_from_slice(&[x, y, width, height]);
    }

    fn draw_triangles(&mut self, count: u32, buffer: &IndexBuffer) {
        self.command_buffer.commands.push(Command::DrawTriangles);
        self.command_buffer
            .u32_data
            .extend_from_slice(&[count * 3, buffer.0.index()]);
    }

    fn draw_triangles_without_buffer(&mut self, count: u32) {
        self.command_buffer.commands.push(Command::DrawTriangles);
        self.command_buffer
            .u32_data
            .extend_from_slice(&[count * 3, 0]);
    }

    fn set_depth_mask(&mut self, value: bool) {
        self.command_buffer.commands.push(Command::SetDepthMask);
        self.command_buffer
            .u32_data
            .extend_from_slice(&[if value { 1 } else { 0 }]);
    }

    fn blit_framebuffer(
        self,
        target: Framebuffer,
        source_x: u32,
        source_y: u32,
        source_width: u32,
        source_height: u32,
        dest_x: u32,
        dest_y: u32,
        dest_width: u32,
        dest_height: u32,
    ) {
        todo!()
    }
}

impl CommandBuffer {
    pub(crate) fn clear(&mut self) {
        self.commands.clear();
        self.u32_data.clear();
        self.f32_data.clear();
    }
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
        self.commands.push(Command::BindFramebuffer);
        self.u32_data.extend_from_slice(&[
            framebuffer.0.as_ref().map_or(0, |o| o.index()),
            0,
            0,
            0,
        ]);

        let mut render_pass = RenderPass {
            command_buffer: self,
        };

        // This is before Clear otherwise the Clear doesn't clear depth.
        render_pass.set_depth_mask(true);

        if let Some((r, g, b, a)) = clear_color {
            render_pass.command_buffer.commands.push(Command::Clear);
            render_pass
                .command_buffer
                .f32_data
                .extend_from_slice(&[r, g, b, a]);
        }

        render_pass
    }

    /// If the color_texture binds to the DefaultFramebuffer then
    /// all textures will bind to the default framebuffer.
    fn begin_render_pass<'a>(
        &'a mut self,
        _color_texture: Option<&Texture>,
        _depth_texture: Option<&Texture>,
        _stencil_texture: Option<&Texture>,
        _clear_color: Option<(f32, f32, f32, f32)>,
    ) -> RenderPass<'a> {
        todo!()
        /*
        debug_assert!(
            color_texture.is_none() && depth_texture.is_none() && stencil_texture.is_none(),
            "Configurable render textures are disabled for now"
        );
        self.commands.push(Command::BindFramebuffer);
        match color_texture {
            Some(Texture {
                texture_type: TextureType::DefaultFramebuffer,
            }) => {
                self.u32_data.extend_from_slice(&[0, 0, 0, 0]);
            }
            _ => {
                let color_index = color_texture.map_or(0, |t| match &t.texture_type {
                    TextureType::Texture(t) => t.index(),
                    TextureType::DefaultFramebuffer => {
                        panic!("Depth texture cannot be bound to the default framebuffer")
                    }
                });

                let depth_index = depth_texture.map_or(0, |t| match &t.texture_type {
                    TextureType::Texture(t) => t.index(),
                    TextureType::DefaultFramebuffer => {
                        panic!("Depth texture cannot be bound to the default framebuffer")
                    }
                });
                let stencil_index = stencil_texture.map_or(0, |t| match &t.texture_type {
                    TextureType::Texture(t) => t.index(),
                    TextureType::DefaultFramebuffer => {
                        panic!("Stencil texture cannot be bound to the default framebuffer")
                    }
                });

                self.u32_data
                    .extend_from_slice(&[1, color_index, depth_index, stencil_index]);
            }
        }

        if let Some((r, g, b, a)) = clear_color {
            self.commands.push(Command::Clear);
            self.f32_data.extend_from_slice(&[r, g, b, a]);
        }

        RenderPass {
            command_buffer: self,
        }
        */
    }

    fn present(&mut self) {}
}

struct WebGLJS {
    new: JSObjectDynamic,
    resize: JSObjectDynamic,
    new_vertex_function: JSObjectDynamic,
    new_fragment_function: JSObjectDynamic,
    new_data_buffer: JSObjectDynamic,
    new_index_buffer: JSObjectDynamic,
    delete_buffer: JSObjectDynamic,
    new_texture: JSObjectDynamic,
    update_texture: JSObjectDynamic,
    delete_texture: JSObjectDynamic,
    new_program: JSObjectDynamic,
    get_uniform_name_and_type: JSObjectDynamic,
    get_uniform_location: JSObjectDynamic,
    get_program_parameter: JSObjectDynamic,
    get_attribute_name_and_type: JSObjectDynamic,
    run_command_buffer: JSObjectDynamic,
    get_attribute_location: JSObjectDynamic,
    get_multiview_supported: JSObjectDynamic,
    generate_mip_map: JSObjectDynamic,
    framebuffer_texture_2d: JSObjectDynamic,
    bind_framebuffer: JSObjectDynamic,
    create_framebuffer: JSObjectDynamic,
    delete_framebuffer: JSObjectDynamic,
}

impl WebGLJS {
    pub fn new() -> Self {
        let o = JSObjectFromString::new(include_str!("webgl_backend.js"));
        Self {
            new: o.get_property("new"),
            resize: o.get_property("resize"),
            new_vertex_function: o.get_property("new_vertex_function"),
            new_fragment_function: o.get_property("new_fragment_function"),
            new_data_buffer: o.get_property("new_data_buffer"),
            new_index_buffer: o.get_property("new_index_buffer"),
            delete_buffer: o.get_property("delete_buffer"),
            new_texture: o.get_property("new_texture"),
            update_texture: o.get_property("update_texture"),
            delete_texture: o.get_property("delete_texture"),
            new_program: o.get_property("new_program"),
            get_uniform_name_and_type: o.get_property("get_uniform_name_and_type"),
            get_uniform_location: o.get_property("get_uniform_location"),
            get_program_parameter: o.get_property("get_program_parameter"),
            get_attribute_name_and_type: o.get_property("get_attribute_name_and_type"),
            run_command_buffer: o.get_property("run_command_buffer"),
            get_attribute_location: o.get_property("get_attribute_location"),
            get_multiview_supported: o.get_property("get_multiview_supported"),
            generate_mip_map: o.get_property("generate_mip_map"),
            framebuffer_texture_2d: o.get_property("framebuffer_texture_2d"),
            bind_framebuffer: o.get_property("bind_framebuffer"),
            create_framebuffer: o.get_property("create_framebuffer"),
            delete_framebuffer: o.get_property("delete_framebuffer"),
        }
    }
}

impl GraphicsContext {
    /// A web-only way to use browser image decoders.
    fn new_texture_from_js_object(
        &mut self,
        width: u32,
        height: u32,
        js_object_data: kwasm::JSObjectDynamic,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<Texture, ()> {
        let js_texture_object = self.js.new_texture.call().unwrap();

        let texture = Texture {
            texture_type: TextureType::Texture(js_texture_object),
            mip: 0,
        };
        self.update_texture_internal(
            &texture,
            width,
            height,
            js_object_data,
            None,
            pixel_format,
            texture_settings,
        );
        Ok(texture)
    }

    fn update_texture_internal(
        &mut self,
        texture: &Texture,
        width: u32,
        height: u32,
        js_object_data: kwasm::JSObjectDynamic,
        data: Option<&[u8]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) {
        let (pixel_format, inner_pixel_format, type_) =
            crate::gl_shared::pixel_format_to_gl_format_and_inner_format_and_type(
                pixel_format,
                texture_settings.srgb,
            );

        let (target, texture) = match &texture.texture_type {
            TextureType::Texture(t) => (TEXTURE_2D, t.index()),
            TextureType::CubeMap {
                face,
                texture_native,
            } => (
                TEXTURE_CUBE_MAP_POSITIVE_X + *face as u32,
                texture_native.index(),
            ),
            TextureType::DefaultFramebuffer => panic!("Cannot update default framebuffer"),
        };

        let minification_filter = minification_filter_to_gl_enum(
            texture_settings.minification_filter,
            texture_settings.mipmap_filter,
            texture_settings.generate_mipmaps,
        );
        let magnification_filter =
            magnification_filter_to_gl_enum(texture_settings.magnification_filter);

        let wrapping_horizontal = wrapping_to_gl_enum(texture_settings.wrapping_horizontal);
        let wrapping_vertical = wrapping_to_gl_enum(texture_settings.wrapping_vertical);

        let (data_ptr, data_len) = data.map_or((0, 0), |d| (d.as_ptr() as u32, d.len() as u32));

        self.js.update_texture.call_raw(&[
            texture,
            target,
            target,
            inner_pixel_format,
            width,
            height,
            pixel_format,
            type_,
            js_object_data.index(),
            data_ptr,
            data_len,
            minification_filter,
            magnification_filter,
            wrapping_horizontal,
            wrapping_vertical,
        ]);

        if texture_settings.generate_mipmaps {
            self.js.generate_mip_map.call_raw(&[texture, TEXTURE_2D]);
        }
    }
}

impl GraphicsContextTrait for GraphicsContext {
    fn new() -> Result<Self, ()> {
        Self::new_with_settings(Default::default())
    }
    fn new_with_settings(settings: crate::GraphicsContextSettings) -> Result<Self, ()> {
        let js = WebGLJS::new();

        let msaa_enabled = if settings.samples > 0 { 1 } else { 0 };
        // Initialize context
        js.new.call_raw(&[msaa_enabled]);
        Ok(Self {
            js,
            old_command_buffers: Vec::new(),
        })
    }

    /// This must only be called once per window.
    unsafe fn get_render_target_for_window(
        &mut self,
        _window: &impl HasRawWindowHandle,
        _width: u32,
        _height: u32,
    ) -> Result<RenderTarget, ()> {
        // This is obviously incorrect. This should be fixed.
        let pixel_format = PixelFormat::RGBA8Unorm;

        // There's only one RenderTarget per context right now.
        Ok(RenderTarget { pixel_format })
    }

    fn resize(&mut self, _window: &impl HasRawWindowHandle, width: u32, height: u32) {
        self.js.resize.call_raw(&[width, height]);
    }
    fn new_vertex_function(&mut self, source: &str) -> Result<VertexFunction, String> {
        let source = "#version 300 es\n".to_owned() + source;
        let source = JSString::new(&source);
        let js_object = self.js.new_vertex_function.call_1_arg(&source).unwrap();
        Ok(VertexFunction { js_object })
    }

    fn new_fragment_function(&mut self, source: &str) -> Result<FragmentFunction, String> {
        let source = "#version 300 es\nprecision mediump float;\n".to_owned() + source;
        let source = JSString::new(&source);
        let js_object = self.js.new_fragment_function.call_1_arg(&source).unwrap();
        Ok(FragmentFunction { js_object })
    }

    fn new_data_buffer<T>(&mut self, data: &[T]) -> Result<DataBuffer<T>, ()> {
        let js_object = self
            .js
            .new_data_buffer
            .call_raw(&[
                data.as_ptr() as u32,
                (data.len() * std::mem::size_of::<T>()) as u32,
            ])
            .unwrap();

        Ok(DataBuffer {
            js_object,
            phantom: std::marker::PhantomData,
        })
    }

    fn delete_data_buffer<T>(&mut self, data_buffer: DataBuffer<T>) {
        self.js.delete_buffer.call_1_arg(&data_buffer.js_object);
    }

    fn new_index_buffer(&mut self, data: &[u32]) -> Result<IndexBuffer, ()> {
        let js_object = self
            .js
            .new_index_buffer
            .call_raw(&[data.as_ptr() as u32, data.len() as u32])
            .unwrap();
        Ok(IndexBuffer(js_object))
    }
    fn delete_index_buffer(&mut self, index_buffer: IndexBuffer) {
        self.js.delete_buffer.call_1_arg(&index_buffer.0);
    }

    fn new_texture(
        &mut self,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<Texture, ()> {
        let js_object = self.js.new_texture.call().unwrap();

        let texture = Texture {
            texture_type: TextureType::Texture(js_object),
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
    }

    fn update_texture(
        &mut self,
        texture: &Texture,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) {
        self.update_texture_internal(
            texture,
            width,
            height,
            JSObject::null(),
            data,
            pixel_format,
            texture_settings,
        )
    }

    fn delete_texture(&mut self, texture: Texture) {
        match texture.texture_type {
            TextureType::Texture(js_object) => {
                self.js.delete_texture.call_1_arg(&js_object);
            }
            TextureType::CubeMap { .. } => {}
            TextureType::DefaultFramebuffer => {
                panic!("Can't delete default framebuffer");
            }
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

    fn new_command_buffer(&mut self) -> CommandBuffer {
        // Reuse a previously allocated [CommandBuffer] if one is available
        self.old_command_buffers
            .pop()
            .unwrap_or_else(|| CommandBuffer {
                commands: Vec::new(),
                u32_data: Vec::new(),
                f32_data: Vec::new(),
            })
    }
    fn commit_command_buffer(&mut self, mut command_buffer: CommandBuffer) {
        // Run [CommandBuffer] here
        self.js.run_command_buffer.call_raw(&[
            command_buffer.commands.as_ptr() as u32,
            command_buffer.commands.len() as u32,
            command_buffer.f32_data.as_ptr() as u32,
            (command_buffer.f32_data.len()) as u32,
            command_buffer.u32_data.as_ptr() as u32,
            (command_buffer.u32_data.len()) as u32,
        ]);

        // panic!();

        command_buffer.clear();
        self.old_command_buffers.push(command_buffer);
    }

    fn new_cube_map(
        &mut self,
        width: u32,
        height: u32,
        data: Option<[&[u8]; 6]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) -> Result<CubeMap, ()> {
        let texture = self.js.new_texture.call().unwrap();

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

    fn update_cube_map(
        &mut self,
        cube_map: &CubeMap,
        width: u32,
        height: u32,
        data: Option<[&[u8]; 6]>,
        pixel_format: PixelFormat,
        texture_settings: TextureSettings,
    ) {
        // Convert data to linear instead of sRGB if needed and flip the image vertically.
        let (pixel_format, inner_pixel_format, type_) =
            crate::gl_shared::pixel_format_to_gl_format_and_inner_format_and_type(
                pixel_format,
                texture_settings.srgb,
            );

        let texture = &cube_map.texture;

        let minification_filter = minification_filter_to_gl_enum(
            texture_settings.minification_filter,
            texture_settings.mipmap_filter,
            texture_settings.generate_mipmaps,
        );
        let magnification_filter =
            magnification_filter_to_gl_enum(texture_settings.magnification_filter);

        let wrapping_horizontal = wrapping_to_gl_enum(texture_settings.wrapping_horizontal);
        let wrapping_vertical = wrapping_to_gl_enum(texture_settings.wrapping_vertical);

        for i in 0..6 {
            let (data_ptr, data_len) =
                data.map_or((0, 0), |d| (d[i].as_ptr() as u32, d[i].len() as u32));

            self.js.update_texture.call_raw(&[
                texture.index(),
                TEXTURE_CUBE_MAP,
                TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                inner_pixel_format,
                width,
                height,
                pixel_format,
                type_,
                data_ptr,
                data_len,
                minification_filter,
                magnification_filter,
                wrapping_horizontal,
                wrapping_vertical,
            ]);
        }
        if texture_settings.generate_mipmaps {
            self.js
                .generate_mip_map
                .call_raw(&[texture.index(), TEXTURE_CUBE_MAP]);
        }
    }

    fn delete_cube_map(&mut self, cube_map: CubeMap) {
        self.js.delete_texture.call_1_arg(&cube_map.texture);
    }

    fn generate_mip_map_for_texture(&mut self, texture: &Texture) {
        let (target, texture) = match &texture.texture_type {
            TextureType::Texture(t) => (TEXTURE_2D, t.index()),
            TextureType::CubeMap {
                face,
                texture_native,
            } => (
                TEXTURE_CUBE_MAP_POSITIVE_X + *face as u32,
                texture_native.index(),
            ),
            TextureType::DefaultFramebuffer => panic!("Cannot update default framebuffer"),
        };
        self.js.generate_mip_map.call_raw(&[texture, target]);
    }

    fn generate_mip_map_for_cube_map(&mut self, texture: &CubeMap) {
        self.js
            .generate_mip_map
            .call_raw(&[texture.texture.index(), TEXTURE_CUBE_MAP]);
    }

    fn new_framebuffer(
        &mut self,
        color_texture: Option<&Texture>,
        depth_texture: Option<&Texture>,
        stencil_texture: Option<&Texture>,
    ) -> Framebuffer {
        fn get_target_and_native_texture(texture: Option<&Texture>) -> (u32, u32, u32) {
            let texture_type = texture.map(|t| &t.texture_type);
            let level = texture.map_or(0, |t| t.mip as u32);
            match texture_type {
                Some(TextureType::Texture(t)) => (TEXTURE_2D, t.index(), level),
                Some(TextureType::CubeMap {
                    face,
                    texture_native,
                }) => (
                    TEXTURE_CUBE_MAP_POSITIVE_X + *face as u32,
                    texture_native.index(),
                    level,
                ),
                Some(TextureType::DefaultFramebuffer) => {
                    panic!("Cannot update default framebuffer")
                }
                None => (TEXTURE_2D, 0, 0),
            }
        }
        let framebuffer = self.js.create_framebuffer.call().unwrap();

        self.js.bind_framebuffer.call_1_arg(&framebuffer);

        let (target, texture, level) = get_target_and_native_texture(color_texture);
        self.js
            .framebuffer_texture_2d
            .call_raw(&[COLOR_ATTACHMENT0, target, texture, level]);

        let (target, texture, level) = get_target_and_native_texture(depth_texture);
        self.js
            .framebuffer_texture_2d
            .call_raw(&[DEPTH_ATTACHMENT, target, texture, level]);
        let (target, texture, level) = get_target_and_native_texture(stencil_texture);
        self.js
            .framebuffer_texture_2d
            .call_raw(&[STENCIL_ATTACHMENT, target, texture, level]);
        Framebuffer(Some(framebuffer))
    }

    fn delete_framebuffer(&mut self, framebuffer: Framebuffer) {
        if let Some(framebuffer) = framebuffer.0 {
            self.js.delete_framebuffer.call_1_arg(&framebuffer);
        }
    }

    fn get_multiview_supported(&self) -> MultiviewSupport {
        match self
            .js
            .get_multiview_supported
            .call()
            .unwrap()
            .get_value_u32()
        {
            0 => MultiviewSupport::None,
            1 => MultiviewSupport::WithoutMsaa,
            2 => MultiviewSupport::OculusWithMsaa,
            _ => unreachable!(),
        }
    }
}

impl<'a> PipelineBuilderTrait for PipelineBuilder<'a> {
    fn build(self) -> Result<Pipeline, String> {
        // Build the program
        let program = self
            .g
            .js
            .new_program
            .call_2_arg(
                &self.vertex.unwrap().js_object,
                &self.fragment.unwrap().js_object,
            )
            .unwrap();

        let mut uniforms = HashMap::new();

        let uniform_count = self
            .g
            .js
            .get_program_parameter
            .call_raw(&[program.index(), ACTIVE_UNIFORMS])
            .unwrap()
            .get_value_u32();

        for i in 0..uniform_count {
            let uniform_type = self
                .g
                .js
                .get_uniform_name_and_type
                .call_raw(&[program.index(), i])
                .unwrap()
                .get_value_u32();
            let uniform_name = kwasm::get_string_from_host();

            // Passing the name immediately back to JS probably isn't the best here.
            if let Some(uniform_location) = self
                .g
                .js
                .get_uniform_location
                .call_2_arg(&program, &JSString::new(&uniform_name))
            {
                uniforms.insert(
                    uniform_name,
                    Uniform {
                        uniform_type,
                        location: uniform_location,
                    },
                );
            }
        }

        let mut vertex_attributes = HashMap::new();
        let vertex_attribute_count = self
            .g
            .js
            .get_program_parameter
            .call_raw(&[program.index(), ACTIVE_ATTRIBUTES])
            .unwrap()
            .get_value_u32();

        for i in 0..vertex_attribute_count {
            let attribute_type = self
                .g
                .js
                .get_attribute_name_and_type
                .call_raw(&[program.index(), i])
                .unwrap()
                .get_value_u32();

            let byte_size = match attribute_type {
                FLOAT => 4,
                FLOAT_VEC2 => 8,
                FLOAT_VEC3 => 12,
                FLOAT_VEC4 => 16,
                FLOAT_MAT4 => 64,
                _ => continue,
            };

            let attribute_name = kwasm::get_string_from_host();

            // Passing the name immediately back to JS probably isn't the best here.
            // Notably the attribute location index is *not* the index passed into `GetActiveAttrib`
            if let Some(attribute_location) = self
                .g
                .js
                .get_attribute_location
                .call_2_arg(&program, &JSString::new(&attribute_name))
            {
                let attribute_location = attribute_location.get_value_u32();

                vertex_attributes.insert(
                    attribute_name,
                    VertexAttributeInfo {
                        index: attribute_location,
                        byte_size,
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
            depth_clear_value: self.depth_clear_value,
        })
    }
}
