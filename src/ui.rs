use kui::{GetStandardStyle, StandardContext, StandardInput};

use crate::*;

pub struct UIManager {
    pub entity: Entity,
    pub drawer: kui::Drawer,
    pub initial_constraints: Box3,
    pub ui_scale: f32,
}

impl UIManager {
    pub fn new(world: &mut World) -> Self {
        let entity = world.spawn((Transform::new(), Material::UI, RenderFlags::USER_INTERFACE));
        let drawer = kui::Drawer::new();
        Self {
            entity,
            drawer,
            initial_constraints: Box3::ZERO,
            ui_scale: 1.0,
        }
    }

    pub fn update_size<Style: GetStandardStyle, Input>(
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

        self.initial_constraints =
            Box3::new_with_min_corner_and_size(Vec3::ZERO, Vec3::new(width, height, f32::MAX));
    }

    pub fn update_input(&mut self, world: &mut World, standard_input: &mut StandardInput) {
        let input = world.get_singleton::<Input>();
        standard_input.pointer_down = input.pointer_button_down(PointerButton::Primary);
        standard_input.pointer_position = {
            let (x, y) = input.pointer_position();
            Vec2::new(x as f32, y as f32) / self.ui_scale
        };
    }

    pub fn update<Data, Context>(
        &mut self,
        data: &mut Data,
        context: &mut Context,
        root_widget: &mut impl kui::Widget<Data, Context>,
    ) {
        let (width, height, _) = self.initial_constraints.size().into();
        self.drawer.set_view_width_height(width, height);

        root_widget.update(data, context);
        root_widget.layout(data, context);
        root_widget.draw(data, context, &mut self.drawer, self.initial_constraints);
    }

    pub fn draw(&mut self, world: &mut World) {
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
