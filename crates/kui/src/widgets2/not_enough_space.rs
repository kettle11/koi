use crate::*;

// A component that switches to a different component when there's not size to display it.
pub fn not_enough_space<State, Context>(
    child: impl Widget<State, Context>,
    fallback_child: impl Widget<State, Context>,
) -> impl Widget<State, Context> {
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
    Child: Widget<State, Context>,
    FallbackChild: Widget<State, Context>,
> {
    child: Child,
    fallback_child: FallbackChild,
    child_size: Vec3,
    render_first_child: bool,
    phantom: std::marker::PhantomData<fn() -> (State, Context)>,
}

impl<State, Context, Child: Widget<State, Context>, FallbackChild: Widget<State, Context>>
    Widget<State, Context> for NotEnoughSpace<State, Context, Child, FallbackChild>
{
    fn layout(
        &mut self,
        state: &mut State,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        // The child can request whatever size it likes.
        self.child_size = self.child.layout(state, context, min_and_max_size);

        self.render_first_child = !self
            .child_size
            .greater_than_per_component(min_and_max_size.max)
            .any();

        if !self.render_first_child {
            self.child_size = self.fallback_child.layout(state, context, min_and_max_size);
        }
        self.child_size
    }
    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        if self.render_first_child {
            self.child.draw(state, context, drawer, constraints)
        } else {
            self.fallback_child
                .draw(state, context, drawer, constraints)
        }
    }
}
