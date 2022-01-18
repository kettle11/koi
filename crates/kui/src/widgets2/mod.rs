use crate::*;

mod widget_children;
pub use widget_children::*;

mod text;
pub use text::*;

mod consecutive;
pub use consecutive::*;

pub const fn fill(color: Color) -> Fill {
    Fill { color }
}
pub struct Fill {
    pub color: Color,
}

impl<State, Context, Constraints: Default + GetStandardConstraints, Drawer: GetStandardDrawer>
    Widget<State, Context, Constraints, Drawer> for Fill
{
    fn layout(&mut self, _state: &mut State, _context: &mut Context) -> Constraints {
        Constraints::default()
    }
    fn draw(
        &mut self,
        _state: &mut State,
        _context: &mut Context,
        drawer: &mut Drawer,
        constraints: Constraints,
    ) {
        drawer
            .standard()
            .rectangle(constraints.standard().bounds, self.color);
    }
}

/// Just a colored rectangle for debug purposes
pub struct Rectangle {
    pub size: Vec2,
    pub color: Color,
}

pub fn rectangle(size: Vec2, color: Color) -> Rectangle {
    Rectangle { size, color }
}

impl<State, Context, Constraints: Default + GetStandardConstraints, Drawer: GetStandardDrawer>
    Widget<State, Context, Constraints, Drawer> for Rectangle
{
    fn layout(&mut self, _state: &mut State, _context: &mut Context) -> Constraints {
        let mut constraints = Constraints::default();
        constraints.standard_mut().set_size(self.size);
        constraints
    }
    fn draw(
        &mut self,
        _state: &mut State,
        _context: &mut Context,
        drawer: &mut Drawer,
        constraints: Constraints,
    ) {
        let size = constraints.standard().bounds.size().min(self.size);
        let bounds = Box2::new_with_min_corner_and_size(constraints.standard().bounds.min, size);
        drawer.standard().rectangle(bounds, self.color);
    }
}
