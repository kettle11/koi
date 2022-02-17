use crate::*;

// In the future this should be generalized.
pub fn ignore_size<Data, Context>(child: impl Widget<Data, Context>) -> impl Widget<Data, Context> {
    IgnoreSize {
        child,
        phantom: std::marker::PhantomData,
    }
}

struct IgnoreSize<Data, Context, Child: Widget<Data, Context>> {
    child: Child,
    phantom: std::marker::PhantomData<fn() -> (Data, Context)>,
}

impl<Data, Context, Child: Widget<Data, Context>> Widget<Data, Context>
    for IgnoreSize<Data, Context, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let _ = self.child.layout(state, context, min_and_max_size);
        Vec3::ZERO
    }
    fn draw(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        self.child.draw(state, context, drawer, constraints)
    }
    fn update(&mut self, state: &mut Data, context: &mut Context) {
        self.child.update(state, context)
    }
}
