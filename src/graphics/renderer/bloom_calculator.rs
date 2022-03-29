use crate::*;
use kgraphics::{CommandBuffer, CommandBufferTrait, PipelineTrait, RenderPassTrait};

pub struct BloomCalculator {
    passes: usize,
    downscale_targets: Vec<OffscreenRenderTarget>,
    upscale_targets: Vec<OffscreenRenderTarget>,
    downsample_shader: Shader,
    upscale_shader: Shader,
}

impl BloomCalculator {
    pub fn new(graphics: &mut Graphics, textures: &mut Assets<Texture>) -> Self {
        let passes = 6;
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
                include_str!("../built_in_shaders/bloom_upsample.glsl"),
                PipelineSettings {
                    depth_test: kgraphics::DepthTest::AlwaysPass,
                    ..Default::default()
                },
            )
            .unwrap();
        let mut downscale_targets = Vec::new();
        let mut upscale_targets = Vec::new();

        for _ in 0..passes {
            downscale_targets.push(OffscreenRenderTarget::new(
                graphics,
                textures,
                Vec2u::ZERO,
                settings,
                None,
            ));
            upscale_targets.push(OffscreenRenderTarget::new(
                graphics,
                textures,
                Vec2u::ZERO,
                settings,
                None,
            ));
        }
        Self {
            passes,
            downsample_shader,
            upscale_shader,
            downscale_targets,
            upscale_targets,
        }
    }

    fn resize(&mut self, mut size: Vec2u, graphics: &mut Graphics, textures: &mut Assets<Texture>) {
        let mut steps = 0;
        for (upscale_target, downscale_target) in self
            .upscale_targets
            .iter_mut()
            .zip(self.downscale_targets.iter_mut())
        {
            if size.x <= 2 || size.y <= 2 {
                break;
            }
            steps += 1;

            upscale_target.resize_exact(graphics, textures, size);
            size /= 2;

            // The first texture in the downscale chain is the incoming color buffer.
            downscale_target.resize_exact(graphics, textures, size);
        }
        self.passes = steps;
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

        let p_corresponding_downsample_texture = &self
            .upscale_shader
            .pipeline
            .get_texture_property("p_corresponding_downsample_texture")
            .unwrap();

        let mut last_half_pixel_size = Vec2::fill(0.5).div_by_component(starting_size.as_f32());
        let mut last_texture = starting_texture;

        // println!("STARTING SIZE: {:?}", starting_size);

        for i in 0..self.passes {
            let current_target = &self.downscale_targets[i];
            // For some reason not clearing the target is significantly faster.
            let mut render_pass = command_buffer
                .begin_render_pass_with_framebuffer(current_target.framebuffer(), None);

            render_pass.set_viewport(
                0,
                0,
                current_target.size().x as _,
                current_target.size().y as _,
            );

            // println!(
            //     "DOWNSAMPLE CURRENT TARGET SIZE: {:?}",
            //     current_target.inner_texture_size
            // );
            render_pass.set_pipeline(&self.downsample_shader.pipeline);
            render_pass.set_texture_property(
                p_texture_downsample,
                Some(textures.get(last_texture)),
                0,
            );
            render_pass.set_vec2_property(p_half_pixel_downsample, last_half_pixel_size.into());
            render_pass.set_vec2_property(p_texture_coordinate_scale_downsample, Vec2::ONE.into());

            render_pass.draw_triangles_without_buffer(1);

            last_texture = current_target.color_texture();
            last_half_pixel_size = Vec2::fill(0.5).div_by_component(current_target.size().as_f32())
        }

        for i in (0..self.passes).rev() {
            let current_target = &self.upscale_targets[i];
            let mut render_pass = command_buffer
                .begin_render_pass_with_framebuffer(current_target.framebuffer(), None);

            render_pass.set_viewport(
                0,
                0,
                current_target.size().x as _,
                current_target.size().y as _,
            );
            // println!(
            //     "UPSCALE CURRENT TARGET SIZE: {:?}",
            //     current_target.inner_texture_size
            // );
            render_pass.set_pipeline(&self.upscale_shader.pipeline);
            render_pass.set_texture_property(
                p_texture_upsample,
                Some(textures.get(last_texture)),
                0,
            );
            render_pass.set_texture_property(
                p_corresponding_downsample_texture,
                Some(textures.get(if i != 0 {
                    self.downscale_targets[i - 1].color_texture()
                } else {
                    starting_texture
                })),
                1,
            );
            render_pass.set_vec2_property(p_half_pixel_upsample, last_half_pixel_size.into());
            render_pass.set_vec2_property(p_texture_coordinate_scale_upsample, Vec2::ONE.into());

            render_pass.draw_triangles_without_buffer(1);

            last_texture = current_target.color_texture();
            last_half_pixel_size = Vec2::fill(0.5).div_by_component(current_target.size().as_f32())
        }

        self.upscale_targets[0].color_texture()
    }
}
