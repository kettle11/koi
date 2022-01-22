use crate::*;

/// Provide child widgets only the space they request.
pub fn fit<Data, Context>(child: impl Widget<Data, Context>) -> impl Widget<Data, Context> {
    Fit {
        child,
        child_size: Vec3::ZERO,
        phantom: std::marker::PhantomData,
    }
}

pub struct Fit<Data, Context, Child: Widget<Data, Context>> {
    child: Child,
    child_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context)>,
}

impl<Data, Context, Child: Widget<Data, Context>> Widget<Data, Context>
    for Fit<Data, Context, Child>
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
        constraints.max = constraints.min + self.child_size;
        self.child.draw(state, context, drawer, constraints)
    }
    fn update(&mut self, state: &mut Data, context: &mut Context) {
        self.child.update(state, context)
    }
}
