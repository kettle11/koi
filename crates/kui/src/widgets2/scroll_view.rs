use std::cell::RefCell;

use crate::*;

pub fn vertical_scroll_view<
    State: 'static,
    Context: GetStandardInput + GetEventHandlers<State> + GetStandardStyle,
    ExtraState,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    scroll_view(false, true, child_widget)
}

pub fn horizontal_scroll_view<
    State: 'static,
    Context: GetStandardInput + GetEventHandlers<State> + GetStandardStyle,
    ExtraState,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    scroll_view(true, false, child_widget)
}
pub fn scroll_view<
    State: 'static,
    Context: GetStandardInput + GetEventHandlers<State> + GetStandardStyle,
    ExtraState,
>(
    horizontal: bool,
    vertical: bool,
    child_widget: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    let offset = Rc::new(RefCell::new(Vec2::ZERO));

    let offset0 = offset.clone();
    on_scroll(
        move |_, delta_x, delta_y| {
            let mut offset = offset.borrow_mut();
            if horizontal {
                offset.x += delta_x;
            }

            if vertical {
                offset.y += delta_y;
            }
        },
        //stack((
        ScrollView {
            child: child_widget,
            offset: offset0,
            horizontal,
            vertical,
            child_size: Vec3::ZERO,
            phantom: std::marker::PhantomData,
        },
        /*
        align(
            Alignment::End,
            Alignment::Start,
            exact_size(
                Vec3::fill(100.0),
                rounded_fill(
                    |_, _, c: &Context| Color::YELLOW,
                    |_, c| c.standard_style().rounding,
                ),
            ),
        ),
        */
        //  )),
    )
}
struct ScrollView<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    offset: Rc<RefCell<Vec2>>,
    horizontal: bool,
    vertical: bool,
    child_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context: GetStandardStyle, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for ScrollView<Data, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.child_size = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        let mut size = min_and_max_size.max;
        size.z = self.child_size.z;
        size
    }

    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let mut offset = self.offset.borrow_mut();

        let bounds_size = bounds.size();

        let min = (bounds_size.xy() - self.child_size.xy()).min(Vec2::ZERO);
        *offset = offset.clamp(min, Vec2::ZERO);

        drawer.push_clipping_mask(Box2::new(bounds.min.xy(), bounds.max.xy()));
        let mut bounds_max = bounds.max;
        if self.horizontal {
            bounds_max.x = f32::MAX
        }
        if self.vertical {
            bounds_max.y = f32::MAX
        }

        let child_bounds = Box3::new(bounds.min + offset.extend(0.0), bounds_max);
        self.child
            .draw(state, extra_state, context, drawer, child_bounds);
        drawer.pop_clipping_mask();

        let scroll_bar_size = 10.0;

        if self.vertical {
            let bar_start = bounds.min.y + bounds_size.y * (-offset.y / self.child_size.y);
            let bar_height_percent = bounds_size.y / self.child_size.y;

            // A min-height means that even if the scroll length is super long the bar can be seen.
            let bar_height = (bounds_size.y * bar_height_percent)
                .max(10.0)
                .min(bounds.max.y - bar_start);

            if bar_height_percent < 1.0 {
                drawer.rounded_rectangle(
                    Box3::new(
                        Vec3::new(bounds.max.x - scroll_bar_size, bar_start, bounds.min.z),
                        Vec3::new(bounds.max.x, bar_start + bar_height, bounds.min.z),
                    ),
                    Vec4::fill(context.standard_style().rounding),
                    context.standard_style().disabled_color,
                );
            }
        }

        if self.horizontal {
            // Todo
        }
    }
}
