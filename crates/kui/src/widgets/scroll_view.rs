use std::marker::PhantomData;

use crate::*;

pub fn scroll_view<Style, Data, W: WidgetTrait<Style, Data>>(
    child: W,
) -> ScrollView<Style, Data, W> {
    ScrollView {
        offset: Vec2::ZERO,
        child,
        child_size: Vec2::ZERO,
        phantom: std::marker::PhantomData,
    }
}
pub struct ScrollView<Style, Data, W: WidgetTrait<Style, Data>> {
    offset: Vec2,
    child: W,
    child_size: Vec2,
    phantom: PhantomData<(Style, Data)>,
}

impl<Style: Send + 'static, Data: Send + 'static, W: WidgetTrait<Style, Data>>
    WidgetTrait<Style, Data> for ScrollView<Style, Data, W>
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        self.child_size = self.child.size(style, data);
        Vec2::MAX
    }

    fn draw(
        &mut self,
        style: &mut Style,
        data: &mut Data,
        drawer: &mut Drawer,
        mut rectangle: Box2,
    ) {
        // This behavior ensures that as the window size changes the offset value updates as appropriate.
        let mut size_diff = rectangle.size() - self.child_size;
        if size_diff.x > 0.0 {
            self.offset.x += size_diff.x;
            size_diff.x = 0.0;
        }
        self.offset.x = self.offset.x.clamp(size_diff.x, 0.0);

        if size_diff.y > 0.0 {
            self.offset.y += size_diff.y;
            size_diff.y = 0.0;
        }
        self.offset.y = self.offset.y.clamp(size_diff.y, 0.0);

        rectangle.min += self.offset;
        self.child.draw(style, data, drawer, rectangle)
    }

    fn event(&mut self, data: &mut Data, event: &Event) -> bool {
        if let Event::Scroll {
            delta_y, delta_x, ..
        } = event
        {
            self.offset.x += *delta_x as f32;
            self.offset.y += *delta_y as f32;
        }

        self.child.event(data, event)
    }
}
