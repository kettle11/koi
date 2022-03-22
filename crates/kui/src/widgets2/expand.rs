use crate::*;

pub fn expand<Data, Context, ExtraState>(
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Expand {
        child,
        axis: Vec3::XY,
        phantom: std::marker::PhantomData,
    }
}

pub fn expand_horizontal<Data, Context, ExtraState>(
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Expand {
        child,
        axis: Vec3::X,
        phantom: std::marker::PhantomData,
    }
}

pub fn expand_vertical<Data, Context, ExtraState>(
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Expand {
        child,
        axis: Vec3::Y,
        phantom: std::marker::PhantomData,
    }
}

pub struct Expand<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    axis: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for Expand<Data, Context, ExtraState, Child>
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
        min_and_max_size
            .max
            .mul_by_component(self.axis)
            .max(child_size)
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
