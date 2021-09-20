use kapp_platform_common::PointerButton;

use crate::*;

pub struct Split<
    Style,
    Data,
    FirstWidget: WidgetTrait<Style, Data>,
    SecondWidget: WidgetTrait<Style, Data>,
> {
    first_percent: f32,
    handle_sliding: bool,
    first_widget: FirstWidget,
    second_widget: SecondWidget,
    handle_bounding_box: Rectangle,
    total_bounding_box: Rectangle,
    phantom: std::marker::PhantomData<fn() -> (Style, Data)>,
}

pub fn split_horizontal<
    Style: GetStandardStyleTrait,
    Data: 'static,
    FirstWidget: WidgetTrait<Style, Data>,
    SecondWidget: WidgetTrait<Style, Data>,
>(
    first_widget: FirstWidget,
    second_widget: SecondWidget,
    initial: f32,
) -> Split<Style, Data, FirstWidget, SecondWidget> {
    Split {
        first_percent: initial,
        handle_sliding: false,
        first_widget,
        second_widget,
        handle_bounding_box: BoundingBox::ZERO,
        total_bounding_box: BoundingBox::ZERO,
        phantom: std::marker::PhantomData,
    }
}

impl<
        Style: GetStandardStyleTrait + 'static,
        Data: 'static,
        FirstWidget: WidgetTrait<Style, Data>,
        SecondWidget: WidgetTrait<Style, Data>,
    > WidgetTrait<Style, Data> for Split<Style, Data, FirstWidget, SecondWidget>
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        let _ = self.first_widget.size(style, data);
        let _ = self.second_widget.size(style, data);
        Vec2::MAX
    }

    fn draw(
        &mut self,
        style: &mut Style,
        data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        let handle_size = 2.;

        let (width, height) = rectangle.size().into();
        let first_width = width * self.first_percent - handle_size / 2.0;
        let second_width = width * (1.0 - self.first_percent) - handle_size / 2.0;

        let first_rectangle =
            Rectangle::new_with_min_corner_and_size(rectangle.min, Vec2::new(first_width, height));

        let handle_rectangle = Rectangle::new_with_min_corner_and_size(
            rectangle.min + Vec2::X * first_width,
            Vec2::new(handle_size, height),
        );

        let second_rectangle = Rectangle::new_with_min_corner_and_size(
            rectangle.min + Vec2::X * (first_width + handle_size / 2.0),
            Vec2::new(second_width, height),
        );

        self.first_widget.draw(style, data, drawer, first_rectangle);
        self.second_widget
            .draw(style, data, drawer, second_rectangle);
        drawer.rectangle(handle_rectangle, Color::from_srgb_hex(0x5B5B5B, 1.0));

        let handle_padding = 10.0;
        let padded_handle = BoundingBox::new(
            handle_rectangle.min - Vec2::X * handle_padding,
            handle_rectangle.max + Vec2::X * handle_padding,
        );
        self.handle_bounding_box = padded_handle;
        self.total_bounding_box = rectangle;
    }

    fn event(&mut self, data: &mut Data, event: &Event) {
        match event {
            Event::PointerDown {
                x,
                y,
                button: PointerButton::Primary,
                ..
            } => {
                if self
                    .handle_bounding_box
                    .contains_point(Vec2::new(*x as f32, *y as f32))
                {
                    self.handle_sliding = true;
                }
            }
            Event::PointerMoved { x, .. } => {
                if self.handle_sliding {
                    let offset = *x as f32 - self.total_bounding_box.min.x;
                    let percent = offset / self.total_bounding_box.size().x;
                    self.first_percent = percent;
                }
            }
            Event::PointerUp {
                button: PointerButton::Primary,
                ..
            } => {
                self.handle_sliding = false;
            }
            _ => {}
        }
        self.first_widget.event(data, event);
        self.second_widget.event(data, event);
    }
}
