use glow::*;
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let window = app.new_window().build().unwrap();

    // Create a GLContext
    let mut gl_context = GLContext::new().build().unwrap();

    // Assign the GLContext's window.
    gl_context.set_window(Some(&window)).unwrap();

    // Glow is a library for accessing GL function calls from a variety of platforms
    // Glow requires a cross platform way to load function pointers,
    // which GLContext provides with get_proc_address.
    // Glow requires different setup on web, hence the cfgs below.

    #[cfg(target_arch = "wasm32")]
    let gl = glow::Context::from_webgl1_context(gl_context.webgl1_context().unwrap());
    #[cfg(not(target_arch = "wasm32"))]
    let gl = glow::Context::from_loader_function(|s| gl_context.get_proc_address(s));

    event_loop.run(move |event| {
        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::Draw { .. } => {
                // Clear the screen.
                unsafe {
                    gl.clear_color(0.3765, 0.3137, 0.8627, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }
                // Finally display what we've drawn.
                gl_context.swap_buffers();

                // It is not necessary for this example,
                // but calling request_frame ensures the program redraws continuously.
                window.request_redraw();
            }
            Event::PointerMoved { .. } | Event::EventsCleared => {}
            _ => {
                println!("event: {:?}", event);
            }
        }
    });
}
