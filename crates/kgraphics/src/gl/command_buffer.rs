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
    SetUniformBlock((UniformBlockInfo, Option<gl_native::Buffer>, usize, usize)),
    SetVertexAttribute((VertexAttributeInfo, Option<gl_native::Buffer>, bool)),
    SetVertexAttributeToConstant {
        attribute: VertexAttributeInfo,
        length: u8,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    },
    SetIndexBuffer(IndexBuffer),
    SetFloatUniform((UniformLocation, BumpHandle)),
    SetIntUniform((UniformLocation, BumpHandle)),
    SetVec2Uniform((UniformLocation, BumpHandle)),
    SetVec3Uniform((UniformLocation, BumpHandle)),
    SetVec4Uniform((UniformLocation, BumpHandle)),
    SetMat4Uniform((UniformLocation, BumpHandle)),
    SetTextureUnit((UniformLocation, u8, Option<gl_native::TextureNative>)),
    SetTextureUnitToCubeMap((UniformLocation, u8, Option<gl_native::TextureNative>)),
    SetViewport((u32, u32, u32, u32)),
    DrawTriangles(u32),
    DrawTriangleArrays(u32),
    DrawTrianglesInstanced(u32, u32),
    SetDepthMask(bool),
    BlitFramebuffer {
        target: Framebuffer,
        dest_x: u32,
        dest_y: u32,
        dest_width: u32,
        dest_height: u32,
        source_x: u32,
        source_y: u32,
        source_width: u32,
        source_height: u32,
    },
    Present,
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
    fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    fn begin_render_pass_with_framebuffer<'a>(
        &'a mut self,
        framebuffer: &Framebuffer,
        clear_color: Option<(f32, f32, f32, f32)>,
    ) -> RenderPass<'a> {
        self.actions
            .push(CommandBufferAction::BindFramebuffer(*framebuffer));

        // This is before Clear otherwise the Clear doesn't clear depth.
        self.actions.push(CommandBufferAction::SetDepthMask(true));

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
        _color_texture: Option<&Texture>,
        _depth_texture: Option<&Texture>,
        _stencil_texture: Option<&Texture>,
        _clear_color: Option<(f32, f32, f32, f32)>,
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
    fn present(&mut self) {
        self.actions.push(CommandBufferAction::Present);
    }
}

impl<'a> RenderPassTrait for RenderPass<'a> {
    fn set_pipeline(&mut self, pipeline: &Pipeline) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::ChangePipeline(pipeline.clone()))
    }

    fn set_uniform_block<T>(
        &mut self,
        uniform_block: &UniformBlock<T>,
        buffer: Option<&DataBuffer<T>>,
    ) {
        if let Some(info) = uniform_block.info.clone() {
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetUniformBlock((
                    info,
                    buffer.map(|b| b.buffer),
                    0,
                    buffer.map_or(0, |b| b.len),
                )))
        }
    }

    fn set_instance_attribute<T>(
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
                    true,
                )))
        }
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
                    false,
                )))
        }
    }

    /// Vertex attributes are arrays of data for each vertex.
    fn set_vertex_attribute_to_constant<T>(
        &mut self,
        vertex_attribute: &VertexAttribute<T>,
        value: &[f32],
    ) {
        if let Some(info) = vertex_attribute.info.clone() {
            let length = value.len() as u8;
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetVertexAttributeToConstant {
                    attribute: info,
                    length,
                    x: value.get(0).cloned().unwrap_or(0.0),
                    y: value.get(0).cloned().unwrap_or(0.0),
                    z: value.get(0).cloned().unwrap_or(0.0),
                    w: value.get(0).cloned().unwrap_or(0.0),
                })
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
            TextureType::RenderBuffer(..) => {
                panic!(
                    "Cannot bind a texture with MSAA samples as a property. Resolve it to another texture first."
                )
            }
            TextureType::DefaultFramebuffer => panic!("Cannot update default framebuffer"),
            TextureType::CubeMap { .. } => todo!(),
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

    /// The texture unit should be 0 to 16
    /// Perhaps that restriction should be waved later after research.
    fn set_cube_map_property(
        &mut self,
        property: &CubeMapProperty,
        texture: Option<&CubeMap>,
        texture_unit: u8,
    ) {
        // The minimum number of texture units is 16
        // In the future this could cache bindings and if they're already the same this command could be ignored
        debug_assert!(texture_unit < 16);
        if let Some(uniform_location) = property.location {
            self.command_buffer
                .actions
                .push(CommandBufferAction::SetTextureUnitToCubeMap((
                    uniform_location,
                    texture_unit,
                    texture.map(|t| t.texture),
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

    fn draw_triangles_instanced(&mut self, count: u32, buffer: &IndexBuffer, instances: u32) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::SetIndexBuffer(buffer.clone()));
        self.command_buffer
            .actions
            .push(CommandBufferAction::DrawTrianglesInstanced(
                count, instances,
            ))
    }

    fn set_depth_mask(&mut self, depth_mask: bool) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::SetDepthMask(depth_mask))
    }
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
    ) {
        self.command_buffer
            .actions
            .push(CommandBufferAction::BlitFramebuffer {
                target: *target,
                source_x,
                source_y,
                source_width,
                source_height,
                dest_x,
                dest_y,
                dest_width,
                dest_height,
            })
    }
}
