use crate::PixelFormat;
/// This backend uses the following for binding small amounts of data for uniforms:
/// https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515846-setvertexbytes?language=objc
/// This is recommended by the following Apple docs:
/// https://developer.apple.com/library/archive/documentation/3DDrawing/Conceptual/MTLBestPracticesGuide/BufferBindings.html
/// This approach is also more conceptually similar to how the uniform API work with OpenGL.
/// In the future it may be worthwhile to consider allocating buffers instead.
use raw_window_handle::*;
use std::collections::HashMap;

pub struct GraphicsContext {
    device: *mut Object,
    command_queue: *mut Object,
}

pub struct VertexFunction {
    function: *mut Object,
}

pub struct FragmentFunction {
    function: *mut Object,
}

pub struct Pipeline {
    pipeline: *mut Object,
    properties: HashMap<String, PipelineProperty>,
}

enum PipelineProperty {
    FloatProperty(PropertyInner),
    Vec2Property(PropertyInner),
    Vec3Property(PropertyInner),
    Vec4Property(PropertyInner),
    Mat4Property(PropertyInner),
    TextureProperty(PropertyInner),
}

pub struct RenderTarget {
    ca_layer: *mut Object,
}

pub struct RenderPass {
    render_pass_descriptor: *mut Object,
    drawable: *mut Object,
    auto_release_pool: *mut Object,
}

pub struct CommandBuffer {
    command_buffer: *mut Object,
}

pub struct DataBuffer {
    buffer: *mut Object,
}

pub struct IndexBuffer {
    buffer: *mut Object,
}

pub struct Texture {
    texture: *mut Object,
}

#[derive(Clone)]
struct PropertyInner {
    index: u64,
    size: u64,
    /// Properties are associated with a stage
    vertex_or_fragment: VertexOrFragmentProperty,
}

#[derive(Clone, Copy)]
enum VertexOrFragmentProperty {
    Vertex,
    Fragment,
}

#[derive(Clone)]
pub struct FloatProperty(Option<PropertyInner>);
#[derive(Clone)]
pub struct Vec2Property(Option<PropertyInner>);
#[derive(Clone)]
pub struct Vec3Property(Option<PropertyInner>);
#[derive(Clone)]
pub struct Vec4Property(Option<PropertyInner>);
#[derive(Clone)]
pub struct Mat4Property(Option<PropertyInner>);
#[derive(Clone)]
pub struct TextureProperty(Option<PropertyInner>);

impl Pipeline {
    pub fn get_float_property(&self, name: &str) -> Result<FloatProperty, ()> {
        if let Some(p) = self.properties.get(name) {
            match p {
                PipelineProperty::FloatProperty(property_inner) => {
                    Ok(FloatProperty(Some(property_inner.clone())))
                }
                _ => Err(()),
            }
        } else {
            Ok(FloatProperty(None))
        }
    }

    pub fn get_vec2_property(&self, name: &str) -> Result<Vec2Property, ()> {
        if let Some(p) = self.properties.get(name) {
            match p {
                PipelineProperty::Vec2Property(property_inner) => {
                    Ok(Vec2Property(Some(property_inner.clone())))
                }
                _ => Err(()),
            }
        } else {
            Ok(Vec2Property(None))
        }
    }

    pub fn get_vec3_property(&self, name: &str) -> Result<Vec3Property, ()> {
        if let Some(p) = self.properties.get(name) {
            match p {
                PipelineProperty::Vec3Property(property_inner) => {
                    Ok(Vec3Property(Some(property_inner.clone())))
                }
                _ => Err(()),
            }
        } else {
            Ok(Vec3Property(None))
        }
    }

    pub fn get_vec4_property(&self, name: &str) -> Result<Vec4Property, ()> {
        if let Some(p) = self.properties.get(name) {
            match p {
                PipelineProperty::Vec4Property(property_inner) => {
                    Ok(Vec4Property(Some(property_inner.clone())))
                }
                _ => Err(()),
            }
        } else {
            Ok(Vec4Property(None))
        }
    }

    pub fn get_mat4_property(&self, name: &str) -> Result<Mat4Property, ()> {
        if let Some(p) = self.properties.get(name) {
            match p {
                PipelineProperty::Mat4Property(property_inner) => {
                    Ok(Mat4Property(Some(property_inner.clone())))
                }
                _ => Err(()),
            }
        } else {
            Ok(Mat4Property(None))
        }
    }

    pub fn get_texture_property(&self, name: &str) -> Result<TextureProperty, ()> {
        if let Some(p) = self.properties.get(name) {
            match p {
                PipelineProperty::TextureProperty(property_inner) => {
                    Ok(TextureProperty(Some(property_inner.clone())))
                }
                _ => Err(()),
            }
        } else {
            Ok(TextureProperty(None))
        }
    }
}

fn bytes_to_bgra(bytes: &[u8], pixel_format: PixelFormat) -> Vec<u8> {
    match pixel_format {
        PixelFormat::RGB8Unorm => {
            let mut new_bytes = Vec::with_capacity(4 * (bytes.len() / 3));
            for s in bytes.chunks(3) {
                new_bytes.extend_from_slice(&[s[2], s[1], s[0], 255])
            }
            new_bytes
        }
        PixelFormat::RGBA8Unorm => {
            let mut new_bytes = Vec::with_capacity(bytes.len());
            for s in bytes.chunks(3) {
                new_bytes.extend_from_slice(&[s[2], s[1], s[0], s[3]])
            }
            new_bytes
        }
        // This is an unnecessary copy
        PixelFormat::BGRA8Unorm => bytes.to_vec(),
        _ => unimplemented!(),
    }
}

impl GraphicsContext {
    pub fn new() -> Result<Self, ()> {
        let device = unsafe { MTLCreateSystemDefaultDevice() };
        if device == nil {
            return Err(());
        }

        let command_queue: *mut Object = unsafe { msg_send![device, newCommandQueue] };
        if command_queue == nil {
            return Err(());
        }

        Ok(GraphicsContext {
            device,
            command_queue,
        })
    }

    /// This must only be called once per window.
    pub unsafe fn get_render_target_for_window(
        &mut self,
        window: &impl HasRawWindowHandle,
    ) -> Result<RenderTarget, ()> {
        let raw_window_handle = window.raw_window_handle();
        let view = match raw_window_handle {
            RawWindowHandle::MacOS(handle) => handle.ns_view as *mut Object,
            _ => unreachable!(),
        };
        // Setup the backing layer for the window

        let ca_layer: *mut Object = msg_send![class!(CAMetalLayer), layer];

        let () = msg_send![ca_layer, setDevice: self.device];

        let () = msg_send![ca_layer, setDisplaySyncEnabled: YES];
        let () = msg_send![ca_layer, setAllowsNextDrawableTimeout: YES];
        // let () = msg_send![ca_layer, setPresentsWithTransaction: YES];
        let () = msg_send![ca_layer, setMaximumDrawableCount: 2];

        let () = msg_send![view, setWantsLayer: YES];
        let () = msg_send![view, setLayer: ca_layer];

        let () = msg_send![ca_layer, setDelegate: view];

        let () = msg_send![view, setLayerContentsRedrawPolicy: 1 | 3];

        Ok(RenderTarget { ca_layer })
    }

    pub fn resize(&mut self, window: &impl HasRawWindowHandle, width: u32, height: u32) {
        let raw_window_handle = window.raw_window_handle();
        let view = match raw_window_handle {
            RawWindowHandle::MacOS(handle) => handle.ns_view as *mut Object,
            _ => unreachable!(),
        };
        unsafe {
            let layer: *mut Object = msg_send![view, layer];
            if (*layer).class() == class!(CAMetalLayer) {
                let () = msg_send![layer, setDrawableSize: CGSize::new(width as CGFloat, height as CGFloat)];
            }
        }
    }

    pub fn new_vertex_function(&self, source: &str) -> Result<VertexFunction, String> {
        let library = self.compile_library(&source)?;

        unsafe {
            let function: *mut Object =
                msg_send![library, newFunctionWithName: NSString::new("main1")];
            let () = msg_send![library, release];
            Ok(VertexFunction { function })
        }
    }

    pub fn new_fragment_function(&self, source: &str) -> Result<FragmentFunction, String> {
        let library = self.compile_library(&source)?;

        unsafe {
            let function: *mut Object =
                msg_send![library, newFunctionWithName: NSString::new("main1")];
            let () = msg_send![library, release];
            Ok(FragmentFunction { function })
        }
    }

    pub fn new_pipeline(
        &mut self,
        vertex_function: VertexFunction,
        fragment_function: FragmentFunction,
    ) -> PipelineBuilder {
        PipelineBuilder {
            g: self,
            vertex: Some(vertex_function),
            fragment: Some(fragment_function),
            vertex_attributes: Vec::new(),
        }
    }

    pub fn new_render_pass(
        &self,
        render_target: &RenderTarget,
        clear_color: Option<(f64, f64, f64, f64)>,
    ) -> Result<RenderPass, ()> {
        unsafe {
            let auto_release_pool: *mut Object = msg_send![class!(NSAutoreleasePool), new];

            let drawable: *mut Object = msg_send![render_target.ca_layer, nextDrawable];

            if drawable == nil {
                return Err(());
            }

            let render_pass_descriptor: *mut Object =
                msg_send![class!(MTLRenderPassDescriptor), new];

            let color_attachments: *mut Object =
                msg_send![render_pass_descriptor, colorAttachments];
            let color_attachment0: *mut Object =
                msg_send![color_attachments, objectAtIndexedSubscript: 0u64];
            let drawable_texture: *mut Object = msg_send![drawable, texture];
            let () = msg_send![color_attachment0, setTexture: drawable_texture];
            let () = msg_send![color_attachment0, setLoadAction: MTLLoadActionLoad::Clear];

            if let Some(clear_color) = clear_color {
                let clear_color = MTLClearColor {
                    red: clear_color.0,
                    green: clear_color.1,
                    blue: clear_color.2,
                    alpha: clear_color.3,
                };
                let () = msg_send![color_attachment0, setClearColor: clear_color];
            }

            Ok(RenderPass {
                render_pass_descriptor,
                drawable,
                auto_release_pool,
            })
        }
    }

    pub fn new_data_buffer<T>(&self, data: &[T]) -> Result<DataBuffer, ()> {
        let data_size = data.len() * std::mem::size_of::<T>();
        let buffer: *mut Object = unsafe {
            msg_send![self.device, newBufferWithBytes: data.as_ptr() length: data_size options: 0]
        };

        Ok(DataBuffer { buffer })
    }

    pub fn new_index_buffer(&self, data: &[u32]) -> Result<IndexBuffer, ()> {
        let data_size = data.len() * std::mem::size_of::<u32>();
        let buffer: *mut Object = unsafe {
            msg_send![self.device, newBufferWithBytes: data.as_ptr() length: data_size options: 0]
        };

        Ok(IndexBuffer { buffer })
    }

    pub fn new_texture(
        &self,
        width: u32,
        height: u32,
        data: &[u8],
        pixel_format: PixelFormat,
    ) -> Result<Texture, ()> {
        let data = bytes_to_bgra(data, pixel_format);
        unsafe {
            let texture_descriptor: *mut Object = msg_send![class!(MTLTextureDescriptor), alloc];
            let texture_descriptor: *mut Object = msg_send![texture_descriptor, init];
            let () = msg_send![texture_descriptor, setPixelFormat: MTLPixelFormatBGRA8Unorm];
            let () = msg_send![texture_descriptor, setWidth: width as u64];
            let () = msg_send![texture_descriptor, setHeight: height as u64];

            let texture: *mut Object =
                msg_send![self.device, newTextureWithDescriptor: texture_descriptor];

            // The region to update is the entire image.
            let region = MTLRegion {
                origin: MTLOrigin { x: 0, y: 0, z: 0 },
                size: MTLSize {
                    width: width as u64,
                    height: height as u64,
                    depth: 1,
                },
            };

            let bytes_per_row = (4 * width) as u64;
            let () = msg_send![texture, replaceRegion:region mipmapLevel:0 withBytes: data.as_ptr() bytesPerRow: bytes_per_row];
            let () = msg_send![texture_descriptor, release];
            Ok(Texture { texture })
        }
    }

    pub fn new_command_buffer(&mut self) -> CommandBuffer {
        let command_buffer: *mut Object = unsafe { msg_send![self.command_queue, commandBuffer] };
        CommandBuffer { command_buffer }
    }

    pub fn commit_command_buffer(&mut self, command_buffer: CommandBuffer) {
        unsafe {
            let () = msg_send![command_buffer.command_buffer, commit];
        }
    }
}
impl CommandBuffer {
    pub fn render_command_encoder<'a>(
        &'a mut self,
        render_pass: &'a RenderPass,
    ) -> RenderCommandEncoder<'a> {
        unsafe {
            let render_encoder: *mut Object = msg_send![
                self.command_buffer,
                renderCommandEncoderWithDescriptor: render_pass.render_pass_descriptor
            ];

            //let () = msg_send![render_encoder, retain];
            RenderCommandEncoder {
                render_encoder,
                command_buffer: self,
                drawable: render_pass.drawable,
            }
        }
    }
}

use crate::pipeline_builder::*;

impl<'a> PipelineBuilder<'a> {
    fn check_properties(
        properties: &mut HashMap<String, PipelineProperty>,
        arguments: *mut Object,
        vertex_or_fragment: VertexOrFragmentProperty,
    ) {
        unsafe {
            let count: u64 = msg_send![arguments, count];
            for i in 0..count {
                let argument: *mut Object = msg_send![arguments, objectAtIndex: i as u64];
                let name: NSString = msg_send![argument, name];
                let active: bool = msg_send![argument, isActive];
                let index: u64 = msg_send![argument, index];
                let type_: u64 = msg_send![argument, type]; // Whether it's a sampler, buffer, etc.

                // This is a buffer or a texture
                if active && (type_ == 0 || type_ == 2) {
                    let name = name.to_string();
                    // Is there not a way to check if a Metal buffer is a single item?
                    // Until there's a way assume that items that start with 'p_' are a single item.
                    // The 'p_' stands for 'property' to map to kgraphics.
                    if name.starts_with("p_") {
                        match type_ {
                            0 => {
                                let buffer_data_type: u64 = msg_send![argument, bufferDataType];

                                let buffer_data_size: u64 = msg_send![argument, bufferDataSize];
                                let property_inner = PropertyInner {
                                    size: buffer_data_size,
                                    index,
                                    vertex_or_fragment,
                                };
                                match buffer_data_type {
                                    MTLDataTypeFloat => {
                                        properties.insert(
                                            name,
                                            PipelineProperty::FloatProperty(property_inner),
                                        );
                                    }
                                    MTLDataTypeFloat2 => {
                                        properties.insert(
                                            name,
                                            PipelineProperty::Vec2Property(property_inner),
                                        );
                                    }
                                    MTLDataTypeFloat3 => {
                                        properties.insert(
                                            name,
                                            PipelineProperty::Vec3Property(property_inner),
                                        );
                                    }
                                    MTLDataTypeFloat4 => {
                                        properties.insert(
                                            name,
                                            PipelineProperty::Vec4Property(property_inner),
                                        );
                                    }
                                    MTLDataTypeFloat4x4 => {
                                        properties.insert(
                                            name,
                                            PipelineProperty::Mat4Property(property_inner),
                                        );
                                    }
                                    _ => {}
                                };
                            }
                            2 => {
                                let property_inner = PropertyInner {
                                    size: 0,
                                    index,
                                    vertex_or_fragment,
                                };
                                properties.insert(
                                    name,
                                    PipelineProperty::TextureProperty(property_inner),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    pub fn build(self) -> Result<Pipeline, String> {
        let mut properties = HashMap::new();
        unsafe {
            let pipeline_descriptor: *mut Object =
                msg_send![class!(MTLRenderPipelineDescriptor), new];
            let () =
                msg_send![pipeline_descriptor, setVertexFunction: self.vertex.unwrap().function];
            let () = msg_send![pipeline_descriptor, setFragmentFunction: self.fragment.unwrap().function];

            // Color attachment, should be made configurable later.
            let color_attachments: *mut Object = msg_send![pipeline_descriptor, colorAttachments];
            let color_attachment0: *mut Object =
                msg_send![color_attachments, objectAtIndexedSubscript: 0u64];
            let () = msg_send![
                color_attachment0,
                setPixelFormat: MTLPixelFormat::BGRA8Unorm
            ];

            let mut err: *mut Object = nil;
            let mut reflection: *mut Object = nil;
            let pipeline_state: *mut Object = msg_send![
                self.g.device,
                newRenderPipelineStateWithDescriptor: pipeline_descriptor
                options: MTLPipelineOptionBufferTypeInfo | MTLPipelineOptionArgumentInfo
                reflection: &mut reflection
                error: &mut err
            ];

            let vertex_arguments: *mut Object = msg_send![reflection, vertexArguments];
            Self::check_properties(
                &mut properties,
                vertex_arguments,
                VertexOrFragmentProperty::Vertex,
            );

            let fragment_arguments: *mut Object = msg_send![reflection, fragmentArguments];
            Self::check_properties(
                &mut properties,
                fragment_arguments,
                VertexOrFragmentProperty::Fragment,
            );

            if err != nil {
                let err_str: *mut Object = msg_send![err, localizedDescription];
                let ns_string = NSString::from_raw(err_str);
                Err(ns_string.to_string())
            } else {
                Ok(Pipeline {
                    properties,
                    pipeline: pipeline_state,
                })
            }
        }
    }
}

pub struct RenderCommandEncoder<'a> {
    render_encoder: *mut Object,
    drawable: *mut Object,
    command_buffer: &'a CommandBuffer,
}

impl<'a> Drop for RenderCommandEncoder<'a> {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.render_encoder, endEncoding];
        }
    }
}

impl<'a> RenderCommandEncoder<'a> {
    pub fn set_pipeline(&mut self, pipeline: &Pipeline) {
        unsafe {
            let () = msg_send![self.render_encoder, setRenderPipelineState: pipeline.pipeline];
        }
    }

    pub fn set_vertex_buffer(&mut self, buffer: &DataBuffer) {
        unsafe {
            let () =
                msg_send![self.render_encoder, setVertexBuffer: buffer.buffer offset: 0 atIndex: 0];
        }
    }

    pub fn draw_triangles_indexed(&mut self, count: u32, index_buffer: &IndexBuffer) {
        unsafe {
            // https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515542-drawindexedprimitives?language=objc
            let () = msg_send![
                self.render_encoder,
                drawIndexedPrimitives: MTLPrimitiveType::Triangle
                indexCount: count as u64 * 3
                indexType: 1 /* MTLIndexTypeUInt32 */
                indexBuffer: index_buffer.buffer
                indexBufferOffset: 0
            ];

            // let () = msg_send![self.render_encoder, drawPrimitives: MTLPrimitiveType::Triangle vertexStart: 0 vertexCount: count * 3 instanceCount: 1];
        }
    }

    unsafe fn set_property<T>(&self, property: Option<&PropertyInner>, t: T) {
        if let Some(property) = property {
            match property.vertex_or_fragment {
                VertexOrFragmentProperty::Vertex => {
                    let () = msg_send![
                        self.render_encoder,
                        setVertexBytes: &t as *const T
                        length: std::mem::size_of::<T>()
                        atIndex: property.index
                    ];
                }
                VertexOrFragmentProperty::Fragment => {
                    let () = msg_send![
                        self.render_encoder,
                        setFragmentBytes: &t as *const T
                        length: std::mem::size_of::<T>()
                        atIndex: property.index
                    ];
                }
            }
        }
    }

    pub fn set_float_property(&mut self, property: &FloatProperty, value: f32) {
        unsafe {
            self.set_property(property.0.as_ref(), value);
        }
    }

    pub fn set_vec2_property(&mut self, property: &Vec2Property, value: (f32, f32)) {
        unsafe {
            self.set_property(property.0.as_ref(), value);
        }
    }

    pub fn set_vec3_property(&mut self, property: &Vec3Property, value: (f32, f32, f32)) {
        unsafe {
            self.set_property(property.0.as_ref(), value);
        }
    }

    pub fn set_vec4_property(&mut self, property: &Vec4Property, value: (f32, f32, f32, f32)) {
        unsafe {
            self.set_property(property.0.as_ref(), value);
        }
    }

    pub fn set_mat4_property(&mut self, property: &Mat4Property, value: &[f32; 16]) {
        unsafe {
            self.set_property(property.0.as_ref(), *value);
        }
    }

    /// The texture unit should be 0 to 16
    /// Perhaps that restriction should be waved later after research.
    pub fn set_texture_property(
        &mut self,
        property: &TextureProperty,
        texture: Option<&Texture>,
        texture_unit: u8,
    ) {
        unsafe {
            if let Some(texture) = texture {
                if let Some(property) = property.0.as_ref() {
                    match property.vertex_or_fragment {
                        VertexOrFragmentProperty::Vertex => {
                            let () = msg_send![
                                self.render_encoder,
                                setVertexTexture: texture.texture
                                atIndex: texture_unit
                            ];
                        }
                        VertexOrFragmentProperty::Fragment => {
                            let () = msg_send![
                                self.render_encoder,
                                setFragmentTexture: texture.texture
                                atIndex: texture_unit
                            ];
                        }
                    }
                }
            }
        }
    }

    /// This should be on the command buffer, not here
    pub fn present(&self) {
        unsafe {
            let () = msg_send![self.command_buffer.command_buffer, presentDrawable: self.drawable];
        }
    }

    pub fn end_encoding(self) {
        unsafe {
            let () = msg_send![self.render_encoder, endEncoding];
        }
    }
}
// Metal specific functions
impl GraphicsContext {
    fn compile_library(&self, source: &str) -> Result<*mut Object, String> {
        unsafe {
            // Needs to be deallocated
            let options: *mut Object = msg_send![class!(MTLCompileOptions), new];
            let mut err: *mut Object = nil;

            let source = NSString::new(&source);

            let library: *mut Object = msg_send![
                self.device,
                newLibraryWithSource: source.raw
                options: options
                error: &mut err
            ];

            let () = msg_send![options, release];
            if library == nil {
                let err_str: *mut Object = msg_send![err, localizedDescription];
                let ns_string = NSString::from_raw(err_str);
                Err(ns_string.to_string())
            } else {
                Ok(library)
            }
        }
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.render_pass_descriptor, release];
            let () = msg_send![self.auto_release_pool, release];
        }
    }
}

pub use objc::runtime::{Object, NO, YES};
pub use objc::*;
pub use std::os::raw::c_uint;

pub const nil: *mut Object = 0 as *mut Object;
pub const kCALayerWidthSizable: c_uint = 1 << 1;
pub const kCALayerHeightSizable: c_uint = 1 << 4;
pub const MTLPipelineOptionArgumentInfo: c_uint = 1 << 0;
pub const MTLPipelineOptionBufferTypeInfo: c_uint = 1 << 1;

pub const MTLDataTypeFloat: u64 = 3;
pub const MTLDataTypeFloat2: u64 = 4;
pub const MTLDataTypeFloat3: u64 = 5;
pub const MTLDataTypeFloat4: u64 = 6;
pub const MTLDataTypeFloat4x4: u64 = 15;

pub const MTLPixelFormatBGRA8Unorm: u64 = 80;

#[repr(C)]

struct MTLRegion {
    origin: MTLOrigin,
    size: MTLSize,
}

#[repr(C)]
struct MTLSize {
    width: u64,
    height: u64,
    depth: u64,
}
#[repr(C)]

struct MTLOrigin {
    x: u64,
    y: u64,
    z: u64,
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    pub fn MTLCreateSystemDefaultDevice() -> *mut Object;
    pub fn MTLCopyAllDevices() -> *mut Object;
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MTLPixelFormat {
    BGRA8Unorm = 80,
    Depth32Float = 252,
    Stencil8 = 253,
    Depth24Unorm_Stencil8 = 255,
    Depth32Float_Stencil8 = 260,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MTLLoadActionLoad {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

pub struct NSString {
    pub raw: *mut Object,
}

impl NSString {
    pub fn new(string: &str) -> Self {
        unsafe {
            let raw: *mut Object = msg_send![class!(NSString), alloc];
            let raw: *mut Object = msg_send![
                raw,
                initWithBytes: string.as_ptr()
                length: string.len()
                encoding:UTF8_ENCODING as *mut Object
            ];

            Self { raw }
        }
    }

    pub fn from_raw(raw: *mut Object) -> Self {
        Self { raw }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let utf8_string: *const std::os::raw::c_uchar = msg_send![self.raw, UTF8String];
            let utf8_len: usize = msg_send![self.raw, lengthOfBytesUsingEncoding: UTF8_ENCODING];
            let slice = std::slice::from_raw_parts(utf8_string, utf8_len);
            std::str::from_utf8_unchecked(slice).to_owned()
        }
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.raw, release];
        }
    }
}

pub const UTF8_ENCODING: usize = 4;

#[repr(C)]
pub struct MTLClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MTLPrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}
use std::os::raw::c_double;

pub type CGFloat = c_double;

#[repr(C)]
#[derive(Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        Self { width, height }
    }
}
