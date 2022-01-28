use super::*;

pub(crate) struct RenderFramebuffer {
    framebuffer: Framebuffer,
    framebuffer_color_texture: Texture,
    framebuffer_depth_texture: Texture,
    resolve_framebuffer: Framebuffer,
    color_resolve_texture: Texture,
    size: (u32, u32),
    // For now there's no depth resolve texture because it's not used.
}

impl RenderFramebuffer {
    fn new(graphics: &mut Graphics, width: u32, height: u32, msaa_samples: u8) -> Self {
        let framebuffer_color_texture = graphics
            .new_texture(
                None,
                width,
                height,
                PixelFormat::RGBA32F,
                TextureSettings {
                    srgb: false,
                    generate_mipmaps: false,
                    msaa_samples,
                    ..TextureSettings::default()
                },
            )
            .unwrap();

        let framebuffer_depth_texture = graphics
            .new_texture(
                None,
                width,
                height,
                PixelFormat::Depth16,
                TextureSettings {
                    srgb: false,
                    generate_mipmaps: false,
                    msaa_samples,
                    ..TextureSettings::default()
                },
            )
            .unwrap();
        let framebuffer = graphics.context.new_framebuffer(
            Some(&framebuffer_color_texture),
            Some(&framebuffer_depth_texture),
            None,
        );

        let color_resolve_texture = graphics
            .new_texture(
                None,
                width,
                height,
                PixelFormat::RGBA32F,
                TextureSettings {
                    srgb: false,
                    generate_mipmaps: false,
                    ..TextureSettings::default()
                },
            )
            .unwrap();

        let resolve_framebuffer =
            graphics
                .context
                .new_framebuffer(Some(&color_resolve_texture), None, None);

        RenderFramebuffer {
            framebuffer,
            framebuffer_color_texture,
            framebuffer_depth_texture,
            resolve_framebuffer,
            color_resolve_texture,
            size: (width, height),
        }
    }

    fn delete(self, graphics: &mut Graphics) {
        graphics.context.delete_framebuffer(self.framebuffer);
        graphics
            .context
            .delete_texture(self.framebuffer_color_texture.0);
        graphics
            .context
            .delete_texture(self.framebuffer_depth_texture.0);
        graphics
            .context
            .delete_framebuffer(self.resolve_framebuffer);
        graphics
            .context
            .delete_texture(self.color_resolve_texture.0);
    }

    /// This must be the same format as the current framebuffer.
    /// This is designed to be used by the same `RenderFramebuffer` that began the `RenderPass`.
    fn resolve(&self, render_pass: &mut RenderPass) -> &Texture {
        render_pass.blit_framebuffer(
            self.resolve_framebuffer,
            0,
            0,
            self.size.0,
            self.size.1,
            0,
            0,
            self.size.0,
            self.size.1,
        );
        &self.color_resolve_texture
    }
}

/// A triple buffered framebuffer abstraction to avoid OpenGL stalls
pub struct TripleBufferedFramebuffer {
    render_framebuffers: [RenderFramebuffer; 3],
    width: u32,
    height: u32,
    viewport_size: (u32, u32),
    current: usize,
}

impl TripleBufferedFramebuffer {
    pub fn new(graphics: &mut Graphics, width: u32, height: u32, msaa_samples: u8) -> Self {
        Self {
            render_framebuffers: [
                RenderFramebuffer::new(graphics, width, height, msaa_samples),
                RenderFramebuffer::new(graphics, width, height, msaa_samples),
                RenderFramebuffer::new(graphics, width, height, msaa_samples),
            ],
            width,
            height,
            viewport_size: (width, height),
            current: 0,
        }
    }

    pub fn resize(&mut self, graphics: &mut Graphics, width: u32, height: u32) {
        if width > self.width || height > self.height {
            let mut old_self = Self::new(graphics, width, height, self.msaa_samples);
            std::mem::swap(self, &mut old_self);

            // This should be done automatically by `kgraphics` instead.
            for render_framebuffer in old_self.render_framebuffers {
                render_framebuffer.delete(graphics);
            }
        }
        self.viewport_size = (width, height);
    }

    pub fn get_next(&mut self) -> &Framebuffer {
        self.current += 1;
        if self.current > 2 {
            self.current = 0;
        }
        &self.render_framebuffers[self.current].0
    }

    pub fn size(&self) -> (u32, u32) {
        self.viewport_size
    }
}
