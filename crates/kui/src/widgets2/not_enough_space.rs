use crate::*;

// A component that switches to a different component when there's not size to display it.
pub fn not_enough_space<State, Context, ExtraState>(
    child: impl Widget<State, Context, ExtraState>,
    fallback_child: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    NotEnoughSpace {
        child,
        fallback_child,
        child_size: Vec3::ZERO,
        render_first_child: true,
        phantom: std::marker::PhantomData,
    }
}

struct NotEnoughSpace<
    State,
    Context,
    ExtraState,
    Child: Widget<State, Context, ExtraState>,
    FallbackChild: Widget<State, Context, ExtraState>,
> {
    child: Child,
    fallback_child: FallbackChild,
    child_size: Vec3,
    render_first_child: bool,
    phantom: std::marker::PhantomData<fn() -> (State, Context, ExtraState)>,
}

impl<
        State,
        Context,
        ExtraState,
        Child: Widget<State, Context, ExtraState>,
        FallbackChild: Widget<State, Context, ExtraState>,
    > Widget<State, Context, ExtraState>
    for NotEnoughSpace<State, Context, ExtraState, Child, FallbackChild>
{
    fn layout(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        // The child can request whatever size it likes.
        self.child_size = self
            .child
            .layout(state, extra_state, context, min_and_max_size);

        self.render_first_child = !self
            .child_size
            .greater_than_per_component(min_and_max_size.max)
            .any();

        if !self.render_first_child {
            self.child_size =
                self.fallback_child
                    .layout(state, extra_state, context, min_and_max_size);
        }
        self.child_size
    }
    fn draw(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        if self.render_first_child {
            self.child
                .draw(state, extra_state, context, drawer, constraints)
        } else {
            self.fallback_child
                .draw(state, extra_state, context, drawer, constraints)
        }
    }
}
