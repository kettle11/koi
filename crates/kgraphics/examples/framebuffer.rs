use kapp::*;
use kgraphics::*;

fn main() {
    let (app, event_loop) = initialize();
    event_loop.run_async(app, run_async);
}

async fn run_async(app: Application, events: Events) {
    let window = app
        .new_window()
        .title("kgraphics hello")
        .size(800, 800)
        .build()
        .unwrap();

    let mut g = GraphicsContext::new_with_settings(GraphicsContextSettings {
        high_resolution_framebuffer: true,
        ..Default::default()
    })
    .unwrap();

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
            layout(location = 0) out vec4 color_out;

            //uniform vec4 p_customColor;

            void main()
            {
                color_out = vec4(1.0, 0.0, 0.0, 1.0);
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

    //let custom_color = pipeline.get_vec4_property("p_customColor").unwrap();

    let vertex_data: [(f32, f32, f32); 3] = [(0.0, 1.0, 0.0), (-1.0, -1.0, 0.0), (1.0, -1.0, 0.0)];
    let vertex_buffer = g.new_data_buffer(&vertex_data).unwrap();
    let index_buffer = g.new_index_buffer(&[0, 1, 2]).unwrap();

    let mut color = (0., 0., 0., 1.);
    //let mut triangle_color = (1., 0., 0., 1.);

    // For some reason the following error occurs on my M1 Mac if BGRA8Unorm is used:
    // UNSUPPORTED (log once): POSSIBLE ISSUE: unit 0 GLD_TEXTURE_INDEX_2D is unloadable and bound to sampler type (Float) - using zero texture because texture unloadable
    let target_color_texture = g
        .new_texture(
            256,
            256,
            None,
            PixelFormat::RGBA8Unorm,
            TextureSettings::default(),
        )
        .unwrap();

    let fullscreen_vertex = g
        .new_vertex_function(
            r#"
            out vec2 TexCoords;
 
            void main()
            {
                float x = -1.0 + float((gl_VertexID & 1) << 2);
                float y = -1.0 + float((gl_VertexID & 2) << 1);
                TexCoords.x = (x+1.0)*0.5;
                TexCoords.y = (y+1.0)*0.5;
                gl_Position = vec4(x, y, 0, 1);
            }
            "#,
        )
        .unwrap();

    let fullscreen_fragment = g
        .new_fragment_function(
            r#"
            in vec2 TexCoords;

            uniform sampler2D p_texture;

            layout(location = 0) out vec4 color_out;

            void main()
            {
                vec3 color = texture(p_texture, TexCoords).rgb;
                color_out = vec4(color, 1.0);
            }"#,
        )
        .unwrap();

    let fullscreen_pipeline = g
        .new_pipeline(
            fullscreen_vertex,
            fullscreen_fragment,
            render_target.pixel_format(),
        )
        .build()
        .unwrap();

    let texture_property = pipeline.get_texture_property("p_texture").unwrap();
    window.request_redraw();
    loop {
        let event = events.next().await;
        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::KeyDown { key: Key::A, .. } => {
                color = (0., 0., 1., 1.);
                window.request_redraw();
            }
            Event::KeyDown { key: Key::B, .. } => {
                //  triangle_color = (0., 0., 1., 1.);
                window.request_redraw();
            }
            Event::Draw { .. } => {
                {
                    let mut command_buffer = g.new_command_buffer();

                    // Render pass

                    {
                        let mut render_pass = command_buffer.begin_render_pass(
                            Some(&target_color_texture),
                            None,
                            None,
                            Some(color),
                        );

                        render_pass.set_pipeline(&pipeline);

                        // Draw an object

                        render_pass
                            .set_vertex_attribute(&vertex_position_attribute, Some(&vertex_buffer));
                        // render_pass.set_vec4_property(&custom_color, triangle_color);
                        render_pass.draw_triangles(1, &index_buffer);
                    }

                    // Second render pass
                    {
                        let render_texture = render_target.current_frame().unwrap();

                        let mut render_pass = command_buffer.begin_render_pass(
                            Some(&render_texture),
                            None,
                            None,
                            Some(color),
                        );

                        render_pass.set_pipeline(&fullscreen_pipeline);

                        render_pass.set_texture_property(
                            &texture_property,
                            Some(&target_color_texture),
                            0,
                        );

                        // Draw an object

                        render_pass.draw_triangles_without_buffer(1);
                    }

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
