use crate::*;
pub struct Drawer2d<'a> {
    commands: &'a mut Commands,
}

impl<'a> Drawer2d<'a> {
    pub fn new(commands: &'a mut Commands) -> Self {
        Self { commands }
    }

    pub fn image(&mut self, position: Vec3, path: &str) {
        // Todo spawn something here.
        self.commands
            .spawn((Temporary, Transform::new_with_position(position)))
    }
}
