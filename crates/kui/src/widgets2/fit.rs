use crate::*;

/// Provide child widgets only the space they request.
pub fn fit<State, Context, Constraints: GetStandardConstraints, Drawer>(
    child: impl Widget<State, Context, Constraints, Drawer>,
) -> impl Widget<State, Context, Constraints, Drawer> {
    Fit {
        child,
        child_size: Vec3::ZERO,
        phantom: std::marker::PhantomData,
    }
}

pub struct Fit<
    State,
    Context,
    Constraints,
    Drawer,
    Child: Widget<State, Context, Constraints, Drawer>,
> {
    child: Child,
    child_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (State, Context, Constraints, Drawer)>,
}

impl<
        State,
        Context,
        Constraints: GetStandardConstraints,
        Drawer,
        Child: Widget<State, Context, Constraints, Drawer>,
    > Widget<State, Context, Constraints, Drawer>
    for Fit<State, Context, Constraints, Drawer, Child>
{
    fn layout(&mut self, state: &mut State, context: &mut Context) -> Constraints {
        let child_constraints = self.child.layout(state, context);
        self.child_size = child_constraints.standard().bounds.size();
        child_constraints
    }
    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        mut constraints: Constraints,
    ) {
        let min = constraints.standard().bounds.min;
        constraints.standard_mut().bounds = Box3 {
            min: min,
            max: min + self.child_size,
        };
        self.child.draw(state, context, drawer, constraints)
    }
    fn update(&mut self, state: &mut State, context: &mut Context) {
        self.child.update(state, context)
    }
}
