/*
use crate::*;
use kgraphics::*;

fn create_cubemap(graphics: &mut Graphics) {
    let cube_map = graphics.context.new_cube_map(
        512,
        512,
        None,
        PixelFormat::RGB16F,
        TextureSettings {
            srgb: false,
            minification_filter: FilterMode::Linear,
            magnification_filter: FilterMode::Linear,
            wrapping_horizontal: WrappingMode::ClampToEdge,
            wrapping_vertical: WrappingMode::ClampToEdge,
            generate_mipmaps: false,
            ..Default::default()
        },
    );
    let mut command_buffer = graphics.context.new_command_buffer();

    let framebuffer = todo!();
    let render_pass = command_buffer.begin_render_pass_with_framebuffer(framebuffer, None);
    // render_pass.draw_triangles(count, index_buffer)
}
*/
