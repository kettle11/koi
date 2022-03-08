use crate::*;

/// Provide child widgets only the space they request.
pub fn fit<Data, Context, ExtraState>(
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Fit {
        child,
        child_size: Vec3::ZERO,
        phantom: std::marker::PhantomData,
    }
}

pub struct Fit<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    child_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for Fit<Data, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let child_size = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        self.child_size = child_size;
        child_size
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,

        context: &mut Context,
        drawer: &mut Drawer,
        mut constraints: Box3,
    ) {
        constraints.max = constraints.min + self.child_size;
        self.child
            .draw(state, extra_state, context, drawer, constraints)
    }
}
