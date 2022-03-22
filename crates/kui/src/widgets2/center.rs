use crate::*;

pub fn center<Data, Context, ExtraState>(
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Center {
        child,
        child_size: Vec3::ZERO,
        phantom: std::marker::PhantomData,
    }
}

pub struct Center<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    child_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for Center<Data, Context, ExtraState, Child>
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
        constraints: Box3,
    ) {
        let child_constraints = Box3 {
            min: (constraints.center() - self.child_size / 2.0).max(constraints.min),
            max: (constraints.center() + self.child_size / 2.0).min(constraints.max),
        };

        self.child
            .draw(state, extra_state, context, drawer, child_constraints)
    }
}

/// Set a child element to a fixed position in relation to the view.
/// This probably should only be used for debug purposes.
pub fn fixed_position_and_size<Data, Context, ExtraState>(
    bounds: Box3,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    FixedPositionAndSize {
        child,
        bounds,
        phantom: std::marker::PhantomData,
    }
}

pub struct FixedPositionAndSize<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
{
    child: Child,
    bounds: Box3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for FixedPositionAndSize<Data, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        _min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.child.layout(
            state,
            extra_state,
            context,
            MinAndMaxSize {
                min: Vec3::ZERO,
                max: Vec3::MAX,
            },
        );
        Vec3::ZERO
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        _constraints: Box3,
    ) {
        self.child
            .draw(state, extra_state, context, drawer, self.bounds)
    }
}
