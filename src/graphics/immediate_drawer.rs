use crate::*;

pub fn immediate_drawer_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_systems.system()],
        draw_systems: vec![draw_system.system()],
        ..Default::default()
    }
}

fn setup_systems(world: &mut World) {
    world.spawn(ImmediateDrawer::new());
}

fn draw_system(world: &mut World) {
    let immediate_drawer = world.get_singleton::<ImmediateDrawer>();
    let mut commands = Commands::new();
    std::mem::swap(&mut commands, &mut immediate_drawer.commands);
    commands.apply(world);
}

/// [ImmediateDrawer] draws things for a single frame. Useful for debug visualizations.
#[derive(NotCloneComponent)]
pub struct ImmediateDrawer {
    commands: Commands,
    color: Color,
    material: Handle<Material>,
}

impl Default for ImmediateDrawer {
    fn default() -> Self {
        Self::new()
    }
}

impl ImmediateDrawer {
    pub fn new() -> Self {
        Self {
            commands: Commands::new(),
            color: Color::WHITE,
            material: Material::DEFAULT,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_material(&mut self, material: &Handle<Material>) {
        self.material = material.clone()
    }

    pub fn draw_sphere(&mut self, transform: Transform) {
        self.commands.spawn((
            Temporary(1),
            transform,
            Mesh::SPHERE,
            self.color,
            self.material.clone(),
        ))
    }

    pub fn draw_sphere_for_n_frames(&mut self, transform: Transform, n: usize) {
        self.commands.spawn((
            Temporary(n),
            transform,
            Mesh::SPHERE,
            self.color,
            self.material.clone(),
        ))
    }

    pub fn draw_cube(&mut self, transform: Transform) {
        self.commands.spawn((
            Temporary(1),
            transform,
            Mesh::CUBE,
            self.color,
            self.material.clone(),
        ))
    }

    pub fn draw_mesh(&mut self, transform: Transform, mesh: &Handle<Mesh>) {
        self.commands.spawn((
            Temporary(1),
            transform,
            mesh.clone(),
            self.color,
            self.material.clone(),
        ))
    }

    pub fn apply(&mut self, world: &mut World) {
        self.commands.apply(world);
    }
}
