use crate::*;
pub use kui::*;

pub fn ui_plugin() -> Plugin {
    Plugin {
        ..Default::default()
    }
}

pub struct UI<STYLE: GetStandardStyleTrait + 'static> {
    phantom: std::marker::PhantomData<STYLE>,
}

impl<Style: GetStandardStyleTrait> UI<Style> {
    pub fn new(world: &mut World, root_widget: impl WidgetTrait<Style, World>) -> Self {
        world.spawn((
            UIComponent::new(Box::new(root_widget)),
            Handle::<Mesh>::default(),
            Material::UI,
            Transform::new(),
            Sprite::new(Handle::default(), BoundingBox::ZERO),
            RenderLayers::USER_INTERFACE,
        ));

        Self {
            phantom: std::marker::PhantomData,
        }
    }

    // Call during the pre-draw step to update the UI.
    pub fn draw(&mut self, world: &mut World, style: &mut Style) {
        let mut events = Vec::new();
        (|events_in: &mut KappEvents| std::mem::swap(&mut events, &mut events_in.0)).run(world);

        let ((window_width, window_height), ui_scale) =
            (|window: &NotSendSync<kapp::Window>| (window.size(), window.scale())).run(world);
        let (window_width, window_height, ui_scale) =
            (window_width as f32, window_height as f32, ui_scale as f32);

        let mut ui_entities = Vec::new();
        (|query: Query<&mut UIComponent<Style>>| {
            for (entity, _) in query.entities_and_components() {
                ui_entities.push(*entity);
            }
        })
        .run(world);

        for entity in ui_entities {
            let mut ui = world
                .remove_component::<UIComponent<Style>>(entity)
                .unwrap();
            let mut mesh_handle = world.remove_component::<Handle<Mesh>>(entity).unwrap();
            let mut sprite = world.remove_component::<Sprite>(entity).unwrap();

            for event in &events {
                ui.handle_event(world, event);
            }

            ui.draw(
                world,
                style,
                window_width / ui_scale,
                window_height / ui_scale,
                ui_scale,
            );

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

                if ui.drawer.texture_atlas.changed {
                    ui.drawer.texture_atlas.changed = false;
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
                    sprite =
                        Sprite::new(new_texture_handle, BoundingBox::new(Vec2::ZERO, Vec2::ONE));
                }
            })
            .run(world);
            world.add_component(entity, ui).unwrap();
            world.add_component(entity, mesh_handle).unwrap();
            world.add_component(entity, sprite).unwrap();
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

#[derive(NotCloneComponent)]
pub struct UIComponent<Style: 'static> {
    root_widget: SyncGuard<Box<dyn WidgetTrait<Style, World>>>,
    drawer: Drawer,
}

impl<STYLE: GetStandardStyleTrait> UIComponent<STYLE> {
    pub fn new(root_widget: Box<dyn WidgetTrait<STYLE, World>>) -> Self {
        Self {
            root_widget: SyncGuard::new(root_widget),
            drawer: Drawer::new(),
        }
    }

    pub fn handle_event(&mut self, data: &mut World, event: &KappEvent) {
        let Self { root_widget, .. } = self;

        root_widget.inner().event(data, event);
    }

    pub fn draw(
        &mut self,
        data: &mut World,
        style: &mut STYLE,
        width: f32,
        height: f32,
        scale: f32,
    ) {
        let Self {
            root_widget,
            drawer,
        } = self;
        drawer.reset();

        drawer.set_view_width_height(width, height);
        style.standard_mut().ui_scale = scale;

        let root_widget = root_widget.inner();

        root_widget.size(style, data);
        root_widget.draw(
            style,
            data,
            drawer,
            Rectangle::new(Vec2::ZERO, Vec2::new(width, height)),
        )
    }
}
