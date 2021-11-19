use std::marker::PhantomData;

use crate::*;

pub fn padding<Style, Data, W: WidgetTrait<Style, Data>>(
    amount: f32,
    child: W,
) -> Padding<Style, Data, W> {
    Padding {
        amount,
        child,
        phantom: std::marker::PhantomData,
    }
}
pub struct Padding<Style, Data, W: WidgetTrait<Style, Data>> {
    amount: f32,
    pub child: W,
    phantom: PhantomData<(Style, Data)>,
}

impl<Style: Send + 'static, Data: Send + 'static, W: WidgetTrait<Style, Data>>
    WidgetTrait<Style, Data> for Padding<Style, Data, W>
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        self.child.size(style, data) + Vec2::fill(self.amount) * 2.0
    }

    fn draw(&mut self, style: &mut Style, data: &mut Data, drawer: &mut Drawer, rectangle: Box2) {
        self.child.draw(
            style,
            data,
            drawer,
            Box2::new(
                rectangle.min + Vec2::fill(self.amount),
                rectangle.max - Vec2::fill(self.amount),
            ),
        )
    }

    fn event(&mut self, data: &mut Data, event: &Event) -> bool {
        self.child.event(data, event)
    }
}
