use std::ffi::CString;

pub use gl33::gl_enumerations::*;
pub use gl33::*;

pub type GLInt = std::os::raw::c_uint;

#[derive(Clone, PartialEq, Debug, Copy)]

pub struct Shader(GLInt);
#[derive(Clone, PartialEq, Debug, Copy)]

pub struct Program(GLInt);
#[derive(Clone, PartialEq, Debug, Copy)]

pub struct Buffer(GLInt);
#[derive(Clone, PartialEq, Debug, Copy)]

pub struct TextureNative(GLInt);

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct RenderBufferNative(GLInt);

#[derive(Clone, PartialEq, Debug, Copy)]

pub struct UniformLocation(GLInt);

#[derive(Clone, PartialEq, Debug, Copy)]

pub struct VertexArray(GLInt);

#[derive(Clone, PartialEq, Debug, Copy, Default)]
pub struct Framebuffer(pub(crate) GLInt);

pub struct GL {
    pub(crate) gl: gl33::GlFns,
}

pub struct ActiveUniform {
    pub size_members: i32,
    pub uniform_type: GLenum,
    pub name: String,
}

#[derive(Debug)]
pub struct ActiveAttribute {
    pub size_members: i32,
    pub attribute_type: GLenum,
    pub name: String,
}

impl GL {
    pub unsafe fn get_active_uniforms(&self, program: Program) -> u32 {
        let mut count = 0;
        self.gl
            .GetProgramiv(program.0, GL_ACTIVE_UNIFORMS, &mut count);
        count as u32
    }

    pub unsafe fn get_active_uniform(&self, program: Program, index: u32) -> Option<ActiveUniform> {
        let mut uniform_max_length = 0;
        self.gl.GetProgramiv(
            program.0,
            GL_ACTIVE_UNIFORM_MAX_LENGTH,
            &mut uniform_max_length,
        );

        let mut name: Vec<u8> = vec![0; uniform_max_length as usize];
        let mut length = 0;
        let mut size_members = 0;
        let mut uniform_type = GLenum(0);
        self.gl.GetActiveUniform(
            program.0,
            index,
            uniform_max_length,
            &mut length,
            &mut size_members,
            &mut uniform_type,
            name.as_mut_ptr(),
        );
        name.truncate(length as usize);
        let name = String::from_utf8(name).unwrap();

        Some(ActiveUniform {
            size_members,
            uniform_type,
            name,
        })
    }

    pub unsafe fn get_uniform_location(
        &self,
        program: Program,
        name: &str,
    ) -> Option<UniformLocation> {
        let name = CString::new(name).unwrap();
        let uniform_location = self
            .gl
            .GetUniformLocation(program.0, name.as_ptr() as *const u8);
        if uniform_location < 0 {
            None
        } else {
            Some(UniformLocation(uniform_location as u32))
        }
    }

    pub unsafe fn get_active_attributes(&self, program: Program) -> u32 {
        let mut count = 0;
        self.gl
            .GetProgramiv(program.0, GL_ACTIVE_ATTRIBUTES, &mut count);
        count as u32
    }

    pub unsafe fn get_active_attribute(
        &self,
        program: Program,
        index: u32,
    ) -> Option<ActiveAttribute> {
        let mut attribute_max_length = 0;
        self.gl.GetProgramiv(
            program.0,
            GL_ACTIVE_ATTRIBUTE_MAX_LENGTH,
            &mut attribute_max_length,
        );
        let mut name: Vec<u8> = vec![0; attribute_max_length as usize];
        let mut length = 0;
        let mut size_members = 0;
        let mut attribute_type = GLenum(0);
        self.gl.GetActiveAttrib(
            program.0,
            index,
            attribute_max_length,
            &mut length,
            &mut size_members,
            &mut attribute_type,
            name.as_mut_ptr(),
        );

        name.truncate(length as usize);
        let name = String::from_utf8(name).unwrap();

        Some(ActiveAttribute {
            name,
            size_members,
            attribute_type,
        })
    }

    pub fn get_attribute_location(&self, program: Program, name: &str) -> i32 {
        let c_string = CString::new(name).unwrap();
        unsafe {
            self.gl
                .GetAttribLocation(program.0, c_string.as_ptr() as *const u8)
        }
    }

    pub unsafe fn create_vertex_array(&self) -> Result<VertexArray, String> {
        let mut vertex_array = 0;
        self.gl.GenVertexArrays(1, &mut vertex_array);
        Ok(VertexArray(vertex_array))
    }

    pub unsafe fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.gl.Viewport(x, y, width, height);
    }

    pub unsafe fn bind_vertex_array(&self, vertex_array: Option<VertexArray>) {
        self.gl.BindVertexArray(vertex_array.map_or(0, |v| v.0));
    }

    pub unsafe fn bind_buffer(&self, target: GLenum, buffer: Option<Buffer>) {
        self.gl.BindBuffer(target, buffer.map_or(0, |v| v.0));
    }

    pub unsafe fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32) {
        self.gl.BufferData(
            GLenum(target),
            data.len() as isize,
            data.as_ptr() as *const std::ffi::c_void,
            GLenum(usage),
        );
    }

    pub unsafe fn create_buffer(&self) -> Result<Buffer, String> {
        let mut buffer = 0;
        self.gl.GenBuffers(1, &mut buffer);
        Ok(Buffer(buffer))
    }

    pub unsafe fn delete_buffer(&self, buffer: Buffer) {
        self.gl.DeleteBuffers(1, &buffer.0);
    }

    pub unsafe fn bind_texture(&self, target: GLenum, texture: Option<TextureNative>) {
        self.gl.BindTexture(target, texture.map_or(0, |v| v.0));
    }

    pub unsafe fn tex_parameter_i32(&self, target: GLenum, parameter: GLenum, value: i32) {
        self.gl.TexParameteri(target, parameter, value);
    }

    /*
    pub unsafe fn tex_parameter_f32_slice(&self, target: GLenum, parameter: GLenum, value: &[f32]) {
        self.gl.TexParameterfv(target, parameter, value.as_ptr());
    }
    */

    #[allow(clippy::too_many_arguments)]
    pub unsafe fn tex_image_2d(
        &self,
        target: GLenum,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: GLenum,
        type_: GLenum,
        pixels: Option<&[u8]>,
    ) {
        self.gl.TexImage2D(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            type_,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub unsafe fn renderbuffer_storage_multisample(
        &self,
        target: GLenum,
        samples: i32,
        internal_format: i32,
        width: i32,
        height: i32,
    ) {
        self.gl.RenderbufferStorageMultisample(
            target,
            samples,
            GLenum(internal_format as u32),
            width,
            height,
        );
    }

    pub unsafe fn generate_mipmap(&self, target: GLenum) {
        self.gl.GenerateMipmap(target);
    }

    pub unsafe fn create_texture(&self) -> Result<TextureNative, String> {
        let mut name = 0;
        self.gl.GenTextures(1, &mut name);
        Ok(TextureNative(name))
    }

    pub unsafe fn create_renderbuffer(&self) -> RenderBufferNative {
        let mut name = 0;
        self.gl.GenRenderbuffers(1, &mut name);
        RenderBufferNative(name)
    }

    pub unsafe fn delete_texture(&self, texture: TextureNative) {
        self.gl.DeleteTextures(1, &texture.0);
    }

    pub unsafe fn delete_renderbuffer(&self, renderbuffer: RenderBufferNative) {
        self.gl.DeleteRenderbuffers(1, &renderbuffer.0);
    }

    pub unsafe fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.gl.ClearColor(red, green, blue, alpha);
    }

    pub unsafe fn clear(&self, mask: u32) {
        self.gl.Clear(GLbitfield(mask));
    }

    pub unsafe fn bind_framebuffer(&self, target: GLenum, framebuffer: Framebuffer) {
        self.gl.BindFramebuffer(target, framebuffer.0);
    }

    pub unsafe fn blit_framebuffer(
        &self,
        source_x: u32,
        source_y: u32,
        source_width: u32,
        source_height: u32,
        dest_x: u32,
        dest_y: u32,
        dest_width: u32,
        dest_height: u32,
    ) {
        self.gl.BlitFramebuffer(
            source_x as i32,
            source_y as i32,
            source_width as i32,
            source_height as i32,
            dest_x as i32,
            dest_y as i32,
            dest_width as i32,
            dest_height as i32,
            GL_COLOR_BUFFER_BIT,
            GL_LINEAR,
        );
    }

    pub unsafe fn create_framebuffer(&self) -> Result<Framebuffer, String> {
        let mut framebuffer = 0;
        self.gl.GenFramebuffers(1, &mut framebuffer);
        Ok(Framebuffer(framebuffer))
    }

    pub unsafe fn framebuffer_texture_2d(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture_target: GLenum,
        texture: Option<TextureNative>,
        level: i32,
    ) {
        self.gl.FramebufferTexture2D(
            target,
            attachment,
            texture_target,
            texture.map_or(0, |v| v.0),
            level,
        );
    }

    pub unsafe fn framebuffer_renderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture_target: GLenum,
        renderbuffer: RenderBufferNative,
    ) {
        self.gl
            .FramebufferRenderbuffer(target, attachment, texture_target, renderbuffer.0);
    }

    pub unsafe fn delete_framebuffer(&self, framebuffer: Framebuffer) {
        self.gl.DeleteFramebuffers(1, &framebuffer.0);
    }

    pub unsafe fn use_program(&self, program: Option<Program>) {
        self.gl.UseProgram(program.map_or(0, |v| v.0));
    }

    pub unsafe fn enable(&self, parameter: GLenum) {
        self.gl.Enable(parameter);
    }

    pub unsafe fn disable(&self, parameter: GLenum) {
        self.gl.Disable(parameter);
    }

    pub unsafe fn depth_func(&self, parameter: GLenum) {
        self.gl.DepthFunc(parameter);
    }

    pub unsafe fn blend_func(&self, source: GLenum, destination: GLenum) {
        self.gl.BlendFunc(source, destination);
    }

    pub unsafe fn cull_face(&self, parameter: GLenum) {
        self.gl.CullFace(parameter);
    }

    pub unsafe fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: GLenum,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        self.gl.VertexAttribPointer(
            index,
            size,
            data_type,
            normalized as u8,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    pub unsafe fn vertex_attrib(&self, index: u32, length: u8, values: &[f32; 4]) {
        match length {
            1 => self.gl.VertexAttrib1f(index, values[0]),
            2 => self.gl.VertexAttrib2f(index, values[0], values[1]),
            3 => self
                .gl
                .VertexAttrib3f(index, values[0], values[1], values[2]),
            4 => self
                .gl
                .VertexAttrib4f(index, values[0], values[1], values[2], values[3]),
            _ => unreachable!(),
        }
    }

    pub unsafe fn disable_vertex_attrib_array(&self, index: u32) {
        self.gl.DisableVertexAttribArray(index);
    }

    pub unsafe fn enable_vertex_attrib_array(&self, index: u32) {
        self.gl.EnableVertexAttribArray(index);
    }

    pub unsafe fn uniform_1_i32(&self, location: Option<UniformLocation>, x: i32) {
        if let Some(location) = location {
            self.gl.Uniform1i(location.0 as i32, x);
        }
    }

    pub unsafe fn uniform_1_f32(&self, location: Option<UniformLocation>, x: f32) {
        if let Some(location) = location {
            self.gl.Uniform1f(location.0 as i32, x);
        }
    }

    pub unsafe fn uniform_2_f32(&self, location: Option<UniformLocation>, x: f32, y: f32) {
        if let Some(location) = location {
            self.gl.Uniform2f(location.0 as i32, x, y);
        }
    }

    pub unsafe fn uniform_3_f32(&self, location: Option<UniformLocation>, x: f32, y: f32, z: f32) {
        if let Some(location) = location {
            self.gl.Uniform3f(location.0 as i32, x, y, z);
        }
    }

    pub unsafe fn uniform_4_f32(
        &self,
        location: Option<UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        if let Some(location) = location {
            self.gl.Uniform4f(location.0 as i32, x, y, z, w);
        }
    }

    pub unsafe fn uniform_matrix_4_f32_slice(
        &self,
        location: Option<UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        if let Some(location) = location {
            self.gl.UniformMatrix4fv(
                location.0 as i32,
                v.len() as i32 / 16,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    pub unsafe fn active_texture(&self, unit: u32) {
        self.gl.ActiveTexture(GLenum(unit));
    }

    pub unsafe fn shader_source(&self, shader: Shader, source: &str) {
        self.gl
            .ShaderSource(shader.0, 1, &(source.as_ptr()), &(source.len() as i32));
    }

    pub unsafe fn compile_shader(&self, shader: Shader) {
        self.gl.CompileShader(shader.0);
    }

    pub unsafe fn draw_elements(
        &self,
        mode: GLenum,
        count: i32,
        element_type: GLenum,
        offset: i32,
    ) {
        self.gl
            .DrawElements(mode, count, element_type, offset as *const std::ffi::c_void);
    }

    pub unsafe fn create_shader(&self, shader_type: GLenum) -> Result<Shader, String> {
        Ok(Shader(self.gl.CreateShader(shader_type)))
    }

    pub unsafe fn get_shader_compile_status(&self, shader: Shader) -> bool {
        let mut status = 0;
        self.gl
            .GetShaderiv(shader.0, GL_COMPILE_STATUS, &mut status);
        1 == status
    }

    pub unsafe fn get_shader_info_log(&self, shader: Shader) -> String {
        let mut length = 0;
        self.gl
            .GetShaderiv(shader.0, GL_INFO_LOG_LENGTH, &mut length);
        if length > 0 {
            let mut log: Vec<u8> = vec![0; length as usize];

            self.gl
                .GetShaderInfoLog(shader.0, length, &mut length, log.as_mut_ptr());
            log.truncate(length as usize);
            String::from_utf8(log).unwrap()
        } else {
            String::from("")
        }
    }

    pub unsafe fn create_program(&self) -> Result<Program, String> {
        Ok(Program(self.gl.CreateProgram()))
    }

    pub unsafe fn attach_shader(&self, program: Program, shader: Shader) {
        self.gl.AttachShader(program.0, shader.0);
    }

    pub unsafe fn link_program(&self, program: Program) {
        self.gl.LinkProgram(program.0);
    }

    pub unsafe fn get_program_link_status(&self, program: Program) -> bool {
        let mut status = 0;
        self.gl.GetProgramiv(program.0, GL_LINK_STATUS, &mut status);
        1 == status
    }

    pub unsafe fn get_program_info_log(&self, program: Program) -> String {
        let mut length = 0;
        self.gl
            .GetProgramiv(program.0, GL_INFO_LOG_LENGTH, &mut length);
        if length > 0 {
            let mut log: Vec<u8> = vec![0; length as usize];

            self.gl
                .GetProgramInfoLog(program.0, length, &mut length, log.as_mut_ptr());
            log.truncate(length as usize);
            String::from_utf8(log).unwrap()
        } else {
            String::from("")
        }
    }

    pub unsafe fn draw_arrays(&self, mode: GLenum, first: i32, count: i32) {
        self.gl.DrawArrays(mode, first, count);
    }
    pub unsafe fn set_depth_mask(&self, value: bool) {
        self.gl.DepthMask(if value { 1 } else { 0 })
    }
}
