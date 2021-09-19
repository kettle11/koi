/*
mod button;
pub use button::*;

mod text;
pub use text::*;

mod column_and_row;
pub use column_and_row::*;
*/

use crate::*;
pub trait WidgetTrait<Style, Data>: Send {
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

pub fn fill<Style, Data>(color: Color) -> Box<dyn WidgetTrait<Style, Data>> {
    Box::new(Fill { color })
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

pub fn colored_rectangle<Style, Data>(
    size: Vec2,
    color: Color,
) -> Box<dyn WidgetTrait<Style, Data>> {
    Box::new(ColoredRectangle { size, color })
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
