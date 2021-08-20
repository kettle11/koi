use crate::*;
use kgraphics::*;

/*
#[derive(Component, Clone)]
pub struct Renderer {

}
*/

pub fn renderer_plugin() -> Plugin {
    Plugin {
        draw_systems: vec![render_scene.system()],
        ..Default::default()
    }
}

pub fn render_scene(graphics: &mut Graphics, cameras: Query<(&Camera, &Transform)>) {
    let mut command_buffer = graphics.context.new_command_buffer();
    let frame = graphics.render_target.current_frame().unwrap();
    {
        let render_pass = command_buffer.begin_render_pass(
            Some(&frame),
            Some(&frame),
            None,
            Some((1.0, 0.0, 0.0, 1.0)),
        );

        for (camera, camera_transform) in &cameras {}
    }

    graphics.context.commit_command_buffer(command_buffer);
}
