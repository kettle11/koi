use crate::*;
pub use kui::*;

pub fn ui_plugin() -> Plugin {
    Plugin {
        ..Default::default()
    }
}

pub struct UI<DATA: 'static, STYLE: GetStandardStyleTrait + 'static> {
    ui_component: UIComponent<DATA, STYLE>,
}

impl<DATA, STYLE: GetStandardStyleTrait> UI<DATA, STYLE> {
    pub fn new(world: &mut World, root_widget: Box<dyn WidgetTrait<STYLE, DATA>>) -> Self {
        Self {
            ui_component: UIComponent::new(root_widget),
        }
    }

    // Call during the pre-draw step to update the UI.
    pub fn draw(&mut self, world: &World, style: &mut STYLE, data: &mut DATA) {
        let (window_width, window_height) =
            (|window: &NotSendSync<kapp::Window>| window.size()).run(world);
        let (window_width, window_height) = (window_width as f32, window_height as f32);

        (|mut query: Query<&mut UIComponent<DATA, STYLE>>, events: &KappEvents| {
            for ui in &mut query {
                for event in events.iter() {
                    ui.handle_event(data, event)
                }
            }
        })
        .run(world);

        (|mut query: Query<&mut UIComponent<DATA, STYLE>>| {
            for ui in &mut query {
                ui.draw(data, style, window_width, window_height);
            }
        })
        .run(world);

        (|mut query: Query<(
            &mut UIComponent<DATA, STYLE>,
            &mut Handle<Mesh>,
            &mut Sprite,
        )>,
          graphics: &mut Graphics,
          meshes: &mut Assets<Mesh>,
          textures: &mut Assets<Texture>| {
            for (ui, mesh_handle, sprite) in &mut query {
                let mesh_data = MeshData {
                    positions: ui.drawer.positions.clone(),
                    indices: ui.drawer.indices.clone(),
                    colors: ui.drawer.colors.clone(),
                    texture_coordinates: ui.drawer.texture_coordinates.clone(),
                    ..Default::default()
                };

                let new_mesh_handle = meshes.add(Mesh::new(graphics, mesh_data));
                *mesh_handle = new_mesh_handle;

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
                *sprite = Sprite::new(new_texture_handle, BoundingBox::new(Vec2::ZERO, Vec2::ONE));
            }
        })
        .run(world)
    }
}

// Todo: Make this Clone
#[derive(NotCloneComponent)]
pub struct UIComponent<DATA: 'static, STYLE: 'static> {
    root_widget: SyncGuard<Box<dyn WidgetTrait<STYLE, DATA>>>,
    drawer: Drawer,
}

impl<DATA, STYLE: GetStandardStyleTrait> UIComponent<DATA, STYLE> {
    pub fn new(root_widget: Box<dyn WidgetTrait<STYLE, DATA>>) -> Self {
        Self {
            root_widget: SyncGuard::new(root_widget),
            drawer: Drawer::new(),
        }
    }

    pub fn handle_event(&mut self, data: &mut DATA, event: &KappEvent) {
        let Self { root_widget, .. } = self;

        root_widget.inner().event(data, event);
    }

    pub fn draw(&mut self, data: &mut DATA, style: &mut STYLE, width: f32, height: f32) {
        let Self {
            root_widget,
            drawer,
        } = self;
        drawer.reset();

        drawer.set_view_width_height(width, height);

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
