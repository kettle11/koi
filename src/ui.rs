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
            Name("User Interface Visuals".into()),
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

    fn update_size<State>(
        &mut self,
        world: &mut World,
        standard_context: &mut StandardContext<State>,
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

    pub fn handle_event<Data>(
        &mut self,
        event: &KappEvent,
        data: &mut Data,
        standard_context: &mut StandardContext<Data>,
    ) -> bool {
        match event {
            &kapp::Event::PointerDown {
                x,
                y,
                source,
                button,
                timestamp,
                id,
            } => {
                let event = kapp::Event::PointerDown {
                    x: x / self.ui_scale as f64,
                    y: y / self.ui_scale as f64,
                    source,
                    button,
                    timestamp,
                    id,
                };
                standard_context.event_handlers.handle_pointer_event(
                    &event,
                    data,
                    Vec2::new(x as f32, y as f32) / self.ui_scale,
                )
            }
            &kapp::Event::PointerMoved {
                x,
                y,
                source,
                timestamp,
                id,
            } => {
                let event = kapp::Event::PointerMoved {
                    x: x / self.ui_scale as f64,
                    y: y / self.ui_scale as f64,
                    source,
                    timestamp,
                    id,
                };
                standard_context.event_handlers.handle_pointer_event(
                    &event,
                    data,
                    Vec2::new(x as f32, y as f32) / self.ui_scale,
                )
            }
            &kapp::Event::PointerUp {
                x,
                y,
                source,
                button,
                timestamp,
                id,
            } => {
                let event = kapp::Event::PointerUp {
                    x: x / self.ui_scale as f64,
                    y: y / self.ui_scale as f64,
                    source,
                    button,
                    timestamp,
                    id,
                };
                standard_context.event_handlers.handle_pointer_event(
                    &event,
                    data,
                    Vec2::new(x as f32, y as f32) / self.ui_scale,
                )
            }
            _ => false,
        }
    }

    pub fn prepare<Data>(
        &mut self,
        world: &mut World,
        standard_context: &mut StandardContext<Data>,
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
    }

    pub fn render_ui(&mut self, world: &mut World) {
        render_ui(world, self.entity, &mut self.drawer);
    }

    pub fn layout<State>(
        &mut self,
        state: &mut State,
        context: &mut StandardContext<State>,
        root_widget: &mut impl kui::Widget<State, StandardContext<State>, ()>,
    ) {
        context.event_handlers.clear();

        root_widget.layout(
            state,
            &mut (),
            context,
            MinAndMaxSize {
                min: Vec3::ZERO,
                max: self.initial_constraints.size(),
            },
        );
        let (width, height, _) = self.initial_constraints.size().into();
        self.drawer.set_view_width_height(width, height);
        root_widget.draw(
            state,
            &mut (),
            context,
            &mut self.drawer,
            self.initial_constraints,
        );
    }

    pub fn layout_and_draw_with_world(
        &mut self,
        world: &mut World,
        context: &mut StandardContext<World>,
        root_widget: &mut impl kui::Widget<World, StandardContext<World>, ()>,
    ) {
        self.prepare(world, context);
        self.layout(world, context, root_widget);
        self.render_ui(world)
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
    root: impl kui::Widget<Data, StandardContext<Data>, ()> + 'static,
) {
    let root = stack((fill(|_, _, _| Color::WHITE), root));
    App::new().setup_and_run(|world| {
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        let mut data = data;
        let mut root = root;

        let mut standard_context =
            kui::StandardContext::new(style, kui::StandardInput::default(), fonts);
        let mut ui_manager = UIManager::new(world);

        move |event, world| match event {
            Event::KappEvent(event) => {
                ui_manager.handle_event(&event, &mut data, &mut standard_context)
            }
            Event::Draw => {
                ui_manager.prepare(world, &mut standard_context);
                ui_manager.layout(&mut data, &mut standard_context, &mut root);
                ui_manager.render_ui(world);
                false
            }
            _ => false,
        }
    })
}
