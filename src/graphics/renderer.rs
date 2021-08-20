use crate::*;
use kgraphics::*;

pub fn renderer_plugin() -> Plugin {
    Plugin {
        draw_systems: vec![render_scene.system()],
        ..Default::default()
    }
}

pub fn render_scene(
    graphics: &mut Graphics,
    cameras: Query<(&Transform, &Camera)>,
    renderables: Query<(&Transform, &Handle<Mesh>)>,
) {
    let mut command_buffer = graphics.context.new_command_buffer();
    let frame = graphics.render_target.current_frame().unwrap();

    for (camera_transform, camera) in &cameras {
        let clear_color = camera.clear_color.map(|c| {
            // Presently the output needs to be in non-linear sRGB.
            // However that means that blending with the clear-color will be incorrect.
            // A post-processing pass is needed to convert into the appropriate output space.
            let c = c.to_rgb_color(color_spaces::ENCODED_SRGB);
            (c.red, c.green, c.blue, c.alpha)
        });

        let render_pass =
            command_buffer.begin_render_pass(Some(&frame), Some(&frame), None, clear_color);

        for (renderable_transform, renderable) in &renderables {
            // Todo
            // Render renderable here.
        }
    }

    graphics.context.commit_command_buffer(command_buffer);
}
