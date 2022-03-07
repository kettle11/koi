use crate::*;

pub fn center<Data, Context>(child: impl Widget<Data, Context>) -> impl Widget<Data, Context> {
    Center {
        child,
        child_size: Vec3::ZERO,
        phantom: std::marker::PhantomData,
    }
}

pub struct Center<Data, Context, Child: Widget<Data, Context>> {
    child: Child,
    child_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context)>,
}

impl<Data, Context, Child: Widget<Data, Context>> Widget<Data, Context>
    for Center<Data, Context, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let child_size = self.child.layout(state, context, min_and_max_size);
        self.child_size = child_size;
        child_size
    }
    fn draw(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        drawer: &mut Drawer,
        mut constraints: Box3,
    ) {
        if self.child_size.y == 5.3 {
            println!("CHILD SIZE: {:?}", self.child_size);
            println!("CONSTRAINTS: {:?}", constraints);
        }
        constraints.min = constraints.center() - self.child_size / 2.0;
        constraints.max = constraints.center() + self.child_size / 2.0;

        self.child.draw(state, context, drawer, constraints)
    }
    fn update(&mut self, state: &mut Data, context: &mut Context) {
        self.child.update(state, context)
    }
}

/// Set a child element to a fixed position in relation to the view.
/// This probably should only be used for debug purposes.
pub fn fixed_position_and_size<Data, Context>(
    bounds: Box3,
    child: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    FixedPositionAndSize {
        child,
        bounds,
        phantom: std::marker::PhantomData,
    }
}

pub struct FixedPositionAndSize<Data, Context, Child: Widget<Data, Context>> {
    child: Child,
    bounds: Box3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context)>,
}

impl<Data, Context, Child: Widget<Data, Context>> Widget<Data, Context>
    for FixedPositionAndSize<Data, Context, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        _min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.child.layout(
            state,
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
        context: &mut Context,
        drawer: &mut Drawer,
        _constraints: Box3,
    ) {
        self.child.draw(state, context, drawer, self.bounds)
    }
    fn update(&mut self, state: &mut Data, context: &mut Context) {
        self.child.update(state, context)
    }
}
