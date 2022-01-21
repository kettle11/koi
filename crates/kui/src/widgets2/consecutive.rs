use crate::*;

pub fn stack<State, Context, I: IntoWidgetChildren<State, Context>>(
    children: I,
) -> Consecutive<State, Context, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::Z,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub fn column<State, Context, I: IntoWidgetChildren<State, Context>>(
    children: I,
) -> Consecutive<State, Context, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::Y,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub fn row<State, Context, I: IntoWidgetChildren<State, Context>>(
    children: I,
) -> Consecutive<State, Context, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::X,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub struct Consecutive<State, Context, Children: WidgetChildren<State, Context>> {
    direction: Vec3,
    children: Children,
    phantom: std::marker::PhantomData<(State, Context)>,
}

// This is written a bit more verbosely and less efficiently than necessary to accomodate rows, columns,
// and maybe eventually Z-axis stacks with the same code.
impl<State, Context, Children: WidgetChildren<State, Context>> Widget<State, Context>
    for Consecutive<State, Context, Children>
{
    fn update(&mut self, data: &mut State, context: &mut Context) {
        self.children.update(data, context)
    }

    fn layout(&mut self, data: &mut State, context: &mut Context) -> Vec3 {
        let mut offset_in_direction = 0.;
        let mut other_dimension_size = Vec3::ZERO;
        self.children.create_children_and_layout(data, context);
        for &child_size in self.children.constraints_iter() {
            let amount_in_directon = child_size.dot(self.direction);
            let non_direction_size = child_size - (amount_in_directon * self.direction);
            other_dimension_size = other_dimension_size.max(non_direction_size);
            offset_in_direction += amount_in_directon;
        }
        other_dimension_size + self.direction * offset_in_direction
    }

    fn draw(&mut self, data: &mut State, context: &mut Context, drawer: &mut Drawer, bounds: Box3) {
        let constraint_size = bounds.size();
        let mut offset = bounds.min;
        let size_not_along_direction =
            constraint_size - (constraint_size.dot(self.direction) * self.direction);
        self.children.draw(data, context, drawer, |constraints| {
            let child_size_along_direction = constraint_size.dot(self.direction) * self.direction;

            let child_constraints = Box3::new(
                offset,
                offset + child_size_along_direction + size_not_along_direction,
            );
            offset += constraints.dot(self.direction) * self.direction;
            child_constraints
        })
    }
}
