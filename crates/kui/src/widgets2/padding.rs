use crate::*;

pub fn padding<State, Context: GetStandardInput, Constraints: GetStandardConstraints, Drawer>(
    amount: f32,
    child: impl Widget<State, Context, Constraints, Drawer>,
) -> impl Widget<State, Context, Constraints, Drawer> {
    Padding {
        child,
        amount,
        phantom: std::marker::PhantomData,
    }
}

pub struct Padding<
    State,
    Context,
    Constraints: GetStandardConstraints,
    Drawer,
    Child: Widget<State, Context, Constraints, Drawer>,
> {
    child: Child,
    amount: f32,
    phantom: std::marker::PhantomData<fn() -> (State, Context, Constraints, Drawer)>,
}

impl<
        State,
        Context,
        Constraints: GetStandardConstraints,
        Drawer,
        Child: Widget<State, Context, Constraints, Drawer>,
    > Widget<State, Context, Constraints, Drawer>
    for Padding<State, Context, Constraints, Drawer, Child>
{
    fn layout(&mut self, state: &mut State, context: &mut Context) -> Constraints {
        let mut child_layout = self.child.layout(state, context);
        let child_bounds = child_layout.standard().bounds;
        child_layout.standard_mut().bounds = child_bounds.inflated(self.amount);
        child_layout
    }
    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        mut constraints: Constraints,
    ) {
        constraints.standard_mut().bounds.min += Vec2::fill(self.amount).extend(0.1);
        constraints.standard_mut().bounds.max -= Vec2::fill(self.amount).extend(0.1);
        self.child.draw(state, context, drawer, constraints)
    }
    fn update(&mut self, state: &mut State, context: &mut Context) {
        self.child.update(state, context)
    }
}
