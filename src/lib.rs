use std::ops::{Deref, DerefMut};

pub use kcolor::*;
pub use kecs::hierarchy::HierarchyNode;
pub use kecs::*;
pub use kserde::*;

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

mod transform;
pub use transform::*;

mod commands;
pub use commands::*;

mod temporary;
pub use temporary::*;

mod input;
pub use input::*;

mod world_assets;
pub use world_assets::*;

mod ecs_components;
pub use ecs_components::*;

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

#[cfg(feature = "ui")]
mod ui;
#[cfg(feature = "ui")]
pub use ui::*;

#[cfg(feature = "physics")]
mod physics;
#[cfg(feature = "physics")]
pub use physics::*;

pub use kapp::{Event as KappEvent, Key, PointerButton};

#[cfg(target_arch = "wasm32")]
use kwasm::libraries::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
pub use kwasm;

pub use klog;

#[cfg(feature = "tracing_allocator")]
pub use ktracing_allocator::*;

// Setup our own allocator to track total memory usage.
#[cfg(feature = "tracing_allocator")]
#[global_allocator]
static GLOBAL_ALLOCATOR: ktracing_allocator::TracingAllocator<std::alloc::System> =
    ktracing_allocator::TracingAllocator(std::alloc::System);

/// Keeps track of the app's systems and title.
pub struct App {
    systems: Plugin,
    title: String,
}

#[derive(Default)]
pub struct Plugin {
    pub setup_systems: Vec<System>,
    pub pre_fixed_update_systems: Vec<System>,
    pub fixed_update_systems: Vec<System>,
    pub pre_draw_systems: Vec<System>,
    pub draw_systems: Vec<System>,
    pub end_of_frame_systems: Vec<System>,
    pub on_kapp_events: Vec<System>,
    pub additional_control_flow: Vec<Box<dyn FnMut(&mut KoiState, KappEvent) -> bool>>,
}

impl Plugin {
    fn append(&mut self, other: Self) {
        let Self {
            mut setup_systems,
            mut pre_fixed_update_systems,
            mut fixed_update_systems,
            mut pre_draw_systems,
            mut draw_systems,
            mut end_of_frame_systems,
            mut on_kapp_events,
            mut additional_control_flow,
        } = other;

        self.setup_systems.append(&mut setup_systems);
        self.pre_fixed_update_systems
            .append(&mut pre_fixed_update_systems);
        self.fixed_update_systems.append(&mut fixed_update_systems);
        self.pre_draw_systems.append(&mut pre_draw_systems);
        self.draw_systems.append(&mut draw_systems);
        self.end_of_frame_systems.append(&mut end_of_frame_systems);
        self.on_kapp_events.append(&mut on_kapp_events);
        self.additional_control_flow
            .append(&mut additional_control_flow);
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let s = Self {
            systems: Plugin::default(),
            title: "Koi".to_string(),
        };
        s.add_default_plugins()
    }

    /// Set the app's title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Adds a [Plugin] to this system.
    pub fn add_plugin(mut self, plugin: Plugin) -> Self {
        self.systems.append(plugin);
        self
    }

    /// Adds standard koi plugins.
    /// Some can be toggled on / off based on feature flags.
    #[allow(clippy::let_and_return)]
    pub fn add_default_plugins(self) -> Self {
        let app = self;
        let app = app.add_plugin(world_assets_plugin());
        let app = app.add_plugin(transform_plugin());

        // Default plugins
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
        // #[cfg(feature = "ui")]
        // let app = app.add_plugin(ui_plugin());
        #[cfg(feature = "physics")]
        let app = app.add_plugin(physics_plguin());

        // Non-default plugins
        #[cfg(feature = "xr")]
        let app = app.add_plugin(xr_plugin());
        app
    }

    /// Pass in a 'setup' function that returns a 'run' function.
    /// The setup functon is called once and gives the program a chance to initialize things.
    /// The 'run' function is called continuously with [Event]s until shutdown.
    pub fn setup_and_run<S: FnMut(Event, &mut World) -> bool + 'static>(
        mut self,
        setup_and_run_function: impl FnOnce(&mut World) -> S,
    ) {
        #[cfg(feature = "tracing_allocator")]
        ktracing_allocator::set_alloc_error_hook();

        // Todo: Base this on number of cores
        ktasks::create_workers();

        let mut world = World::new();
        world.spawn((Name("Commands"), Commands::new()));
        // Setup input
        let input_entity = world.spawn((Name("Input"), Input::new()));

        let kapp_events_entity = world.spawn((Name("KappEvents"), KappEvents(Vec::new())));

        // For now `kapp` is integrated directly into `koi`
        let (kapp_app, kapp_event_loop) = kapp::initialize();

        world.spawn((Name("Kapp Application"), NotSendSync::new(kapp_app.clone())));

        let window_width = 1600;
        let window_height = 1200;

        // For now only a single window is suppported.
        let window = kapp_app
            .new_window()
            .title(&self.title)
            .size(window_width, window_height)
            .build()
            .unwrap();

        window.request_redraw();

        let window_entity = world.spawn((Name("Window"), NotSendSync::new(window)));

        for setup_system in &mut self.systems.setup_systems {
            setup_system.run(&mut world)
        }

        // Setup time tracking
        let start = Instant::now();
        let time_acumulator = 0.0;

        // Hard-coded to 60 fixed updates per second for now.
        let fixed_time_step = 1.0 / 60.0;

        world.spawn((
            Name("Time"),
            Time {
                // Set the delta_time to fixed_time_delta so that a fixed update runs for the first frame.
                delta_seconds_f64: fixed_time_step,
                fixed_time_step,
            },
        ));

        let run_system = Box::new(setup_and_run_function(&mut world));

        let mut koi_state = KoiState {
            world,
            systems: self.systems,
            start,
            time_acumulator,
            fixed_time_step,
            run_system,
            input_entity,
            window_entity,
            kapp_events_entity,
        };
        kapp_event_loop.run(move |event| {
            koi_state.handle_event(event.clone());
            match event {
                KappEvent::WindowCloseRequested { .. } => kapp_app.quit(),
                KappEvent::Draw { .. } => {}
                _ => {}
            }
        })
    }
}

pub struct KoiState {
    pub world: World,
    pub systems: Plugin,
    pub start: Instant,
    pub time_acumulator: f64,
    pub fixed_time_step: f64,
    pub run_system: Box<dyn FnMut(Event, &mut World) -> bool>,
    pub input_entity: Entity,
    pub window_entity: Entity,
    pub kapp_events_entity: Entity,
}

impl KoiState {
    fn handle_event(&mut self, event: KappEvent) {
        ktasks::run_only_local_tasks();

        // Run user callback and give it a chance to consume the event.
        if (self.run_system)(crate::Event::KappEvent(event.clone()), &mut self.world) {
            return;
        }

        let input = self
            .world
            .get_component_mut::<Input>(self.input_entity)
            .unwrap();
        input.0.handle_event(&event);

        self.world
            .get_component_mut::<KappEvents>(self.kapp_events_entity)
            .unwrap()
            .push(event.clone());

        for system in &mut self.systems.on_kapp_events {
            system.run(&mut self.world)
        }

        // Run additional control flow.
        // This is used by things like XR that need to control overall program flow.
        let mut consumed_event = false;
        let mut swap = Vec::new();
        std::mem::swap(&mut self.systems.additional_control_flow, &mut swap);
        for additional_control_flow in &mut swap {
            if (additional_control_flow)(self, event.clone()) {
                consumed_event = true;
                break;
            }
        }
        std::mem::swap(&mut self.systems.additional_control_flow, &mut swap);

        if !consumed_event {
            if let KappEvent::Draw { .. } = event {
                self.draw()
            }
        }
    }

    pub fn draw(&mut self) {
        for system in &mut self.systems.pre_fixed_update_systems {
            system.run(&mut self.world)
        }

        let elapsed = self.start.elapsed();
        let time_elapsed_seconds = elapsed.as_secs_f64();
        //klog::log!("TIME ELAPSED: {:?}", elapsed.as_millis());
        self.start = Instant::now();
        self.time_acumulator += time_elapsed_seconds;

        // Check that there aren't a huge number of fixed time steps to process.
        // This can happen if a computer goes to sleep and then exits sleep.
        if self.time_acumulator / self.fixed_time_step > 30. {
            self.time_acumulator = 0.0;
        }

        while self.time_acumulator >= self.fixed_time_step {
            (self.run_system)(crate::Event::FixedUpdate, &mut self.world);
            apply_commands(&mut self.world);
            for system in &mut self.systems.fixed_update_systems {
                system.run(&mut self.world);

                // Clear Input after each FixedUpdate. This means if there are multiple FixedUpdates per frame
                // only the first will receive input events.
                // Todo: It would be better if Input was updated based on an event's timestamp. Each FixedUpdate would progress time
                // and only events that occurred before that time would progress the input.
                self.world
                    .get_component_mut::<Input>(self.input_entity)
                    .unwrap()
                    .0
                    .clear();
            }
            apply_commands(&mut self.world);
            self.time_acumulator -= self.fixed_time_step;
        }

        apply_commands(&mut self.world);
        for system in &mut self.systems.pre_draw_systems {
            system.run(&mut self.world)
        }

        (self.run_system)(crate::Event::Draw, &mut self.world);
        apply_commands(&mut self.world);
        for system in &mut self.systems.draw_systems {
            system.run(&mut self.world)
        }
        apply_commands(&mut self.world);

        // Run systems after the last draw.
        for system in &mut self.systems.end_of_frame_systems {
            system.run(&mut self.world)
        }
        apply_commands(&mut self.world);

        self.world
            .get_component_mut::<KappEvents>(self.kapp_events_entity)
            .unwrap()
            .clear();
    }
}
