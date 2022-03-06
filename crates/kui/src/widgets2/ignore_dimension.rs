use crate::*;

/// Makes a child not contribute to the width of its parent element.
pub fn ignore_width<Data, Context>(
    child: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    IgnoreDimension {
        child,
        dimension_mask: Vec3::new(0.0, 1.0, 1.0),
        phantom: std::marker::PhantomData,
    }
}

pub struct IgnoreDimension<Data, Context, Child: Widget<Data, Context>> {
    child: Child,
    dimension_mask: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context)>,
}

impl<Data, Context, Child: Widget<Data, Context>> Widget<Data, Context>
    for IgnoreDimension<Data, Context, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let child_size = self.child.layout(state, context, min_and_max_size);
        child_size.mul_by_component(self.dimension_mask)
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
