use kapp_platform_common::PointerButton;

use crate::*;

pub struct Button<Style, Data> {
    on_press: fn(&mut Data),
    text: Text<Style, Data>,
    text_size: Vec2,
    hit_rectangle: Box2,
    held_down: bool,
}

pub fn button<Style: GetStandardStyleTrait, Data: 'static>(
    text: impl Into<TextSource<Data>>,
    on_press: fn(&mut Data),
) -> Button<Style, Data> {
    Button {
        on_press,
        text: Text::new(
            text,
            |style: &Style| style.standard().primary_font,
            |style: &Style| style.standard().primary_text_color,
            |style: &Style| style.standard().primary_text_size,
        ),
        text_size: Vec2::ZERO,
        hit_rectangle: Box2::ZERO,
        held_down: false,
    }
}

impl<Style: 'static + GetStandardStyleTrait, Data: 'static> WidgetTrait<Style, Data>
    for Button<Style, Data>
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        self.text_size = self.text.size(style, data);
        self.text_size + Vec2::fill(style.standard().padding) * 2.0
    }

    fn draw(&mut self, style: &mut Style, data: &mut Data, drawer: &mut Drawer, rectangle: Box2) {
        let draw_rectangle = Box2::new(
            rectangle.min,
            rectangle.min + self.text_size + Vec2::fill(style.standard().padding) * 2.0,
        );

        let color = if self.held_down {
            style.standard().primary_variant_color
        } else {
            style.standard().primary_color
        };

        self.hit_rectangle =
            drawer.rounded_rectangle(draw_rectangle, Vec4::fill(style.standard().rounding), color);
        self.text.draw(
            style,
            data,
            drawer,
            Box2::new(
                rectangle.min + Vec2::fill(style.standard().padding),
                rectangle.min + self.text_size + Vec2::fill(style.standard().padding) * 2.0,
            ),
        )
    }

    fn event(&mut self, data: &mut Data, event: &Event) -> bool {
        match event {
            Event::PointerDown {
                button: PointerButton::Primary,
                x,
                y,
                ..
            } => {
                if self
                    .hit_rectangle
                    .contains_point(Vec2::new(*x as f32, *y as f32))
                {
                    self.held_down = true;
                    (self.on_press)(data);
                    return true;
                }
            }
            Event::PointerUp {
                button: PointerButton::Primary,
                ..
            } => {
                self.held_down = false;
            }
            _ => {}
        }
        false
    }
}
