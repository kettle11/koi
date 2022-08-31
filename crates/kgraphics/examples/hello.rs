use kapp::*;
use kgraphics::*;

fn main() {
    let (app, event_loop) = initialize();
    event_loop.run_async(app, run_async);
}

async fn run_async(app: Application, events: Events) {
    #[cfg(target_arch = "wasm32")]
    kwasm::setup_panic_hook();

    let window = app
        .new_window()
        .title("kgraphics hello")
        .size(800, 800)
        .build();
    let mut g = GraphicsContext::new_with_settings(GraphicsContextSettings {
        high_resolution_framebuffer: true,
        ..Default::default()
    });
    let vertex_function = g
        .new_vertex_function(
            r#"layout(location = 0) in vec3 a_position;

            void main()
            {
                gl_Position = vec4(a_position, 1.0);
            }"#,
        )
        .unwrap();
    let fragment_function = g
        .new_fragment_function(
            r#"
            precision mediump float;

            layout(location = 0) out vec4 color_out;

            uniform vec4 p_custom_color;

            void main()
            {
                color_out = p_custom_color;
            }"#,
        )
        .unwrap();

    let (window_width, window_height) = window.size();
    let render_target = g.get_render_target_for_window(&window, window_width, window_height);

    let pipeline = g
        .new_pipeline(
            vertex_function,
            fragment_function,
            render_target.pixel_format(),
        )
        .build()
        .unwrap();

    let vertex_position_attribute: VertexAttribute<(f32, f32, f32)> =
        pipeline.get_vertex_attribute("a_position").unwrap();

    let custom_color = pipeline.get_vec4_property("p_custom_color").unwrap();

    let vertex_data: [(f32, f32, f32); 3] = [(0.0, 1.0, 0.0), (-1.0, -1.0, 0.0), (1.0, -1.0, 0.0)];
    let vertex_buffer = g.new_data_buffer(&vertex_data).unwrap();
    let index_buffer = g.new_index_buffer(&[0, 1, 2]).unwrap();

    let mut color = (0., 0., 0., 1.);
    let mut triangle_color = (1., 1., 0., 1.);

    window.request_redraw();
    loop {
        let event = events.next().await;
        // klog::log!("EVENT: {:?}", event);

        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::KeyDown { key: Key::A, .. } => {
                color = (0., 0., 1., 1.);
                window.request_redraw();
            }
            Event::KeyDown { key: Key::B, .. } => {
                triangle_color = (0., 0., 1., 1.);
                window.request_redraw();
            }

            Event::Draw { .. } => {
                //println!("DRAW--------------------");

                {
                    let mut command_buffer = g.new_command_buffer();

                    // Render pass
                    {
                        let mut render_pass = command_buffer.begin_render_pass_with_framebuffer(
                            &Framebuffer::default(),
                            Some((1.0, 0.0, 0.0, 1.0)),
                        );

                        render_pass.set_pipeline(&pipeline);

                        // Draw an object

                        render_pass
                            .set_vertex_attribute(&vertex_position_attribute, Some(&vertex_buffer));
                        render_pass.set_vec4_property(&custom_color, triangle_color);
                        render_pass.draw_triangles(1, &index_buffer);
                    }
                    command_buffer.present();
                    g.commit_command_buffer(command_buffer);
                }

                color.1 += 0.02;
                window.request_redraw();
            }
            Event::WindowResized { width, height, .. } => {
                g.resize(&window, width, height);
                window.request_redraw();
            }
            _ => {}
        }
    }
}
