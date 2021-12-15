use kgraphics::{CommandBufferTrait, DepthTest, GraphicsContextTrait, RenderPassTrait};

use crate::*;

pub(crate) fn generate_brdf_lookup(graphics: &mut Graphics) -> Texture {
    let new_shader = graphics
        .new_shader(
            include_str!("../built_in_shaders/brdf_lookup.glsl"),
            PipelineSettings {
                depth_test: DepthTest::LessOrEqual,
                ..PipelineSettings::default()
            },
        )
        .unwrap();

    let size = 512;

    let texture = graphics
        .new_texture(
            None,
            size,
            size,
            kgraphics::PixelFormat::RGBA16F,
            TextureSettings {
                wrapping_horizontal: WrappingMode::ClampToEdge,
                wrapping_vertical: WrappingMode::ClampToEdge,
                minification_filter: FilterMode::Linear,
                magnification_filter: FilterMode::Linear,
                generate_mipmaps: false,
                srgb: false,
                ..Default::default()
            },
        )
        .unwrap();

    let framebuffer = graphics
        .context
        .new_framebuffer(Some(&texture.0), None, None);

    let mut command_buffer = graphics.context.new_command_buffer();
    {
        let mut render_pass = command_buffer
            .begin_render_pass_with_framebuffer(&framebuffer, Some((0.0, 1.0, 1.0, 1.0)));
        render_pass.set_viewport(0, 0, size, size);

        render_pass.set_pipeline(&new_shader.pipeline);

        render_pass.draw_triangles_without_buffer(1);
    }

    graphics.context.commit_command_buffer(command_buffer);
    graphics.context.delete_framebuffer(framebuffer);
    texture
}
