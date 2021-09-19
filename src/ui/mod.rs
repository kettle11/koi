use crate::*;
pub use kui::*;

pub fn ui_plugin() -> Plugin {
    Plugin {
        ..Default::default()
    }
}

pub struct UI<STYLE: GetStandardStyleTrait + 'static> {
    ui_component: UIComponent<STYLE>,
}

impl<STYLE: GetStandardStyleTrait> UI<STYLE> {
    pub fn new(
        world: &mut World,
        root_widget: Box<dyn WidgetTrait<UIContext<STYLE, World>>>,
    ) -> Self {
        Self {
            ui_component: UIComponent::new(root_widget),
        }
    }

    // Call during the pre-draw step to update the UI.
    pub fn draw(&mut self, world: &mut World, style: &mut STYLE) {
        let mut events = Vec::new();
        (|events_in: &mut KappEvents| std::mem::swap(&mut events, &mut events_in.0)).run(world);

        let (window_width, window_height) =
            (|window: &NotSendSync<kapp::Window>| window.size()).run(world);
        let (window_width, window_height) = (window_width as f32, window_height as f32);

        let mut ui_entities = Vec::new();
        (|mut query: Query<&mut UIComponent<STYLE>>| {
            for (entity, _) in query.entities_and_components() {
                ui_entities.push(*entity);
            }
        })
        .run(world);

        for entity in ui_entities {
            let mut ui = world
                .remove_component::<UIComponent<STYLE>>(entity)
                .unwrap();
            let mut mesh_handle = world.remove_component::<Handle<Mesh>>(entity).unwrap();
            let mut sprite = world.remove_component::<Sprite>(entity).unwrap();

            for event in &events {
                ui.handle_event(world, event);
            }
            ui.draw(world, style, window_width, window_height);

            (|graphics: &mut Graphics,
              meshes: &mut Assets<Mesh>,
              textures: &mut Assets<Texture>| {
                let mesh_data = MeshData {
                    positions: ui.drawer.positions.clone(),
                    indices: ui.drawer.indices.clone(),
                    colors: ui.drawer.colors.clone(),
                    texture_coordinates: ui.drawer.texture_coordinates.clone(),
                    ..Default::default()
                };

                let new_mesh_handle = meshes.add(Mesh::new(graphics, mesh_data));
                mesh_handle = new_mesh_handle;

                let new_texture = graphics
                    .new_texture(
                        Some(&ui.drawer.texture_atlas.data),
                        ui.drawer.texture_atlas.width as u32,
                        ui.drawer.texture_atlas.height as u32,
                        kgraphics::PixelFormat::R8Unorm,
                        TextureSettings {
                            srgb: false,
                            ..Default::default()
                        },
                    )
                    .unwrap();
                let new_texture_handle = textures.add(new_texture);
                sprite = Sprite::new(new_texture_handle, BoundingBox::new(Vec2::ZERO, Vec2::ONE));
            })
            .run(world);
            world.add_component(entity, ui);
            world.add_component(entity, mesh_handle);
            world.add_component(entity, sprite);
        }

        (|events_in: &mut KappEvents| std::mem::swap(&mut events, &mut events_in.0)).run(world);
    }
}

pub struct UIContext<Style: 'static, Data: 'static> {
    phantom: std::marker::PhantomData<(Style, Data)>,
}

impl<Style: GetStandardStyleTrait, Data> UIContextTrait for UIContext<Style, Data> {
    type Style = Style;
    type Data = Data;
}

// Todo: Make this Clone
#[derive(NotCloneComponent)]
pub struct UIComponent<STYLE: 'static> {
    root_widget: SyncGuard<Box<dyn WidgetTrait<UIContext<STYLE, World>>>>,
    drawer: Drawer,
}

impl<STYLE: GetStandardStyleTrait> UIComponent<STYLE> {
    pub fn new(root_widget: Box<dyn WidgetTrait<UIContext<STYLE, World>>>) -> Self {
        Self {
            root_widget: SyncGuard::new(root_widget),
            drawer: Drawer::new(),
        }
    }

    pub fn handle_event(&mut self, data: &mut World, event: &KappEvent) {
        let Self { root_widget, .. } = self;

        let mut context = UIContext {
            phantom: std::marker::PhantomData,
        };
        root_widget.inner().event(&mut context, data, event);
    }

    pub fn draw(&mut self, data: &mut World, style: &mut STYLE, width: f32, height: f32) {
        let Self {
            root_widget,
            drawer,
        } = self;
        drawer.reset();

        drawer.set_view_width_height(width, height);

        let root_widget = root_widget.inner();

        let mut context = UIContext {
            phantom: std::marker::PhantomData,
        };

        root_widget.size(&mut context, style, data);
        root_widget.draw(
            &mut context,
            style,
            data,
            drawer,
            Rectangle::new(Vec2::ZERO, Vec2::new(width, height)),
        )
    }
}
