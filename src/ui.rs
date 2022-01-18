use kui::{GetStandardConstraints, GetStandardStyle, StandardStyle};

use crate::*;

pub struct StandardState<'a, 'b, State> {
    pub user_state: &'b mut State,
    standard_style: &'a mut StandardStyle,
}

impl<'a, 'b, State> StandardState<'a, 'b, State> {
    pub fn new(user_state: &'b mut State, standard_style: &'a mut StandardStyle) -> Self {
        Self {
            user_state,
            standard_style,
        }
    }
}

impl<'a, 'b, State> GetStandardStyle for StandardState<'a, 'b, State> {
    fn standard_style(&self) -> &StandardStyle {
        &self.standard_style
    }
    fn standard_style_mut(&mut self) -> &mut StandardStyle {
        &mut self.standard_style
    }
}

pub struct UIManager<Constraints: GetStandardConstraints + Clone> {
    pub entity: Entity,
    pub drawer: kui::Drawer,
    pub initial_constraints: Constraints,
}

impl<Constraints: GetStandardConstraints + Clone> UIManager<Constraints> {
    pub fn new(world: &mut World, initial_constraints: Constraints) -> Self {
        let entity = world.spawn((Transform::new(), Material::UI, RenderFlags::USER_INTERFACE));
        let drawer = kui::Drawer::new();
        let mut s = Self {
            entity,
            drawer,
            initial_constraints,
        };
        s.update_initial_size(world);
        s
    }

    pub fn update_initial_size(&mut self, world: &mut World) {
        let ((window_width, window_height), ui_scale) =
            (|window: &NotSendSync<kapp::Window>| (window.size(), window.scale())).run(world);
        let (window_width, window_height, ui_scale) =
            (window_width as f32, window_height as f32, ui_scale as f32);

        let width = window_width / ui_scale;
        let height = window_height / ui_scale;

        self.initial_constraints.standard_mut().bounds =
            Box2::new_with_min_corner_and_size(Vec2::ZERO, Vec2::new(width, height));
    }

    pub fn update<State, Context>(
        &mut self,
        user_state: &mut State,
        context: &mut Context,
        root_widget: &mut dyn kui::Widget<State, Context, Constraints, kui::Drawer>,
    ) {
        let (width, height) = self.initial_constraints.standard().bounds.size().into();
        self.drawer.set_view_width_height(width, height);

        root_widget.update_layout_draw(
            user_state,
            context,
            &mut self.drawer,
            self.initial_constraints.clone(),
        );
    }

    pub fn draw(&mut self, world: &mut World) {
        self.update_initial_size(world);
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
