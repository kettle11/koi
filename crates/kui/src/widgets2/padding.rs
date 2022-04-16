use crate::*;

pub fn padding<
    Data,
    Context: GetStandardStyle,
    ExtraState,
    Child: Widget<Data, Context, ExtraState>,
>(
    child: Child,
) -> Padding<Data, Context, ExtraState, Child> {
    Padding {
        child,
        amount: |c| c.standard_style().padding,
        phantom: std::marker::PhantomData,
    }
}

pub fn padding_with_amount<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>(
    amount: fn(&Context) -> f32,
    child: Child,
) -> Padding<Data, Context, ExtraState, Child> {
    Padding {
        child,
        amount,
        phantom: std::marker::PhantomData,
    }
}

pub struct Padding<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    amount: fn(&Context) -> f32,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Padding<Data, Context, ExtraState, Child>
{
    pub fn with_amount(mut self, amount: fn(&Context) -> f32) -> Self {
        self.amount = amount;
        self
    }
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for Padding<Data, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        mut min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let padding_amount = (self.amount)(context);
        let padding = Vec3::fill(padding_amount) * 2.0;
        min_and_max_size.max -= padding;
        let child_size = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        child_size + padding
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,

        context: &mut Context,
        drawer: &mut Drawer,
        mut constraints: Box3,
    ) {
        let amount = (self.amount)(context);
        constraints.min += Vec2::fill(amount).extend(0.1);
        constraints.max -= Vec2::fill(amount).extend(0.1);
        self.child
            .draw(state, extra_state, context, drawer, constraints)
    }
}
