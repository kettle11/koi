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

pub struct Fill<Context> {
    pub color: fn(&Context) -> Color,
    pub rounding: fn(&Context) -> f32,
}

impl<Data, Context> Widget<Data, Context> for Fill<Context> {
    fn layout(&mut self, _data: &mut Data, _context: &mut Context) -> Vec3 {
        Vec3::ZERO
    }
    fn draw(&mut self, _data: &mut Data, context: &mut Context, drawer: &mut Drawer, bounds: Box3) {
        drawer.standard().rounded_rectangle(
            bounds,
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

impl<Data, Context> Widget<Data, Context> for Rectangle {
    fn layout(&mut self, _state: &mut Data, _context: &mut Context) -> Vec3 {
        self.size
    }
    fn draw(
        &mut self,
        _state: &mut Data,
        _context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let size = bounds.size().min(self.size);
        let bounds = Box3::new_with_min_corner_and_size(bounds.min, size);
        drawer.standard().rectangle(bounds, self.color);
    }
}

pub fn outlined_rounded_fill<State, Context>(
    outline_color: fn(&Context) -> Color,
    inner_color: fn(&Context) -> Color,
    rounding: fn(&Context) -> f32,
) -> impl Widget<State, Context> {
    stack((
        rounded_fill(outline_color, rounding),
        padding(|_| 2.0, rounded_fill(inner_color, |_| 7.0)),
    ))
}
