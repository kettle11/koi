pub use kui::*;

use crate::*;

pub struct UIManager {
    pub entity: Entity,
    pub drawer: kui::Drawer,
    pub initial_constraints: Box3,
    pub ui_scale: f32,
}

impl UIManager {
    pub fn new(world: &mut World) -> Self {
        let entity = world.spawn((
            Name("User Interface Visuals"),
            Transform::new(),
            Material::UI,
            RenderFlags::USER_INTERFACE,
        ));
        let drawer = kui::Drawer::new();
        Self {
            entity,
            drawer,
            initial_constraints: Box3::ZERO,
            ui_scale: 1.0,
        }
    }

    fn update_size<Style: GetStandardStyle, Input: GetStandardInput>(
        &mut self,
        world: &mut World,
        standard_context: &mut StandardContext<Style, Input>,
    ) {
        let ((window_width, window_height), ui_scale) =
            (|window: &NotSendSync<kapp::Window>| (window.size(), window.scale())).run(world);
        let (window_width, window_height, ui_scale) =
            (window_width as f32, window_height as f32, ui_scale as f32);

        let width = window_width / ui_scale;
        let height = window_height / ui_scale;
        self.ui_scale = ui_scale;
        standard_context.standard_style_mut().ui_scale = ui_scale;
        standard_context.standard_input_mut().view_size = Vec2::new(width, height);

        self.initial_constraints =
            Box3::new_with_min_corner_and_size(Vec3::ZERO, Vec3::new(width, height, f32::MAX));
    }

    fn update_input(&mut self, world: &mut World, standard_input: &mut StandardInput) {
        let input = world.get_singleton::<Input>();

        // Take input events from the world. Later we'll redefined the World's Input
        // based on the events that aren't handled.

        standard_input.input_events.clear();

        for event in input.all_events_since_last_frame() {
            match event {
                &kapp::Event::PointerDown {
                    x,
                    y,
                    source,
                    button,
                    timestamp,
                    id,
                } => standard_input.input_events.push(kapp::Event::PointerDown {
                    x: x / self.ui_scale as f64,
                    y: y / self.ui_scale as f64,
                    source,
                    button,
                    timestamp,
                    id,
                }),
                &kapp::Event::PointerUp {
                    x,
                    y,
                    source,
                    button,
                    timestamp,
                    id,
                } => standard_input.input_events.push(kapp::Event::PointerUp {
                    x: x / self.ui_scale as f64,
                    y: y / self.ui_scale as f64,
                    source,
                    button,
                    timestamp,
                    id,
                }),
                &kapp::Event::PointerMoved {
                    x,
                    y,
                    source,
                    timestamp,
                    id,
                } => standard_input.input_events.push(kapp::Event::PointerMoved {
                    x: x / self.ui_scale as f64,
                    y: y / self.ui_scale as f64,
                    source,
                    timestamp,
                    id,
                }),
                _ => standard_input.input_events.push(event.clone()),
            }
        }

        standard_input
            .input_events_handled
            .resize(standard_input.input_events.len(), false);
        for b in &mut standard_input.input_events_handled {
            *b = false;
        }
        standard_input.delta_time = world.get_singleton::<Time>().delta_seconds_f64 as f32;
    }

    fn prepare<Style: GetStandardStyle>(
        &mut self,
        world: &mut World,
        standard_context: &mut StandardContext<Style, StandardInput>,
    ) {
        if standard_context.input.text_input_rect.is_some() {
            world
                .get_singleton::<NotSendSync<kapp::Application>>()
                .start_text_input();
        } else {
            world
                .get_singleton::<NotSendSync<kapp::Application>>()
                .end_text_input();
        }

        self.update_size(world, standard_context);
        self.update_input(world, standard_context.standard_input_mut())
    }

    /// Remove all handled events from the World so that other systems don't react to them.
    fn remove_handled_events_from_world(
        &mut self,
        standard_input: &mut StandardInput,
        world: &mut World,
    ) {
        let input = world.get_singleton::<Input>();

        // Reuse [Input]'s [Vec] instead of allocating a new one.
        let mut swap_vec = Vec::new();
        std::mem::swap(&mut swap_vec, &mut input.0.all_events_since_last_frame);

        let mut new_events = Vec::new();
        for (handled, event) in standard_input.input_events_iter() {
            if !*handled {
                new_events.push(match event {
                    kapp::Event::PointerDown {
                        x,
                        y,
                        source,
                        button,
                        timestamp,
                        id,
                    } => kapp::Event::PointerDown {
                        x: x * self.ui_scale as f64,
                        y: y * self.ui_scale as f64,
                        source,
                        button,
                        timestamp,
                        id,
                    },
                    kapp::Event::PointerUp {
                        x,
                        y,
                        source,
                        button,
                        timestamp,
                        id,
                    } => kapp::Event::PointerUp {
                        x: x * self.ui_scale as f64,
                        y: y * self.ui_scale as f64,
                        source,
                        button,
                        timestamp,
                        id,
                    },
                    kapp::Event::PointerMoved {
                        x,
                        y,
                        source,
                        timestamp,
                        id,
                    } => kapp::Event::PointerMoved {
                        x: x * self.ui_scale as f64,
                        y: y * self.ui_scale as f64,
                        source,
                        timestamp,
                        id,
                    },
                    _ => event.clone(),
                })
            }
        }

        input.0.set_with_events(new_events);
    }

    fn update_layout_draw<Data, Style: GetStandardStyle>(
        &mut self,
        data: &mut Data,
        context: &mut StandardContext<Style, StandardInput>,
        root_widget: &mut impl kui::Widget<Data, StandardContext<Style, StandardInput>>,
    ) {
        let (width, height, _) = self.initial_constraints.size().into();
        self.drawer.set_view_width_height(width, height);

        root_widget.update(data, context);
        root_widget.layout(
            data,
            context,
            MinAndMaxSize {
                min: Vec3::ZERO,
                max: self.initial_constraints.size(),
            },
        );
        root_widget.draw(data, context, &mut self.drawer, self.initial_constraints);
    }

    pub fn render_ui(&mut self, world: &mut World) {
        render_ui(world, self.entity, &mut self.drawer);
    }
    pub fn update<Style: GetStandardStyle>(
        &mut self,
        world: &mut World,
        context: &mut StandardContext<Style, StandardInput>,
        root_widget: &mut impl kui::Widget<World, StandardContext<Style, StandardInput>>,
    ) {
        self.prepare(world, context);
        root_widget.update(world, context);
        self.remove_handled_events_from_world(context.standard_input_mut(), world);
    }

    pub fn layout_and_draw<Style: GetStandardStyle>(
        &mut self,
        world: &mut World,
        context: &mut StandardContext<Style, StandardInput>,
        root_widget: &mut impl kui::Widget<World, StandardContext<Style, StandardInput>>,
    ) {
        root_widget.layout(
            world,
            context,
            MinAndMaxSize {
                min: Vec3::ZERO,
                max: self.initial_constraints.size(),
            },
        );
        let (width, height, _) = self.initial_constraints.size().into();
        self.drawer.set_view_width_height(width, height);
        root_widget.draw(world, context, &mut self.drawer, self.initial_constraints);
        render_ui(world, self.entity, &mut self.drawer);
    }
}

/// Update's the UI's mesh and texture so that it can be rendered.
pub fn render_ui(world: &mut World, ui_entity: Entity, drawer: &mut kui::Drawer) {
    let mut commands = Commands::new();
    (|graphics: &mut Graphics, meshes: &mut Assets<Mesh>, textures: &mut Assets<Texture>| {
        let mesh_data = MeshData {
            positions: drawer.positions.clone(),
            indices: drawer.indices.clone(),
            colors: drawer.colors.clone(),
            texture_coordinates: drawer.texture_coordinates.clone(),
            ..Default::default()
        };

        let new_mesh_handle = meshes.add(Mesh::new(graphics, mesh_data));

        if drawer.texture_atlas.changed {
            drawer.texture_atlas.changed = false;
            let new_texture = graphics
                .new_texture(
                    Some(&drawer.texture_atlas.data),
                    drawer.texture_atlas.width as u32,
                    drawer.texture_atlas.height as u32,
                    kgraphics::PixelFormat::R8Unorm,
                    TextureSettings {
                        srgb: false,
                        ..Default::default()
                    },
                )
                .unwrap();
            let new_texture_handle = textures.add(new_texture);
            let new_sprite = Sprite::new(new_texture_handle, Box2::new(Vec2::ZERO, Vec2::ONE));
            commands.add_component(ui_entity, new_sprite)
        }

        commands.add_component(ui_entity, new_mesh_handle)
    })
    .run(world);
    commands.apply(world);
    drawer.reset();
}

pub fn run_simple_ui<Data: 'static>(
    data: Data,
    style: StandardStyle,
    fonts: kui::Fonts,
    root: impl kui::Widget<Data, StandardContext<kui::StandardStyle, kui::StandardInput>> + 'static,
) {
    App::new().setup_and_run(|world| {
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        let mut data = data;
        let mut root = root;

        let mut standard_context =
            kui::StandardContext::new(style, kui::StandardInput::default(), fonts);
        let mut ui_manager = UIManager::new(world);

        move |event, world| {
            match event {
                Event::Draw => {
                    ui_manager.prepare(world, &mut standard_context);
                    ui_manager.update_layout_draw(&mut data, &mut standard_context, &mut root);
                    ui_manager.render_ui(world);
                    ui_manager.remove_handled_events_from_world(
                        standard_context.standard_input_mut(),
                        world,
                    );
                }
                _ => {}
            }
            false
        }
    })
}
