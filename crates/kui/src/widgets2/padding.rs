use crate::*;

pub fn padding<Data, Context>(
    amount: fn(&Context) -> f32,
    child: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    Padding {
        child,
        amount,
        phantom: std::marker::PhantomData,
    }
}

pub struct Padding<Data, Context, Child: Widget<Data, Context>> {
    child: Child,
    amount: fn(&Context) -> f32,
    phantom: std::marker::PhantomData<fn() -> (Data, Context)>,
}

impl<Data, Context, Child: Widget<Data, Context>> Widget<Data, Context>
    for Padding<Data, Context, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        mut min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let padding_amount = (self.amount)(context);
        let padding = Vec3::fill(padding_amount) * 2.0;
        min_and_max_size.max -= padding;
        let child_size = self.child.layout(state, context, min_and_max_size);
        child_size + padding
    }
    fn draw(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        drawer: &mut Drawer,
        mut constraints: Box3,
    ) {
        let amount = (self.amount)(context);
        constraints.min += Vec2::fill(amount).extend(0.1);
        constraints.max -= Vec2::fill(amount).extend(0.1);
        self.child.draw(state, context, drawer, constraints)
    }
    fn update(&mut self, state: &mut Data, context: &mut Context) {
        self.child.update(state, context)
    }
}
