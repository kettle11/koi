use crate::*;

mod button;
pub use button::*;

mod text;
pub use text::*;

mod column_and_row;
pub use column_and_row::*;

mod slider;
pub use slider::*;

mod panel;
pub use panel::*;

mod padding;
pub use padding::*;

mod scroll_view;
pub use scroll_view::*;

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
    fn event(&mut self, data: &mut Data, event: &Event) -> bool {
        false
    }
}

pub const fn fill(color: Color) -> Fill {
    Fill { color }
}
pub struct Fill {
    pub color: Color,
}

impl<Style, Data> WidgetTrait<Style, Data> for Fill {
    fn size(&mut self, _style: &mut Style, _data: &mut Data) -> Vec2 {
        // Takes up no space but will render to all available space allocated.
        Vec2::ZERO
    }
    fn draw(
        &mut self,

        _style: &mut Style,
        _data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        drawer.rectangle(rectangle, self.color);
    }
}

pub fn colored_rectangle(size: Vec2, color: Color) -> ColoredRectangle {
    ColoredRectangle { size, color }
}
pub struct ColoredRectangle {
    pub size: Vec2,
    pub color: Color,
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
        );
    }
}
use std::ops::DerefMut;

pub struct Empty;
impl<Style, Data> WidgetTrait<Style, Data> for Empty {}

impl<Style: 'static, Data: 'static> WidgetTrait<Style, Data> for Box<dyn WidgetTrait<Style, Data>> {
    fn draw(
        &mut self,
        style: &mut Style,
        data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        self.deref_mut().draw(style, data, drawer, rectangle)
    }

    fn event(&mut self, data: &mut Data, event: &Event) -> bool {
        self.deref_mut().event(data, event)
    }

    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        self.deref_mut().size(style, data)
    }
}

pub trait ToDynWidget<Style, Data> {
    fn to_dyn_widget(self) -> Box<dyn WidgetTrait<Style, Data>>;
}

impl<Style, Data, W: WidgetTrait<Style, Data>> ToDynWidget<Style, Data> for W {
    fn to_dyn_widget(self) -> Box<dyn WidgetTrait<Style, Data>> {
        Box::new(self)
    }
}
