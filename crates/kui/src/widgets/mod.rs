mod button;
pub use button::*;

mod text;
pub use text::*;

mod column_and_row;
pub use column_and_row::*;

pub trait WidgetTrait<CONTEXT: UIContextTrait>: Send {
    #[allow(unused)]
    fn size(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
    ) -> Vec2 {
        Vec2::ZERO
    }
    #[allow(unused)]
    fn draw(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
    }
    #[allow(unused)]
    fn event(&mut self, context: &mut CONTEXT, data: &mut CONTEXT::Data, event: &Event) {}
}

pub fn fill<CONTEXT: UIContextTrait>(color: Color) -> Box<dyn WidgetTrait<CONTEXT>> {
    Box::new(Fill { color })
}
pub struct Fill {
    color: Color,
}

impl<CONTEXT: UIContextTrait> WidgetTrait<CONTEXT> for Fill {
    fn size(
        &mut self,
        _context: &mut CONTEXT,
        _style: &mut CONTEXT::Style,
        _data: &mut CONTEXT::Data,
    ) -> Vec2 {
        Vec2::MAX
    }
    fn draw(
        &mut self,
        _context: &mut CONTEXT,
        _style: &mut CONTEXT::Style,
        _data: &mut CONTEXT::Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        drawer.rectangle(rectangle, self.color)
    }
}

pub fn colored_rectangle<CONTEXT: UIContextTrait>(
    size: Vec2,
    color: Color,
) -> Box<dyn WidgetTrait<CONTEXT>> {
    Box::new(ColoredRectangle { size, color })
}
pub struct ColoredRectangle {
    size: Vec2,
    color: Color,
}

impl<CONTEXT: UIContextTrait> WidgetTrait<CONTEXT> for ColoredRectangle {
    fn size(
        &mut self,
        _context: &mut CONTEXT,
        _style: &mut CONTEXT::Style,
        _data: &mut CONTEXT::Data,
    ) -> Vec2 {
        self.size
    }
    fn draw(
        &mut self,
        _context: &mut CONTEXT,
        _style: &mut CONTEXT::Style,
        _data: &mut CONTEXT::Data,
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
