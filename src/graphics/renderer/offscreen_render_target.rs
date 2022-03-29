use super::*;

// Todo: Framebuffer is leaked when this is dropped.
pub struct OffscreenRenderTarget {
    framebuffer: Option<NotSendSync<Framebuffer>>,
    color_texture: Option<RenderTargetTexture>,
    depth_texture: Option<RenderTargetTexture>,
    resolve_framebuffer: Option<NotSendSync<Framebuffer>>,
    inner_texture_size: Vec2u,
    used_size: Vec2u,
    needs_resolve: bool,
}

impl OffscreenRenderTarget {
    pub fn new(
        graphics: &mut Graphics,
        textures: &mut Assets<Texture>,
        initial_size: Vec2u,
        color_pixel_format_and_texture_settings: Option<(PixelFormat, TextureSettings)>,
        depth_pixel_format_and_texture_settings: Option<(PixelFormat, TextureSettings)>,
    ) -> Self {
        // If both color and depth are defined check that they have the same msaa properties.
        debug_assert!(
            (color_pixel_format_and_texture_settings.is_none()
                || depth_pixel_format_and_texture_settings.is_none())
                || color_pixel_format_and_texture_settings.map(|(_, s)| s.msaa_samples)
                    == depth_pixel_format_and_texture_settings.map(|(_, s)| s.msaa_samples)
        );
        let needs_resolve =
            color_pixel_format_and_texture_settings.map_or(false, |(_, s)| s.msaa_samples != 0);
        let mut s = OffscreenRenderTarget {
            framebuffer: None,
            color_texture: color_pixel_format_and_texture_settings.map(|(p, s)| {
                RenderTargetTexture {
                    texture: Handle::default(),
                    pixel_format: p,
                    texture_settings: s,
                    resolve_texture: if needs_resolve {
                        Some(Handle::default())
                    } else {
                        None
                    },
                }
            }),
            depth_texture: depth_pixel_format_and_texture_settings.map(|(p, s)| {
                RenderTargetTexture {
                    texture: Handle::default(),
                    pixel_format: p,
                    texture_settings: s,
                    resolve_texture: if needs_resolve {
                        Some(Handle::default())
                    } else {
                        None
                    },
                }
            }),
            resolve_framebuffer: None,
            inner_texture_size: Vec2u::ZERO,
            used_size: Vec2u::ZERO,
            needs_resolve: color_pixel_format_and_texture_settings
                .as_ref()
                .map_or(false, |f| f.1.msaa_samples != 0),
        };
        s.resize(graphics, textures, initial_size);
        s
    }

    /// Resizes the inner texture to be exactly the request size
    pub fn resize_exact(
        &mut self,
        graphics: &mut Graphics,
        textures: &mut Assets<Texture>,
        size: Vec2u,
    ) {
        self.inner_texture_size = Vec2u::ZERO;
        self.resize(graphics, textures, size)
    }

    pub fn resize(&mut self, graphics: &mut Graphics, textures: &mut Assets<Texture>, size: Vec2u) {
        // Resize
        if size
            .greater_than_per_component(self.inner_texture_size)
            .any()
        {
            let size = size.max(self.inner_texture_size);
            self.color_texture
                .as_mut()
                .map(|c| c.resize(size, graphics, textures));
            self.depth_texture
                .as_mut()
                .map(|c| c.resize(size, graphics, textures));

            if let Some(framebuffer) = self.framebuffer.take() {
                graphics.context.delete_framebuffer(framebuffer.take())
            }
            self.framebuffer = Some(NotSendSync::new(
                graphics.context.new_framebuffer(
                    self.color_texture
                        .as_ref()
                        .map(|t| &textures.get(&t.texture).0),
                    self.depth_texture
                        .as_ref()
                        .map(|t| &textures.get(&t.texture).0),
                    None,
                ),
            ));

            if self.needs_resolve {
                if let Some(framebuffer) = self.resolve_framebuffer.take() {
                    graphics.context.delete_framebuffer(framebuffer.take())
                }
                self.resolve_framebuffer = Some(NotSendSync::new(
                    graphics.context.new_framebuffer(
                        self.color_texture
                            .as_ref()
                            .map(|t| &textures.get(t.resolve_texture.as_ref().unwrap()).0),
                        self.depth_texture
                            .as_ref()
                            .map(|t| &textures.get(t.resolve_texture.as_ref().unwrap()).0),
                        None,
                    ),
                ));
            }

            self.inner_texture_size = size;
        }

        self.used_size = size;
    }

    /// This assumes that the framebuffer is currently bound.
    pub fn resolve(&self, render_pass: RenderPass) {
        if let Some(resolve_framebuffer) = self.resolve_framebuffer.as_ref() {
            render_pass.blit_framebuffer(
                &**resolve_framebuffer,
                0,
                0,
                self.used_size.x as _,
                self.used_size.y as _,
                0,
                0,
                self.used_size.x as _,
                self.used_size.y as _,
            )
        }
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &*self.framebuffer.as_ref().unwrap()
    }

    /// Gets the readable color texture.
    pub fn color_texture(&self) -> &Handle<Texture> {
        let color_texture = self.color_texture.as_ref().unwrap();
        if let Some(texture) = color_texture.resolve_texture.as_ref() {
            texture
        } else {
            &color_texture.texture
        }
    }

    /// Gets the readable depth texture.
    pub fn depth_texture(&self) -> &Handle<Texture> {
        let depth_texture = self.depth_texture.as_ref().unwrap();
        if let Some(texture) = depth_texture.resolve_texture.as_ref() {
            texture
        } else {
            &depth_texture.texture
        }
    }

    pub fn inner_texture_scale(&self) -> Vec2 {
        self.used_size
            .as_f32()
            .div_by_component(self.inner_texture_size.as_f32())
    }

    pub fn size(&self) -> Vec2u {
        self.used_size
    }
}
