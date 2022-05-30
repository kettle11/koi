use std::cell::RefCell;

use crate::*;

pub fn vertical_scroll_view<
    State: 'static,
    Context: GetStandardInput + GetEventHandlers<State>,
    ExtraState,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    scroll_view(false, true, child_widget)
}

pub fn horizontal_scroll_view<
    State: 'static,
    Context: GetStandardInput + GetEventHandlers<State>,
    ExtraState,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    scroll_view(true, false, child_widget)
}
pub fn scroll_view<
    State: 'static,
    Context: GetStandardInput + GetEventHandlers<State>,
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
        ScrollView {
            child: child_widget,
            offset: offset0,
            horizontal,
            vertical,
            phantom: std::marker::PhantomData,
        },
    )
}
struct ScrollView<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    offset: Rc<RefCell<Vec2>>,
    horizontal: bool,
    vertical: bool,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for ScrollView<Data, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.child
            .layout(state, extra_state, context, min_and_max_size);
        min_and_max_size.max
    }

    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let offset: Vec2 = *self.offset.borrow_mut();
        drawer.push_clipping_mask(Box2::new(bounds.min.xy(), bounds.max.xy()));
        let mut bounds_max = bounds.max;
        if self.horizontal {
            bounds_max.x = f32::MAX
        }
        if self.vertical {
            bounds_max.y = f32::MAX
        }
        let bounds = Box3::new(bounds.min + offset.extend(0.0), bounds_max);
        self.child.draw(state, extra_state, context, drawer, bounds);
        drawer.pop_clipping_mask();
    }
}
