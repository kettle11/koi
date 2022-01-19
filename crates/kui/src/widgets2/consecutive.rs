use crate::*;

pub fn stack<
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
        direction: Vec3::Z,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

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
        direction: Vec3::Y,
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
        direction: Vec3::X,
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
    direction: Vec3,
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
        let mut other_dimension_size = Vec3::ZERO;
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
        let constraint_bounds = constraints.standard().bounds;
        let constraint_size = constraint_bounds.size();
        let mut offset = constraint_bounds.min;
        let size_not_along_direction =
            constraint_size - (constraint_size.dot(self.direction) * self.direction);
        self.children.draw(state, context, drawer, |constraints| {
            let child_size_along_direction = constraint_size.dot(self.direction) * self.direction;

            let mut child_constraints = Constraints::default();
            child_constraints.standard_mut().bounds = Box3::new(
                offset,
                offset + child_size_along_direction + size_not_along_direction,
            );
            offset += constraints.standard().bounds.size().dot(self.direction) * self.direction;
            child_constraints
        })
    }
}
