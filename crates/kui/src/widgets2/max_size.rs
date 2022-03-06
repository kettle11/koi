use crate::*;

/// Limits the childs size
pub fn max_width<Data, Context>(
    width: f32,
    child: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    MaxSize {
        child,
        max_size: Vec3::new(width, f32::MAX, f32::MAX),
        phantom: std::marker::PhantomData,
    }
}

pub struct MaxSize<Data, Context, Child: Widget<Data, Context>> {
    child: Child,
    max_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context)>,
}

impl<Data, Context, Child: Widget<Data, Context>> Widget<Data, Context>
    for MaxSize<Data, Context, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        mut min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        min_and_max_size.max = min_and_max_size.max.min(self.max_size);
        let child_size = self.child.layout(state, context, min_and_max_size);
        child_size.min(self.max_size)
    }
    fn draw(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let box_size = constraints.size().min(self.max_size);
        let constraints = Box3 {
            min: constraints.min,
            max: constraints.min + box_size,
        };
        self.child.draw(state, context, drawer, constraints)
    }
    fn update(&mut self, state: &mut Data, context: &mut Context) {
        self.child.update(state, context)
    }
}
