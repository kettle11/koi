use kapp_platform_common::PointerButton;

pub use crate::*;

pub struct Button<CONTEXT: UIContextTrait> {
    on_press: fn(&mut CONTEXT::Data),
    text: Text<CONTEXT::Data, CONTEXT::Style>,
    text_size: Vec2,
    hit_rectangle: BoundingBox<f32, 2>,
    held_down: bool,
}

pub fn button<CONTEXT: UIContextTrait>(
    text: impl Into<TextSource<CONTEXT::Data>>,
    on_press: fn(&mut CONTEXT::Data),
) -> Box<dyn WidgetTrait<CONTEXT>> {
    Box::new(Button {
        on_press,
        text: Text::new(
            text,
            |style: &CONTEXT::Style| style.standard().primary_font,
            |style: &CONTEXT::Style| style.standard().primary_text_color,
        ),
        text_size: Vec2::ZERO,
        hit_rectangle: BoundingBox::ZERO,
        held_down: false,
    })
}

impl<CONTEXT: UIContextTrait> WidgetTrait<CONTEXT> for Button<CONTEXT> {
    fn size(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
    ) -> Vec2 {
        self.text_size = self.text.size(context, style, data);
        self.text_size + Vec2::fill(style.standard().padding) * 2.0
    }

    fn draw(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        self.hit_rectangle = Rectangle::new(
            rectangle.min,
            rectangle.min + self.text_size + Vec2::fill(style.standard().padding) * 2.0,
        );

        let color = if self.held_down {
            style.standard().primary_variant_color
        } else {
            style.standard().primary_color
        };

        drawer.rounded_rectangle(
            self.hit_rectangle,
            Vec4::fill(style.standard().rounding),
            color,
        );
        self.text.draw(
            context,
            style,
            data,
            drawer,
            Rectangle::new(
                rectangle.min + Vec2::fill(style.standard().padding),
                rectangle.min + self.text_size + Vec2::fill(style.standard().padding) * 2.0,
            ),
        )
    }

    fn event(&mut self, _context: &mut CONTEXT, data: &mut CONTEXT::Data, event: &Event) {
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
                    (self.on_press)(data)
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
    }
}
