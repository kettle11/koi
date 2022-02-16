use crate::*;

// In the future this should be generalized.
pub fn align_horizontal_end_vertical_start<Data, Context>(
    child: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    AlignUpperEnd {
        child,
        child_size: Vec3::ZERO,
        phantom: std::marker::PhantomData,
    }
}

struct AlignUpperEnd<Data, Context, Child: Widget<Data, Context>> {
    child: Child,
    child_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context)>,
}

impl<Data, Context, Child: Widget<Data, Context>> Widget<Data, Context>
    for AlignUpperEnd<Data, Context, Child>
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
        let corner = Vec3::new(constraints.max.x, constraints.min.y, constraints.min.z);
        constraints.min = Vec3::new(corner.x - self.child_size.x, corner.y, corner.z);
        constraints.max = constraints.min + self.child_size;
        self.child.draw(state, context, drawer, constraints)
    }
    fn update(&mut self, state: &mut Data, context: &mut Context) {
        self.child.update(state, context)
    }
}
