use crate::*;
use kgraphics::{CommandBuffer, CommandBufferTrait, PipelineTrait, RenderPassTrait};

pub struct BlurCalculator {
    targets: [OffscreenRenderTarget; 2],
    downsample_shader: Shader,
    upscale_shader: Shader,
}

impl BlurCalculator {
    pub fn new(graphics: &mut Graphics, textures: &mut Assets<Texture>) -> Self {
        let settings = Some((
            kgraphics::PixelFormat::RGBA16F,
            TextureSettings {
                srgb: false,
                generate_mipmaps: false,
                wrapping_horizontal: WrappingMode::ClampToEdge,
                wrapping_vertical: WrappingMode::ClampToEdge,
                minification_filter: FilterMode::Linear,
                magnification_filter: FilterMode::Linear,
                ..Default::default()
            },
        ));

        let downsample_shader = graphics
            .new_shader(
                include_str!("../built_in_shaders/dual_kawase_downsample.glsl"),
                PipelineSettings {
                    depth_test: kgraphics::DepthTest::AlwaysPass,
                    ..Default::default()
                },
            )
            .unwrap();
        let upscale_shader = graphics
            .new_shader(
                include_str!("../built_in_shaders/dual_kawase_upsample.glsl"),
                PipelineSettings {
                    depth_test: kgraphics::DepthTest::AlwaysPass,
                    ..Default::default()
                },
            )
            .unwrap();
        Self {
            downsample_shader,
            upscale_shader,
            targets: [
                OffscreenRenderTarget::new(graphics, textures, Vec2u::ZERO, settings, None),
                OffscreenRenderTarget::new(graphics, textures, Vec2u::ZERO, settings, None),
            ],
        }
    }

    fn resize(&mut self, size: Vec2u, graphics: &mut Graphics, textures: &mut Assets<Texture>) {
        self.targets[0].resize(graphics, textures, size);
        self.targets[1].resize(graphics, textures, size);
    }

    pub fn blur_texture(
        &mut self,
        graphics: &mut Graphics,
        textures: &mut Assets<Texture>,
        command_buffer: &mut CommandBuffer,
        starting_texture: &Handle<Texture>,
        starting_size: Vec2u,
    ) -> &Handle<Texture> {
        self.resize(starting_size, graphics, textures);

        let mut size = starting_size.as_u32();

        let mut scale = self.targets[0].inner_texture_scale();

        let mut last_texture = starting_texture;
        let mut target = 0;
        let passes = 4;

        let p_texture_downsample = &self
            .downsample_shader
            .pipeline
            .get_texture_property("p_texture")
            .unwrap();
        let p_texture_upsample = &self
            .upscale_shader
            .pipeline
            .get_texture_property("p_texture")
            .unwrap();

        let p_texture_coordinate_scale_downsample = &self
            .downsample_shader
            .pipeline
            .get_vec2_property("p_texture_coordinate_scale")
            .unwrap();
        let p_texture_coordinate_scale_upsample = &self
            .upscale_shader
            .pipeline
            .get_vec2_property("p_texture_coordinate_scale")
            .unwrap();

        let p_half_pixel_downsample = &self
            .downsample_shader
            .pipeline
            .get_vec2_property("p_half_pixel")
            .unwrap();
        let p_half_pixel_upsample = &self
            .upscale_shader
            .pipeline
            .get_vec2_property("p_half_pixel")
            .unwrap();

        // This is consistent because the underlying texture doesn't change size.
        let half_pixel_size = Vec2::fill(0.5).div_by_component(size.as_f32());

        println!("STARTING BLUR WITH SIZE: {:?}", size);
        for _ in 0..passes {
            // For some reason not clearing the screen is significantly faster.
            let mut render_pass = command_buffer
                .begin_render_pass_with_framebuffer(self.targets[target].framebuffer(), None);

            let new_size = size / 2;
            println!("DOWNSCALE SIZE: {:?}", new_size);
            render_pass.set_viewport(0, 0, new_size.x, new_size.y);
            render_pass.set_pipeline(&self.downsample_shader.pipeline);
            render_pass.set_texture_property(
                p_texture_downsample,
                Some(textures.get(last_texture)),
                0,
            );
            render_pass.set_vec2_property(p_texture_coordinate_scale_downsample, scale.into());
            render_pass.set_vec2_property(p_half_pixel_downsample, half_pixel_size.into());

            render_pass.draw_triangles_without_buffer(1);

            last_texture = self.targets[target].color_texture();
            target ^= 1;
            let new_size = size / 2;
            size = new_size;
            scale /= 2.0;
        }

        for _ in 0..passes {
            let mut render_pass = command_buffer
                .begin_render_pass_with_framebuffer(self.targets[target].framebuffer(), None);
            let new_size = size * 2;
            println!("UPSCALE SIZE: {:?}", new_size);
            render_pass.set_viewport(0, 0, new_size.x, new_size.y);
            render_pass.set_pipeline(&self.upscale_shader.pipeline);
            render_pass.set_texture_property(
                p_texture_upsample,
                Some(textures.get(last_texture)),
                0,
            );
            render_pass.set_vec2_property(p_texture_coordinate_scale_upsample, scale.into());
            render_pass.set_vec2_property(p_half_pixel_upsample, half_pixel_size.into());

            render_pass.draw_triangles_without_buffer(1);

            last_texture = self.targets[target].color_texture();
            target ^= 1;

            size = new_size;
            scale *= 2.0;
        }

        self.targets[target ^ 1].color_texture()
    }
}
