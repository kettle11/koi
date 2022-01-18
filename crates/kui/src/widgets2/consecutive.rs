use std::marker::PhantomData;

use crate::*;

pub fn column<State, Constraints, Drawer, I: IntoWidgetChildren<State, Constraints, Drawer>>(
    children: I,
) -> Consecutive<State, Constraints, Drawer, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec2::Y,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub fn row<State, Constraints, Drawer, I: IntoWidgetChildren<State, Constraints, Drawer>>(
    children: I,
) -> Consecutive<State, Constraints, Drawer, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec2::X,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub struct Consecutive<
    State,
    Constraints,
    Drawer,
    Children: WidgetChildren<State, Constraints, Drawer>,
> {
    direction: Vec2,
    children: Children,
    phantom: std::marker::PhantomData<(State, Constraints, Drawer)>,
}

// This is written a bit more verbosely and less efficiently than necessary to accomodate rows, columns,
// and maybe eventually Z-axis stacks with the same code.
impl<
        State,
        Constraints: GetStandardConstraints + Default,
        Drawer,
        Children: WidgetChildren<State, Constraints, Drawer>,
    > Widget<State, Constraints, Drawer> for Consecutive<State, Constraints, Drawer, Children>
{
    fn update(&mut self, state: &mut State) {
        self.children.update(state)
    }

    fn layout(&mut self, state: &mut State) -> Constraints {
        let mut offset_in_direction = 0.;
        let mut other_dimension_size = Vec2::ZERO;
        self.children.create_children_and_layout(state);
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

    fn draw(&mut self, state: &mut State, drawer: &mut Drawer, constraints: Constraints) {
        let mut offset = constraints.standard().bounds.min;
        self.children.draw(state, drawer, |constraints| {
            let mut child_constraints = Constraints::default();
            child_constraints.standard_mut().bounds =
                Box2::new(offset, offset + constraints.standard().bounds.size());
            offset += constraints.standard().bounds.size().dot(self.direction) * self.direction;
            child_constraints
        })
    }
}

pub struct Thingy<State> {
    f: PhantomData<State>,
}

impl<State> Thingy<State> {
    pub fn new() -> Self {
        Self {
            f: std::marker::PhantomData,
        }
    }
}
impl<State, Constraints: GetStandardConstraints + Default, Drawer>
    Widget<State, Constraints, Drawer> for Thingy<State>
{
    fn layout(&mut self, state: &mut State) -> Constraints {
        todo!()
    }
}
