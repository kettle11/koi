pub use klog::*;
pub use kmath::*;
pub use kudo::*;

mod not_send_sync;
pub use not_send_sync::*;

mod time;
pub use time::*;

mod assets;
pub use assets::*;

mod random;
pub use random::*;

mod color;
pub use color::*;

mod transform;
pub use transform::*;

mod commands;
pub use commands::*;

mod math;
pub use math::*;

#[cfg(feature = "graphics")]
mod graphics;
#[cfg(feature = "graphics")]
pub use graphics::*;

#[cfg(target_arch = "wasm32")]
use kwasm::libraries::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

pub struct App {
    setup_systems: Vec<System>,
    fixed_upate_systems: Vec<System>,
    draw_systems: Vec<System>,
}

pub struct Plugin {
    pub setup_systems: Vec<System>,
    pub fixed_update_systems: Vec<System>,
    pub draw_systems: Vec<System>,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            setup_systems: Vec::new(),
            fixed_update_systems: Vec::new(),
            draw_systems: Vec::new(),
        }
    }
}

pub enum Event {
    FixedUpdate,
    Draw,
}

impl App {
    pub fn new() -> Self {
        let s = Self {
            setup_systems: Vec::new(),
            fixed_upate_systems: Vec::new(),
            draw_systems: Vec::new(),
        };
        s.add_default_plugins()
    }

    pub fn add_plugin(mut self, mut plugin: Plugin) -> Self {
        self.setup_systems.append(&mut plugin.setup_systems);
        self.fixed_upate_systems
            .append(&mut plugin.fixed_update_systems);
        self.draw_systems.append(&mut plugin.draw_systems);
        self
    }

    pub fn add_default_plugins(self) -> Self {
        let app = self;
        #[cfg(feature = "graphics")]
        let app = app.add_plugin(graphics_plugin());
        app
    }

    pub fn setup_and_run<S: FnMut(Event, &mut World) + 'static>(
        mut self,
        setup_and_run_function: impl Fn(&mut World) -> S,
    ) {
        // Todo: Base this on number of cores
        ktasks::create_workers(3);

        let mut world = World::new();

        // For now `kapp` is integrated directly into `koi`
        let (kapp_app, kapp_event_loop) = kapp::initialize();

        let window_width = 1600;
        let window_height = 1200;

        // For now only a single
        let window = kapp_app
            .new_window()
            .title("Koi")
            .size(window_width, window_height)
            .build()
            .unwrap();

        window.request_redraw();
        let window_entity = world.spawn(NotSendSync::new(window));

        for setup_system in &mut self.setup_systems {
            setup_system.run(&mut world).unwrap()
        }

        // Setup time tracking
        let mut start = Instant::now();
        let mut time_acumulator = 0.0;

        // Hard-coded to 60 fixed updates per second for now.
        let fixed_time_step = 1.0 / 60.0;

        world.spawn(Time {
            // Set the delta_time to fixed_time_delta so that a fixed update runs for the first frame.
            delta_seconds_f64: fixed_time_step,
            fixed_time_step: fixed_time_step,
        });

        let mut run_system = setup_and_run_function(&mut world);

        kapp_event_loop.run(move |event| {
            use kapp::Event;

            match event {
                Event::WindowCloseRequested { .. } => kapp_app.quit(),
                Event::Draw { .. } => {
                    let elapsed = start.elapsed();
                    let time_elapsed_seconds = elapsed.as_secs_f64();
                    start = Instant::now();

                    time_acumulator += time_elapsed_seconds;
                    while time_acumulator >= fixed_time_step {
                        for system in &mut self.fixed_upate_systems {
                            system.run(&mut world).unwrap()
                        }
                        run_system(crate::Event::FixedUpdate, &mut world);
                        time_acumulator -= fixed_time_step;
                    }

                    run_system(crate::Event::Draw, &mut world);
                    for system in &mut self.draw_systems {
                        system.run(&mut world).unwrap()
                    }

                    // This ensures a continuous redraw.
                    world
                        .get_component_mut::<NotSendSync<kapp::Window>>(window_entity)
                        .unwrap()
                        .request_redraw();
                }
                _ => {}
            }
        })
    }
}
