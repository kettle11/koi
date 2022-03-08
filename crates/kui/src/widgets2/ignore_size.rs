use crate::*;

// In the future this should be generalized.
pub fn ignore_size<Data, Context, ExtraState>(
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    IgnoreSize {
        child,
        phantom: std::marker::PhantomData,
    }
}

struct IgnoreSize<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for IgnoreSize<Data, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let _ = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        Vec3::ZERO
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,

        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        self.child
            .draw(state, extra_state, context, drawer, constraints)
    }
}
