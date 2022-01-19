use crate::*;

mod widget_children;
pub use widget_children::*;

mod text;
pub use text::*;

mod button;
pub use button::*;

mod consecutive;
pub use consecutive::*;

mod padding;
pub use padding::*;

mod fit;
pub use fit::*;

pub fn fill<Context>(color: fn(&Context) -> Color) -> Fill<Context> {
    Fill {
        color,
        rounding: |_| 0.0,
    }
}

pub fn rounded_fill<Context>(
    color: fn(&Context) -> Color,
    rounding: fn(&Context) -> f32,
) -> Fill<Context> {
    Fill { color, rounding }
}

pub fn outlined_rounded_fill<
    State,
    Context,
    Constraints: Default + Copy + GetStandardConstraints + 'static,
    Drawer: GetStandardDrawer,
>(
    outline_color: fn(&Context) -> Color,
    inner_color: fn(&Context) -> Color,
    rounding: fn(&Context) -> f32,
) -> impl Widget<State, Context, Constraints, Drawer> {
    stack((
        rounded_fill(outline_color, rounding),
        padding(|_| 2.0, rounded_fill(inner_color, |_| 7.0)),
    ))
}

pub struct Fill<Context> {
    pub color: fn(&Context) -> Color,
    pub rounding: fn(&Context) -> f32,
}

impl<State, Context, Constraints: Default + GetStandardConstraints, Drawer: GetStandardDrawer>
    Widget<State, Context, Constraints, Drawer> for Fill<Context>
{
    fn layout(&mut self, _state: &mut State, _context: &mut Context) -> Constraints {
        Constraints::default()
    }
    fn draw(
        &mut self,
        _state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Constraints,
    ) {
        drawer.standard().rounded_rectangle(
            constraints.standard().bounds,
            Vec4::fill((self.rounding)(context)),
            (self.color)(context),
        );
    }
}

/// Just a colored rectangle for debug purposes
pub struct Rectangle {
    pub size: Vec3,
    pub color: Color,
}

pub fn rectangle(size: Vec2, color: Color) -> Rectangle {
    Rectangle {
        size: size.extend(0.1),
        color,
    }
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
        let bounds = Box3::new_with_min_corner_and_size(constraints.standard().bounds.min, size);
        drawer.standard().rectangle(bounds, self.color);
    }
}
