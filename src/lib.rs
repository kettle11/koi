use std::ops::{Deref, DerefMut};

pub use kecs::*;
pub use klog::*;
pub use kmath::*;

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

mod input;
pub use input::*;

mod world_assets;
pub use world_assets::*;

#[cfg(feature = "graphics")]
mod graphics;
#[cfg(feature = "graphics")]
pub use graphics::*;

#[cfg(feature = "audio")]
mod audio;
#[cfg(feature = "audio")]
pub use audio::*;

#[cfg(feature = "xr")]
mod xr;
#[cfg(feature = "xr")]
pub use xr::*;

pub use kapp::{Event as KappEvent, Key, PointerButton};
/*
mod experimental;
pub use experimental::*;
*/

#[cfg(target_arch = "wasm32")]
use kwasm::libraries::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
pub use kwasm;

pub use klog;

pub struct App {
    systems: Plugin,
}

pub struct Plugin {
    pub setup_systems: Vec<System>,
    pub pre_fixed_update_systems: Vec<System>,
    pub fixed_update_systems: Vec<System>,
    pub draw_systems: Vec<System>,
    pub end_of_frame_systems: Vec<System>,
    pub on_kapp_events: Vec<System>,
}

impl Plugin {
    fn append(&mut self, mut other: Self) {
        self.setup_systems.append(&mut other.setup_systems);
        self.pre_fixed_update_systems
            .append(&mut other.pre_fixed_update_systems);
        self.fixed_update_systems
            .append(&mut other.fixed_update_systems);
        self.draw_systems.append(&mut other.draw_systems);
        self.end_of_frame_systems
            .append(&mut other.end_of_frame_systems);
        self.on_kapp_events.append(&mut other.on_kapp_events);
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            setup_systems: Vec::new(),
            pre_fixed_update_systems: Vec::new(),
            fixed_update_systems: Vec::new(),
            draw_systems: Vec::new(),
            end_of_frame_systems: Vec::new(),
            on_kapp_events: Vec::new(),
        }
    }
}

pub enum Event {
    FixedUpdate,
    Draw,
    KappEvent(kapp::Event),
}

/// Raw events from `kapp`, cleared at the end of every frame.
#[derive(Component, Clone)]
pub struct KappEvents(pub Vec<KappEvent>);

impl Deref for KappEvents {
    type Target = Vec<KappEvent>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KappEvents {
    fn deref_mut(&mut self) -> &mut Vec<kapp::Event> {
        &mut self.0
    }
}

impl App {
    pub fn new() -> Self {
        let s = Self {
            systems: Plugin::default(),
        };
        s.add_default_plugins()
    }

    pub fn add_plugin(mut self, plugin: Plugin) -> Self {
        self.systems.append(plugin);
        self
    }

    #[allow(clippy::let_and_return)]
    pub fn add_default_plugins(self) -> Self {
        let app = self;
        let app = app.add_plugin(world_assets_plugin());
        let app = app.add_plugin(transform_plugin());
        #[cfg(feature = "graphics")]
        let app = app.add_plugin(graphics_plugin());
        #[cfg(feature = "renderer")]
        let app = app.add_plugin(renderer_plugin());
        #[cfg(feature = "audio")]
        let app = app.add_plugin(audio_plugin());
        let app = app.add_plugin(temporary_despawn_plugin());
        #[cfg(feature = "graphics")]
        let app = app.add_plugin(camera_plugin());
        #[cfg(feature = "graphics")]
        let app = app.add_plugin(camera_controls_plugin());
        #[cfg(feature = "xr")]
        let app = app.add_plugin(xr_plugin());
        app
    }

    pub fn setup_and_run<S: FnMut(Event, &mut World) + 'static>(
        mut self,
        setup_and_run_function: impl Fn(&mut World) -> S,
    ) {
        // Todo: Base this on number of cores
        ktasks::create_workers(4);

        let mut world = World::new();
        world.spawn(Commands::new());
        let kapp_events_entity = world.spawn(KappEvents(Vec::new()));

        // For now `kapp` is integrated directly into `koi`
        let (kapp_app, kapp_event_loop) = kapp::initialize();

        world.spawn(NotSendSync::new(kapp_app.clone()));

        let window_width = 1600;
        let window_height = 1200;

        // For now only a single window is suppported.
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
            fixed_time_step,
        });

        // Setup input
        let input_entity = world.spawn(Input::new());

        let mut run_system = setup_and_run_function(&mut world);

        kapp_event_loop.run(move |event| {
            use kapp::Event;

            // Update the input manager.
            let input = world.get_component_mut::<Input>(input_entity).unwrap();
            input.state.handle_event(&event);

            world
                .get_component_mut::<KappEvents>(kapp_events_entity)
                .unwrap()
                .push(event.clone());
            for system in &mut self.systems.on_kapp_events {
                system.run(&mut world).unwrap()
            }

            run_system(crate::Event::KappEvent(event.clone()), &mut world);

            match event {
                Event::WindowCloseRequested { .. } => kapp_app.quit(),
                Event::Draw { .. } => {
                    //   ktasks::run_only_local_tasks();

                    for system in &mut self.systems.pre_fixed_update_systems {
                        system.run(&mut world).unwrap()
                    }

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

                    world
                        .get_component_mut::<KappEvents>(kapp_events_entity)
                        .unwrap()
                        .clear();
                }
                _ => {}
            }
        })
    }
}
