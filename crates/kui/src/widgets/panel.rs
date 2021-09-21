use kapp_platform_common::PointerButton;

use crate::*;

pub struct Panel<
    Style,
    Data,
    FirstWidget: WidgetTrait<Style, Data>,
    SecondWidget: WidgetTrait<Style, Data>,
> {
    first_pixels: f32,
    handle_sliding: bool,
    first_widget: FirstWidget,
    second_widget: SecondWidget,
    handle_bounding_box: Rectangle,
    total_bounding_box: Rectangle,
    reverse: bool,
    phantom: std::marker::PhantomData<fn() -> (Style, Data)>,
}

pub fn panel_horizontal<
    Style: GetStandardStyleTrait,
    Data: 'static,
    FirstWidget: WidgetTrait<Style, Data>,
    SecondWidget: WidgetTrait<Style, Data>,
>(
    first_widget: FirstWidget,
    second_widget: SecondWidget,
    initial: f32,
) -> Panel<Style, Data, FirstWidget, SecondWidget> {
    Panel {
        first_pixels: initial,
        handle_sliding: false,
        first_widget,
        second_widget,
        handle_bounding_box: BoundingBox::ZERO,
        total_bounding_box: BoundingBox::ZERO,
        reverse: false,
        phantom: std::marker::PhantomData,
    }
}

pub fn panel_horizontal_reversed<
    Style: GetStandardStyleTrait,
    Data: 'static,
    FirstWidget: WidgetTrait<Style, Data>,
    SecondWidget: WidgetTrait<Style, Data>,
>(
    first_widget: FirstWidget,
    second_widget: SecondWidget,
    initial: f32,
) -> Panel<Style, Data, FirstWidget, SecondWidget> {
    let mut panel = panel_horizontal(first_widget, second_widget, initial);
    panel.reverse = true;
    panel
}

impl<
        Style: GetStandardStyleTrait + 'static,
        Data: 'static,
        FirstWidget: WidgetTrait<Style, Data>,
        SecondWidget: WidgetTrait<Style, Data>,
    > WidgetTrait<Style, Data> for Panel<Style, Data, FirstWidget, SecondWidget>
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
        let old_clipping_mask = drawer.clipping_mask;

        let handle_size = 2.;

        let (width, height) = rectangle.size().into();
        let mut first_width = self.first_pixels - handle_size / 2.0;
        let mut second_width = width - self.first_pixels - handle_size / 2.0;

        if self.reverse {
            std::mem::swap(&mut first_width, &mut second_width);
        }

        let first_rectangle =
            Rectangle::new_with_min_corner_and_size(rectangle.min, Vec2::new(first_width, height));

        let second_rectangle = Rectangle::new_with_min_corner_and_size(
            rectangle.min + Vec2::X * (first_width + handle_size / 2.0),
            Vec2::new(second_width, height),
        );

        let handle_rectangle = Rectangle::new_with_min_corner_and_size(
            rectangle.min + Vec2::X * first_width,
            Vec2::new(handle_size, height),
        );

        drawer.clipping_mask = first_rectangle;
        self.first_widget.draw(style, data, drawer, first_rectangle);

        drawer.clipping_mask = second_rectangle;
        self.second_widget
            .draw(style, data, drawer, second_rectangle);

        drawer.clipping_mask = old_clipping_mask;
        drawer.rectangle(handle_rectangle, Color::WHITE);

        let handle_padding = 10.0;
        let padded_handle = BoundingBox::new(
            handle_rectangle.min - Vec2::X * handle_padding,
            handle_rectangle.max + Vec2::X * handle_padding,
        );
        self.handle_bounding_box = padded_handle;
        self.total_bounding_box = rectangle;
    }

    fn event(&mut self, data: &mut Data, event: &Event) -> bool {
        let event_handled = match event {
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
                    true
                } else {
                    false
                }
            }
            Event::PointerMoved { x, .. } => {
                if self.handle_sliding {
                    let offset = *x as f32 - self.total_bounding_box.min.x;
                    if self.reverse {
                        self.first_pixels = self.total_bounding_box.size().x - offset;
                    } else {
                        self.first_pixels = offset;
                    }
                }
                false
            }
            Event::PointerUp {
                button: PointerButton::Primary,
                ..
            } => {
                self.handle_sliding = false;
                false
            }
            _ => false,
        };
        if !event_handled {
            let handled_first = self.first_widget.event(data, event);
            let handled_second = self.second_widget.event(data, event);
            handled_first || handled_second
        } else {
            true
        }
    }
}
