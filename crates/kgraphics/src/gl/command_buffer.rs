use super::*;
pub struct CommandBuffer {
    pub(super) actions: Vec<CommandBufferAction>,
    pub(super) uniforms: BumpAllocator,
}

pub struct RenderPass<'a> {
    command_buffer: &'a mut CommandBuffer,
}

#[derive(Clone)]
pub(super) enum CommandBufferAction {
    Clear((f32, f32, f32, f32)),
    // None represents the default framebuffer
    BindFramebuffer(Framebuffer),
    ChangePipeline(Pipeline),
    SetVertexAttribute((VertexAttributeInfo, Option<gl_native::Buffer>)),
    SetIndexBuffer(IndexBuffer),
    SetFloatUniform((UniformLocation, BumpHandle)),
    SetIntUniform((UniformLocation, BumpHandle)),
    SetVec2Uniform((UniformLocation, BumpHandle)),
    SetVec3Uniform((UniformLocation, BumpHandle)),
    SetVec4Uniform((UniformLocation, BumpHandle)),
    SetMat4Uniform((UniformLocation, BumpHandle)),
    SetTextureUnit((UniformLocation, u8, Option<gl_native::TextureNative>)),
    SetViewport((u32, u32, u32, u32)),
    DrawTriangles(u32),
    DrawTriangleArrays(u32),
    Present,
}

#[derive(Clone)]
pub(super) struct FrameBufferBinding {
    pub(super) color: Option<gl_native::TextureNative>,
    pub(super) depth: Option<gl_native::TextureNative>,
    pub(super) stencil: Option<gl_native::TextureNative>,
}
impl CommandBuffer {
    pub(crate) fn new() -> Self {
        Self {
            actions: Vec::new(),
            uniforms: BumpAllocator::new(),
        }
    }
}

impl CommandBuffer {
    pub(crate) fn clear(&mut self) {
        self.actions.clear();
        self.uniforms.clear();
    }
}

impl CommandBufferTrait for CommandBuffer {
    /// Gets the number of actions encoded in the `CommandBuffer`
    fn len(&self) -> usize {
        self.actions.len()
    }

    fn begin_render_pass_with_framebuffer<'a>(
        &'a mut self,
        framebuffer: &Framebuffer,
        clear_color: Option<(f32, f32, f32, f32)>,
    ) -> RenderPass<'a> {
        self.actions
            .push(CommandBufferAction::BindFramebuffer(framebuffer.clone()));
        if let Some((r, g, b, a)) = clear_color {
            self.actions.push(CommandBufferAction::Clear((
                r as f32, g as f32, b as f32, a as f32,
            )));
        }
        RenderPass {
            command_buffer: self,
        }
    }

    /// If the color_texture binds to the default framebuffer then
    /// all textures will bind to the default framebuffer.
    fn begin_render_pass<'a>(
        &'a mut self,
        color_texture: Option<&Texture>,
        depth_texture: Option<&Texture>,
        stencil_texture: Option<&Texture>,
        clear_color: Option<(f32, f32, f32, f32)>,
    ) -> RenderPass<'a> {
        /*
        match color_texture {
            Some(Texture {
                texture_type: TextureType::DefaultFramebuffer,
            }) => {
                self.actions
                    .push(CommandBufferAction::BindFramebuffer(None));
            }
            _ => {
                let color = color_texture.map(|t| match t.texture_type {
                    TextureType::Texture(t) => t,
                    TextureType::DefaultFramebuffer => {
                        panic!("Depth texture cannot be bound to the default framebuffer")
                    }
                });

                let depth = depth_texture.map(|t| match t.texture_type {
                    TextureType::Texture(t) => t,
                    TextureType::DefaultFramebuffer => {
                        panic!("Depth texture cannot be bound to the default framebuffer")
                    }
                });
                let stencil = stencil_texture.map(|t| match t.texture_type {
                    TextureType::Texture(t) => t,
                    TextureType::DefaultFramebuffer => {
                        panic!("Stencil texture cannot be bound to the default framebuffer")
                    }
                });
                self.actions.push(CommandBufferAction::BindFramebuffer(Some(
                    FrameBufferBinding {
                        color,
                        depth,
                        stencil,
                    },
                )));
            }
        }
        if let Some((r, g, b, a)) = clear_color {
            self.actions.push(CommandBufferAction::Clear((
                r as f32, g as f32, b as f32, a as f32,
            )));
        }
        RenderPass {
            command_buffer: self,
        }
        */
        todo!()
    }
}

impl<'a> RenderPassTrait for RenderPass<'a> {
    fn set_pipeline(&mut self, pipeline: &Pipeline) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::ChangePipeline(pipeline.clone()))
    }

    /// Vertex attributes are arrays of data for each vertex.
    fn set_vertex_attribute<T>(
        &mut self,
        vertex_attribute: &VertexAttribute<T>,
        buffer: Option<&DataBuffer<T>>,
    ) {
        if let Some(info) = vertex_attribute.info.clone() {
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetVertexAttribute((
                    info,
                    buffer.map(|b| b.buffer),
                )))
        }
    }

    fn set_float_property(&mut self, property: &FloatProperty, value: f32) {
        if let Some(uniform_location) = property.location {
            let handle = self.command_buffer.uniforms.push(value);
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetFloatUniform((
                    uniform_location,
                    handle,
                )));
        }
    }

    fn set_int_property(&mut self, property: &IntProperty, value: i32) {
        if let Some(uniform_location) = property.location {
            let handle = self.command_buffer.uniforms.push(value);
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetIntUniform((
                    uniform_location,
                    handle,
                )));
        }
    }

    fn set_vec2_property(&mut self, property: &Vec2Property, value: (f32, f32)) {
        if let Some(uniform_location) = property.location {
            let handle = self.command_buffer.uniforms.push(value);
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetVec2Uniform((
                    uniform_location,
                    handle,
                )));
        }
    }

    fn set_vec3_property(&mut self, property: &Vec3Property, value: (f32, f32, f32)) {
        if let Some(uniform_location) = property.location {
            let handle = self.command_buffer.uniforms.push(value);
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetVec3Uniform((
                    uniform_location,
                    handle,
                )));
        }
    }

    fn set_vec4_property(&mut self, property: &Vec4Property, value: (f32, f32, f32, f32)) {
        if let Some(uniform_location) = property.location {
            let handle = self.command_buffer.uniforms.push(value);
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetVec4Uniform((
                    uniform_location,
                    handle,
                )));
        }
    }

    fn set_mat4_property(&mut self, property: &Mat4Property, value: &[f32; 16]) {
        if let Some(uniform_location) = property.location {
            let handle = self.command_buffer.uniforms.push(*value);
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetMat4Uniform((
                    uniform_location,
                    handle,
                )));
        }
    }

    fn set_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::SetViewport((x, y, width, height)))
    }

    /// The texture unit should be 0 to 16
    /// Perhaps that restriction should be waved later after research.
    fn set_texture_property(
        &mut self,
        property: &TextureProperty,
        texture: Option<&Texture>,
        texture_unit: u8,
    ) {
        let texture = texture.map(|t| match t.texture_type {
            TextureType::Texture(t) => t,
            TextureType::DefaultFramebuffer => panic!("Cannot update default framebuffer"),
        });
        // The minimum number of texture units is 16
        // In the future this could cache bindings and if they're already the same this command could be ignored
        debug_assert!(texture_unit < 16);
        if let Some(uniform_location) = property.location {
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetTextureUnit((
                    uniform_location,
                    texture_unit,
                    texture,
                )))
        } else {
            // println!("WARNING: Binding texture to non-existent uniform")
        }
    }

    fn draw_triangles(&mut self, count: u32, buffer: &IndexBuffer) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::SetIndexBuffer(buffer.clone()));
        self.command_buffer
            .actions
            .push(CommandBufferAction::DrawTriangles(count))
    }

    fn draw_triangles_without_buffer(&mut self, count: u32) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::DrawTriangleArrays(count))
    }
}

impl<'a> Drop for RenderPass<'a> {
    fn drop(&mut self) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::Present);
    }
}
