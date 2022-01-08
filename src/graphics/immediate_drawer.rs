use crate::*;

/// [ImmediateDrawer] draws things for a single frame. Useful for debug visualizations.
pub struct ImmediateDrawer {
    commands: Commands,
    color: Color,
    material: Handle<Material>,
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
            Temporary,
            transform,
            Mesh::SPHERE,
            self.color,
            self.material.clone(),
        ))
    }

    pub fn draw_cube(&mut self, transform: Transform) {
        self.commands.spawn((
            Temporary,
            transform,
            Mesh::SPHERE,
            self.color,
            self.material.clone(),
        ))
    }

    pub fn draw_mesh(&mut self, transform: Transform, mesh: &Handle<Mesh>) {
        self.commands.spawn((
            Temporary,
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
