use crate::*;

/// Makes a child not contribute to the width of its parent element.
pub fn ignore_width<Data, Context, ExtraState>(
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    IgnoreDimension {
        child,
        dimension_mask: Vec3::new(0.0, 1.0, 1.0),
        phantom: std::marker::PhantomData,
    }
}

pub struct IgnoreDimension<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    dimension_mask: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for IgnoreDimension<Data, Context, ExtraState, Child>
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
        child_size.mul_by_component(self.dimension_mask)
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
