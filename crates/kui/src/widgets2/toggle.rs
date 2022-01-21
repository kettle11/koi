use crate::*;

/// Selects child_a if the condition is true, otherwise child_b is selected.
pub fn toggle<Data, Context>(
    select: fn(&Data) -> bool,
    child_a: impl Widget<Data, Context>,
    child_b: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    Toggle {
        select,
        child_a,
        child_b,
        phantom: std::marker::PhantomData,
    }
}

pub struct Toggle<Data, Context, ChildA: Widget<Data, Context>, ChildB: Widget<Data, Context>> {
    select: fn(&Data) -> bool,
    child_a: ChildA,
    child_b: ChildB,
    phantom: std::marker::PhantomData<Context>,
}

impl<Data, Context, ChildA: Widget<Data, Context>, ChildB: Widget<Data, Context>>
    Widget<Data, Context> for Toggle<Data, Context, ChildA, ChildB>
{
    fn update(&mut self, data: &mut Data, context: &mut Context) {
        let select = (self.select)(data);
        if select {
            self.child_a.update(data, context)
        } else {
            self.child_b.update(data, context)
        }
    }

    fn layout(&mut self, data: &mut Data, context: &mut Context) -> Vec3 {
        let select = (self.select)(data);
        if select {
            self.child_a.layout(data, context)
        } else {
            self.child_b.layout(data, context)
        }
    }
    fn draw(&mut self, data: &mut Data, context: &mut Context, drawer: &mut Drawer, bounds: Box3) {
        let select = (self.select)(data);
        if select {
            self.child_a.draw(data, context, drawer, bounds)
        } else {
            self.child_b.draw(data, context, drawer, bounds)
        }
    }
}
