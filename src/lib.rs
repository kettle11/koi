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

mod temporary;
pub use temporary::*;

#[cfg(feature = "graphics")]
mod graphics;
#[cfg(feature = "graphics")]
pub use graphics::*;

mod experimental;
pub use experimental::*;

#[cfg(target_arch = "wasm32")]
use kwasm::libraries::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

pub struct App {
    systems: Plugin,
}

pub struct Plugin {
    pub setup_systems: Vec<System>,
    pub fixed_update_systems: Vec<System>,
    pub draw_systems: Vec<System>,
    pub end_of_frame_systems: Vec<System>,
}

impl Plugin {
    fn append(&mut self, mut other: Self) {
        self.setup_systems.append(&mut other.setup_systems);
        self.fixed_update_systems
            .append(&mut other.fixed_update_systems);
        self.draw_systems.append(&mut other.draw_systems);
        self.end_of_frame_systems
            .append(&mut other.end_of_frame_systems);
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            setup_systems: Vec::new(),
            fixed_update_systems: Vec::new(),
            draw_systems: Vec::new(),
            end_of_frame_systems: Vec::new(),
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
            systems: Plugin::default(),
        };
        s.add_default_plugins()
    }

    pub fn add_plugin(mut self, mut plugin: Plugin) -> Self {
        self.systems.append(plugin);
        self
    }

    pub fn add_default_plugins(self) -> Self {
        let app = self;
        #[cfg(feature = "graphics")]
        let app = app.add_plugin(graphics_plugin());
        let app = app.add_plugin(renderer_plugin());
        let app = app.add_plugin(temporary_despawn_plugin());
        app
    }

    pub fn setup_and_run<S: FnMut(Event, &mut World) + 'static>(
        mut self,
        setup_and_run_function: impl Fn(&mut World) -> S,
    ) {
        // Todo: Base this on number of cores
        ktasks::create_workers(3);

        let mut world = World::new();
        world.spawn(Commands::new());

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

        for setup_system in &mut self.systems.setup_systems {
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
                        for system in &mut self.systems.fixed_update_systems {
                            system.run(&mut world).unwrap()
                        }
                        apply_commands(&mut world);
                        run_system(crate::Event::FixedUpdate, &mut world);
                        apply_commands(&mut world);
                        time_acumulator -= fixed_time_step;
                    }

                    run_system(crate::Event::Draw, &mut world);
                    apply_commands(&mut world);
                    for system in &mut self.systems.draw_systems {
                        system.run(&mut world).unwrap()
                    }
                    apply_commands(&mut world);

                    // Run systems after the last draw.
                    for system in &mut self.systems.end_of_frame_systems {
                        system.run(&mut world).unwrap()
                    }
                    apply_commands(&mut world);

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
