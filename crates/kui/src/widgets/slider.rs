use std::ops::Range;

use crate::*;

pub fn slider<Data: 'static>(
    range: Range<f32>,
    value: impl Fn(&mut Data) -> &mut f32 + 'static + Send,
) -> Slider<Data> {
    Slider {
        min: range.start,
        max: range.end,
        hit_box: Rect::ZERO,
        value: Box::new(value),
        sliding: false,
    }
}

pub struct Slider<Data> {
    min: f32,
    max: f32,
    hit_box: Rect,
    value: Box<dyn Fn(&mut Data) -> &mut f32 + Send>,
    sliding: bool,
}

impl<Style: GetStandardStyleTrait + 'static, Data: 'static> WidgetTrait<Style, Data>
    for Slider<Data>
{
    fn size(&mut self, _style: &mut Style, _data: &mut Data) -> Vec2 {
        Vec2::fill(f32::INFINITY)
    }

    fn draw(&mut self, style: &mut Style, data: &mut Data, drawer: &mut Drawer, rectangle: Rect) {
        let default_style = style.standard();

        let value = (self.value)(data);
        let max_range = self.max - self.min;
        let percent = (*value - self.min) / max_range;
        let height = 100.;

        let bar_rectangle = Rect::new_with_min_corner_and_size(
            rectangle.min,
            Vec2::new(rectangle.size().x, height),
        );
        drawer.rectangle(bar_rectangle, default_style.primary_color);
        drawer.rectangle(
            Rect::new_with_min_corner_and_size(
                rectangle.min + Vec2::X * rectangle.size().x * percent - Vec2::X * 50.,
                Vec2::new(100., height),
            ),
            Color::GREEN,
        );
        self.hit_box = bar_rectangle;
    }

    fn event(&mut self, data: &mut Data, event: &kapp_platform_common::Event) -> bool {
        match event {
            kapp_platform_common::Event::PointerDown {
                button: kapp_platform_common::PointerButton::Primary,
                x,
                y,
                ..
            } => {
                let x = *x as f32;
                let y = *y as f32;
                let hit_box = self.hit_box;
                if hit_box.contains_point(Vec2::new(x, y)) {
                    let offset = (x - hit_box.min.x) / hit_box.size().x;
                    let value = (self.value)(data);
                    *value = offset * (self.max - self.min);
                    self.sliding = true;
                }
                true
            }
            kapp_platform_common::Event::PointerMoved { x, .. } => {
                if self.sliding {
                    let x = *x as f32;
                    let hit_box = self.hit_box;
                    let offset = (x - hit_box.min.x) / hit_box.size().x;
                    let value = (self.value)(data);
                    *value = (offset * (self.max - self.min)).max(self.min).min(self.max);
                }
                false
            }
            kapp_platform_common::Event::PointerUp { .. } => {
                self.sliding = false;
                false
            }
            _ => false,
        }
    }
}
