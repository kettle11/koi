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

mod toggle;
pub use toggle::*;

mod text_box;
pub use text_box::*;

mod align;
pub use align::*;

mod conditional;
pub use conditional::*;

mod ignore_size;
pub use ignore_size::*;

mod fill;
pub use fill::*;

mod not_enough_space;
pub use not_enough_space::*;

mod center;
pub use center::*;

/// Just a colored rectangle for debug purposes
pub struct Rectangle {
    pub size: Vec3,
}

pub fn rectangle(size: Vec2) -> Rectangle {
    Rectangle {
        size: size.extend(0.1),
    }
}

pub fn colored_rectangle<State, Context: GetStandardInput>(
    size: Vec2,
    color: impl Fn(&mut State, &Context) -> Color,
) -> impl Widget<State, Context> {
    stack((
        Rectangle {
            size: size.extend(0.1),
        },
        fill(color),
    ))
}

impl<State, Context> Widget<State, Context> for Rectangle {
    fn layout(
        &mut self,
        _state: &mut State,
        _context: &mut Context,
        _min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.size
    }
    fn draw(
        &mut self,
        _state: &mut State,
        _context: &mut Context,
        _drawer: &mut Drawer,
        _bounds: Box3,
    ) {
    }
}

pub fn outlined_rounded_fill<State, Context: GetStandardInput>(
    outline_color: impl Fn(&mut State, &Context) -> Color,
    inner_color: impl Fn(&mut State, &Context) -> Color,
    rounding: impl Fn(&mut State, &Context) -> f32,
) -> impl Widget<State, Context> {
    stack((
        rounded_fill(outline_color, rounding),
        padding(|_| 2.0, rounded_fill(inner_color, |_, _| 0.0)),
    ))
}

pub struct Empty;
pub fn empty() -> Empty {
    Empty
}

impl<State, Context> Widget<State, Context> for Empty {
    fn update(&mut self, _data: &mut State, _context: &mut Context) {}
    fn layout(
        &mut self,
        _data: &mut State,
        _context: &mut Context,
        _min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        Vec3::ZERO
    }
    fn draw(
        &mut self,
        _data: &mut State,
        _context: &mut Context,
        _drawer: &mut Drawer,
        _bounds: Box3,
    ) {
    }
}
