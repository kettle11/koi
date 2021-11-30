var gl = null;
var canvas = null;
var gl_web_object = {
    new(antialias) {
        canvas = document
            .getElementById("canvas");
        gl =
            canvas.getContext('webgl2', {
                alpha: false,
                desynchronized: false,
                antialias: antialias !== 0,
                depth: true
            });

        if (gl === null) {
            console.log("Could not initialize WebGL2 canvas!");
        }

        // Get an extension to allow linearly filtering float textures.
        gl.getExtension('OES_texture_float_linear');
        gl.getExtension('EXT_color_buffer_half_float');

        // Setup some stuff that won't change
        gl.enable(gl.DEPTH_TEST);
        gl.enable(gl.TEXTURE_CUBE_MAP_SEAMLESS);

        let vertex_array_object = gl.createVertexArray();
        gl.bindVertexArray(vertex_array_object);

    },
    resize(width, height) {
        canvas.width = width;
        canvas.height = height;
    },
    new_vertex_function(shader_source) {
        var shader = gl.createShader(gl.VERTEX_SHADER);
        gl.shaderSource(shader, shader_source);
        gl.compileShader(shader);
        // These errors should be returned somehow for `kgraphics` to handle
        let message = gl.getShaderInfoLog(shader);
        if (message.length > 0) {
            console.error(message);
        }
        return shader;
    },
    new_fragment_function(shader_source) {
        var shader = gl.createShader(gl.FRAGMENT_SHADER);
        gl.shaderSource(shader, shader_source);
        gl.compileShader(shader);
        // These errors should be returned somehow for `kgraphics` to handle
        let message = gl.getShaderInfoLog(shader);
        if (message.length > 0) {
            console.error(message);
        }
        return shader;
    },
    delete_buffer(data_buffer) {
        gl.deleteBuffer(data_buffer);
    },
    new_data_buffer(data_ptr, data_length) {
        const data = new Uint8Array(self.kwasm_memory.buffer, data_ptr, data_length);
        let buffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
        gl.bufferData(gl.ARRAY_BUFFER, data, gl.STATIC_DRAW);
        return buffer;
    },
    new_index_buffer(data_ptr, data_length) {
        const data = new Uint32Array(self.kwasm_memory.buffer, data_ptr, data_length);
        let buffer = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffer);
        gl.bufferData(
            gl.ELEMENT_ARRAY_BUFFER,
            data,
            gl.STATIC_DRAW,
        );
        return buffer;
    },
    update_texture(texture_index, inner_pixel_format, width, height, pixel_format, type_, data_ptr, data_length, min, mag, mipmaps, wrapping_horizontal, wrapping_vertical) {
        /*
         console.log("UPDATE TEXTURE: width" + width + " height:" + height);
 
         console.log("width: " + width);
         console.log("height: " + width);
 
         console.log("INNER PIXEL FORMAT: " + inner_pixel_format);
         console.log("PIXEL FORMAT: " + pixel_format);
         console.log("TYPE: " + type_);
 
         console.log("min: " + min);
         console.log("mag: " + mag);
 
         console.log("wrapping_horizontal: " + wrapping_horizontal);
         console.log("wrapping_vertical: " + wrapping_vertical);
 
         console.log("data: " + data_ptr);
         console.log("data_length: " + data_length);
        console.log("DATA: " + data);
         */
        let data = null;
        if (data_ptr !== 0) {
            if (type_ == 5126) {
                // If it's a floating point array
                data = new Float32Array(self.kwasm_memory.buffer, data_ptr, data_length / 4);
            } else {
                data = new Uint8Array(self.kwasm_memory.buffer, data_ptr, data_length);
            }
        }


        let texture = self.kwasm_get_object(texture_index);
        gl.bindTexture(gl.TEXTURE_2D, texture);

        gl.texImage2D(
            gl.TEXTURE_2D,
            0, /* mip level */
            inner_pixel_format,
            width,
            height,
            0, /* border */
            pixel_format,
            type_,
            data
        );

        gl.texParameteri(
            gl.TEXTURE_2D,
            gl.TEXTURE_MIN_FILTER,
            min
        );
        gl.texParameteri(
            gl.TEXTURE_2D,
            gl.TEXTURE_MAG_FILTER,
            mag
        );

        gl.texParameteri(
            gl.TEXTURE_2D,
            gl.TEXTURE_WRAP_S,
            wrapping_horizontal
        );
        gl.texParameteri(
            gl.TEXTURE_2D,
            gl.TEXTURE_WRAP_T,
            wrapping_vertical
        );

        /* Border color should be set here too */

        if (mipmaps !== 0) {
            gl.generateMipmap(gl.TEXTURE_2D);
        }

    },
    new_texture() {
        let texture = gl.createTexture();
        return texture;
    },
    delete_texture(texture) {
        gl.delete_texture(texture);
    },
    new_program(vertex_shader, fragment_shader) {
        let program = gl.createProgram();
        gl.attachShader(program, vertex_shader);
        gl.attachShader(program, fragment_shader);
        gl.linkProgram(program);

        if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
            var info = gl.getProgramInfoLog(program);
            console.error('Could not compile WebGL program. \n\n' + info);
            return null;
        } else {
            return program;
        }
    },
    get_uniform_name_and_type(program_index, uniform_index) {
        let program = self.kwasm_get_object(program_index);
        let active_info = gl.getActiveUniform(program, uniform_index);
        self.kwasm_pass_string_to_client(active_info.name);
        return active_info.type;
    },
    get_uniform_location(program, name) {
        let result = gl.getUniformLocation(program, name);
        return result;
    },
    get_program_parameter(program_index, parameter) {
        let program = self.kwasm_get_object(program_index);
        return gl.getProgramParameter(program, parameter);
    },
    get_attribute_name_and_type(program_index, attribute_index) {
        let program = self.kwasm_get_object(program_index);
        let info = gl.getActiveAttrib(program, attribute_index);
        self.kwasm_pass_string_to_client(info.name);
        return info.type;
    },
    get_attribute_location(program, name) {
        let location = gl.getAttribLocation(program, name);
        return location;
    },
    get_multiview_supported() {
        // From here: https://developer.oculus.com/documentation/web/web-multiview/
        let ext = gl.getExtension('OCULUS_multiview');
        if (ext) {
            console.log("OCULUS_multiview extension is supported");
            return 2;
        }
        else {
            console.log("OCULUS_multiview extension is NOT supported");
            ext = gl.getExtension('OVR_multiview2');
            if (ext) {
                console.log("OVR_multiview2 extension is supported");
                return 1;
            }
            else {
                console.log("Neither OCULUS_multiview nor OVR_multiview2 extensions are supported");
                return 0;
            }
        }
    },
    run_command_buffer(commands_ptr, commands_length, f32_data_ptr, f32_data_length, u32_data_ptr, u32_data_length) {
        const commands = new Uint8Array(self.kwasm_memory.buffer, commands_ptr, commands_length);
        //: " + commands_length);
        const f32_data = new Float32Array(self.kwasm_memory.buffer, f32_data_ptr, f32_data_length);
        const u32_data = new Uint32Array(self.kwasm_memory.buffer, u32_data_ptr, u32_data_length);

        let length = commands.length;
        let f32_offset = 0;
        let u32_offset = 0;
        //let temp_framebuffer = null;

        for (i = 0; i < length; i++) {
            //console.log("COMMAND " + commands[i]);
            switch (commands[i]) {
                case 0: {
                    // Clear
                    let r = f32_data[f32_offset++];
                    let g = f32_data[f32_offset++];
                    let b = f32_data[f32_offset++];
                    let a = f32_data[f32_offset++];
                    gl.clearColor(r, g, b, a);
                    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
                    break;
                }
                case 1: {
                    // BindFramebuffer
                    let framebuffer_index = u32_data[u32_offset++];
                    let color_framebuffer_index = u32_data[u32_offset++];
                    let depth_framebuffer_index = u32_data[u32_offset++];
                    let stencil_framebuffer_index = u32_data[u32_offset++];

                    let framebuffer_object = self.kwasm_get_object(framebuffer_index);
                    gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer_object);

                    /*
                    if (use_custom_framebuffer === 0) {
                        gl.bindFramebuffer(gl.FRAMEBUFFER, null);
                    } else {
                    
                        // Create a new framebuffer whenever a framebuffer is bound.
                        // This might not be the best idea on some platforms.
                        let framebuffer = gl.createFramebuffer();

                        let color = kwasm_get_object(color_framebuffer_index);
                        let depth = kwasm_get_object(depth_framebuffer_index);
                        let stencil = kwasm_get_object(stencil_framebuffer_index);

                        gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
                        gl.framebufferTexture2D(
                            gl.FRAMEBUFFER,
                            gl.COLOR_ATTACHMENT0,
                            gl.TEXTURE_2D,
                            color,
                            0,
                        );
                        gl.framebufferTexture2D(
                            gl.FRAMEBUFFER,
                            gl.DEPTH_ATTACHMENT,
                            gl.TEXTURE_2D,
                            depth,
                            0,
                        );
                        gl.framebufferTexture2D(
                            gl.FRAMEBUFFER,
                            gl.STENCIL_ATTACHMENT,
                            gl.TEXTURE_2D,
                            stencil,
                            0,
                        );
                        if (temp_framebuffer) {
                            gl.deleteFramebuffer(temp_framebuffer);
                        }
                        temp_framebuffer = framebuffer;
                    }
                    */
                    break;
                }
                case 2: {
                    // ChangePipeline
                    let program_index = u32_data[u32_offset++];
                    let depth_func = u32_data[u32_offset++];
                    let culling = u32_data[u32_offset++];
                    let source_blend_factor = u32_data[u32_offset++];
                    let destination_blend_factor = u32_data[u32_offset++];
                    let depth_clear_value = f32_data[f32_offset++];

                    let program = kwasm_get_object(program_index);

                    gl.useProgram(program);

                    gl.depthFunc(depth_func);

                    if (culling === 0) {
                        gl.disable(gl.CULL_FACE);
                    } else {
                        gl.enable(gl.CULL_FACE);
                        gl.cullFace(culling);
                    }

                    if (source_blend_factor === 0) {
                        gl.disable(gl.BLEND);
                    } else {
                        gl.enable(gl.BLEND);
                        gl.blendFunc(source_blend_factor, destination_blend_factor);
                    }

                    gl.clearDepth(depth_clear_value);
                    break;
                }
                case 3: {
                    // SetVertexAttribute
                    let attribute_index = u32_data[u32_offset++];
                    let number_of_components = u32_data[u32_offset++];
                    let buffer_index = u32_data[u32_offset++];

                    //console.log("ATTRIBUTE INDEX" + attribute_index);
                    let buffer = kwasm_get_object(buffer_index);

                    if (buffer === null) {
                        gl.disableVertexAttribArray(attribute_index);
                    } else {
                        //   console.log("number_of_components: ", + number_of_components);
                        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
                        gl.vertexAttribPointer(
                            attribute_index,                // Index
                            number_of_components, // Number of components. It's assumed that components are always 32 bit.
                            gl.FLOAT,
                            false,
                            0, // 0 means to assume tightly packed
                            0, // Offset
                        );
                        gl.enableVertexAttribArray(attribute_index);
                    }
                    break;
                }
                case 4: {
                    // SetVertexAttribute
                    let attribute_index = u32_data[u32_offset++];
                    let number_of_components = u32_data[u32_offset++];

                    gl.disableVertexAttribArray(attribute_index);

                    let values = f32_data.subarray(f32_offset, f32_offset + number_of_components);
                    f32_offset += number_of_components;
                    switch (number_of_components) {
                        case 1:
                            self.gl.vertexAttrib1fv(attribute_index, values);
                            break;
                        case 2:
                            self.gl.vertexAttrib2fv(attribute_index, values);
                            break;
                        case 3:
                            self.gl.vertexAttrib3fv(attribute_index, values);
                            break;
                        case 4:
                            self.gl.vertexAttrib4fv(attribute_index, values);
                            break;
                    }

                    break;
                }
                case 5: {
                    // SetFloatUniform
                    let index = u32_data[u32_offset++];
                    let location = kwasm_get_object(index);

                    let v = f32_data[f32_offset++];
                    gl.uniform1f(location, v);
                    break;
                }
                case 6: {
                    // SetIntUniform
                    let index = u32_data[u32_offset++];
                    let location = kwasm_get_object(index);

                    let v = u32_data[u32_offset++];
                    gl.uniform1i(location, v);
                    break;
                }
                case 7: {
                    // SetVec2Uniform
                    let index = u32_data[u32_offset++];
                    let location = kwasm_get_object(index);

                    let v0 = f32_data[f32_offset++];
                    let v1 = f32_data[f32_offset++];

                    gl.uniform2f(location, v0, v1);
                    break;
                }
                case 8: {
                    // SetVec3Uniform
                    let index = u32_data[u32_offset++];
                    let location = kwasm_get_object(index);

                    let v0 = f32_data[f32_offset++];
                    let v1 = f32_data[f32_offset++];
                    let v2 = f32_data[f32_offset++];

                    gl.uniform3f(location, v0, v1, v2);
                    break;
                }
                case 9: {
                    // SetVec4Uniform
                    let index = u32_data[u32_offset++];
                    let location = kwasm_get_object(index);

                    let v0 = f32_data[f32_offset++];
                    let v1 = f32_data[f32_offset++];
                    let v2 = f32_data[f32_offset++];
                    let v3 = f32_data[f32_offset++];


                    gl.uniform4f(location, v0, v1, v2, v3);
                    break;
                }
                case 10: {
                    // SetMat4Uniform
                    let index = u32_data[u32_offset++];
                    let location = kwasm_get_object(index);

                    let mat4 = f32_data.subarray(f32_offset, f32_offset + 16);
                    f32_offset += 16;


                    gl.uniformMatrix4fv(location, false, mat4);
                    break;
                }
                case 11: {
                    // SetTextureUniform
                    let uniform_location_index = u32_data[u32_offset++];
                    let texture_index = u32_data[u32_offset++];
                    let texture_unit = u32_data[u32_offset++];

                    let uniform_location = kwasm_get_object(uniform_location_index);
                    let texture = kwasm_get_object(texture_index);

                    gl.activeTexture(gl.TEXTURE0 + texture_unit);
                    gl.bindTexture(gl.TEXTURE_2D, texture);
                    gl.uniform1i(uniform_location, texture_unit);
                    break;
                }
                case 12: {
                    // SetViewport
                    let x = u32_data[u32_offset++];
                    let y = u32_data[u32_offset++];
                    let width = u32_data[u32_offset++];
                    let height = u32_data[u32_offset++];

                    gl.viewport(x, y, width, height);
                    break;
                }
                case 13: {
                    // DrawTriangles

                    let count = u32_data[u32_offset++]; // Number of vertices to draw
                    let buffer_index = u32_data[u32_offset++];

                    if (buffer_index === 0) {
                        gl.drawArrays(gl.TRIANGLES, 0, count);
                    } else {
                        let buffer = kwasm_get_object(buffer_index);
                        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffer);
                        gl.drawElements(gl.TRIANGLES, count, gl.UNSIGNED_INT, 0);
                    }
                    break;
                }
                case 14: {
                    // Present
                    // No need to do anything
                    break;
                }
            }
        }

        // Delete the framebuffer if we've created one.
        /*
        if (temp_framebuffer) {
            gl.deleteFramebuffer(temp_framebuffer);
        }
        */
        // console.log("DONE WITH COMMANDS");

    }
};
gl_web_object