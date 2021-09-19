use crate::*;

/*
mod button;
pub use button::*;

mod text;
pub use text::*;

mod column_and_row;
pub use column_and_row::*;
*/

pub trait WidgetTrait<Style, Data>: Send + 'static {
    #[allow(unused)]
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        Vec2::ZERO
    }
    #[allow(unused)]
    fn draw(
        &mut self,

        style: &mut Style,
        data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
    }
    #[allow(unused)]
    fn event(&mut self, data: &mut Data, event: &Event) {}
}

pub const fn fill(color: Color) -> Fill {
    Fill { color }
}
pub struct Fill {
    color: Color,
}

impl<Style, Data> WidgetTrait<Style, Data> for Fill {
    fn size(&mut self, _style: &mut Style, _data: &mut Data) -> Vec2 {
        Vec2::MAX
    }
    fn draw(
        &mut self,

        _style: &mut Style,
        _data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        drawer.rectangle(rectangle, self.color)
    }
}

pub fn colored_rectangle(size: Vec2, color: Color) -> ColoredRectangle {
    ColoredRectangle { size, color }
}
pub struct ColoredRectangle {
    size: Vec2,
    color: Color,
}

impl<Style, Data> WidgetTrait<Style, Data> for ColoredRectangle {
    fn size(&mut self, _style: &mut Style, _data: &mut Data) -> Vec2 {
        self.size
    }
    fn draw(
        &mut self,

        _style: &mut Style,
        _data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        let size = rectangle.size().min(self.size);
        drawer.rectangle(
            Rectangle::new(rectangle.min, rectangle.min + size),
            self.color,
        )
    }
}
