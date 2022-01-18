use crate::*;

pub fn column<
    State,
    Context,
    Constraints,
    Drawer,
    I: IntoWidgetChildren<State, Context, Constraints, Drawer>,
>(
    children: I,
) -> Consecutive<State, Context, Constraints, Drawer, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec2::Y,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub fn row<
    State,
    Context,
    Constraints,
    Drawer,
    I: IntoWidgetChildren<State, Context, Constraints, Drawer>,
>(
    children: I,
) -> Consecutive<State, Context, Constraints, Drawer, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec2::X,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub struct Consecutive<
    State,
    Context,
    Constraints,
    Drawer,
    Children: WidgetChildren<State, Context, Constraints, Drawer>,
> {
    direction: Vec2,
    children: Children,
    phantom: std::marker::PhantomData<(State, Context, Constraints, Drawer)>,
}

// This is written a bit more verbosely and less efficiently than necessary to accomodate rows, columns,
// and maybe eventually Z-axis stacks with the same code.
impl<
        State,
        Context,
        Constraints: GetStandardConstraints + Default,
        Drawer,
        Children: WidgetChildren<State, Context, Constraints, Drawer>,
    > Widget<State, Context, Constraints, Drawer>
    for Consecutive<State, Context, Constraints, Drawer, Children>
{
    fn update(&mut self, state: &mut State, context: &mut Context) {
        self.children.update(state, context)
    }

    fn layout(&mut self, state: &mut State, context: &mut Context) -> Constraints {
        let mut offset_in_direction = 0.;
        let mut other_dimension_size = Vec2::ZERO;
        self.children.create_children_and_layout(state, context);
        for child_constraints in self.children.constraints_iter() {
            let child_size = child_constraints.standard().bounds.size();
            let amount_in_directon = child_size.dot(self.direction);
            let non_direction_size = child_size - (amount_in_directon * self.direction);
            other_dimension_size = other_dimension_size.max(non_direction_size);
            offset_in_direction += amount_in_directon;
        }
        let mut constraints = Constraints::default();
        constraints
            .standard_mut()
            .set_size(other_dimension_size + self.direction * offset_in_direction);
        constraints
    }

    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Constraints,
    ) {
        let mut offset = constraints.standard().bounds.min;
        self.children.draw(state, context, drawer, |constraints| {
            let mut child_constraints = Constraints::default();
            child_constraints.standard_mut().bounds =
                Box2::new(offset, offset + constraints.standard().bounds.size());
            offset += constraints.standard().bounds.size().dot(self.direction) * self.direction;
            child_constraints
        })
    }
}
