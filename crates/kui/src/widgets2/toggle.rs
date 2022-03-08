use crate::*;

/// Selects child_a if the condition is true, otherwise child_b is selected.
pub fn toggle<Data, Context, ExtraState>(
    select: fn(&Data, &ExtraState) -> bool,
    child_a: impl Widget<Data, Context, ExtraState>,
    child_b: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Toggle {
        select,
        child_a,
        child_b,
        phantom: std::marker::PhantomData,
    }
}

pub struct Toggle<
    Data,
    Context,
    ExtraState,
    ChildA: Widget<Data, Context, ExtraState>,
    ChildB: Widget<Data, Context, ExtraState>,
> {
    select: fn(&Data, &ExtraState) -> bool,
    child_a: ChildA,
    child_b: ChildB,
    phantom: std::marker::PhantomData<Context>,
}

impl<
        Data,
        Context,
        ExtraState,
        ChildA: Widget<Data, Context, ExtraState>,
        ChildB: Widget<Data, Context, ExtraState>,
    > Widget<Data, Context, ExtraState> for Toggle<Data, Context, ExtraState, ChildA, ChildB>
{
    /*
    fn update(&mut self, data: &mut Data, context: &mut Context) {
        let select = (self.select)(data);
        if select {
            self.child_a.update(data, context)
        } else {
            self.child_b.update(data, context)
        }
    }
    */

    fn layout(
        &mut self,
        data: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let select = (self.select)(data, extra_state);
        if select {
            self.child_a
                .layout(data, extra_state, context, min_and_max_size)
        } else {
            self.child_b
                .layout(data, extra_state, context, min_and_max_size)
        }
    }
    fn draw(
        &mut self,
        data: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let select = (self.select)(data, extra_state);
        if select {
            self.child_a
                .draw(data, extra_state, context, drawer, bounds)
        } else {
            self.child_b
                .draw(data, extra_state, context, drawer, bounds)
        }
    }
}
