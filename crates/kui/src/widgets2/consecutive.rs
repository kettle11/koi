use crate::*;

pub fn stack<State, Context, I: IntoWidgetChildren<State, Context>>(
    children: I,
) -> Consecutive<State, Context, I::WidgetChildren, fn(&mut State, &Context) -> f32> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::Z,
        children: children.into_widget_children(),
        get_spacing: |_, _| 0.0,
        phantom: std::marker::PhantomData,
    }
}

pub fn column<State, Context, I: IntoWidgetChildren<State, Context>>(
    children: I,
) -> Consecutive<State, Context, I::WidgetChildren, fn(&mut State, &Context) -> f32> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::Y,
        children: children.into_widget_children(),
        get_spacing: |_, _| 0.0,
        phantom: std::marker::PhantomData,
    }
}

pub fn row<State, Context, I: IntoWidgetChildren<State, Context>>(
    children: I,
) -> Consecutive<State, Context, I::WidgetChildren, fn(&mut State, &Context) -> f32> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::X,
        children: children.into_widget_children(),
        get_spacing: |_, _| 0.0,
        phantom: std::marker::PhantomData,
    }
}

pub fn row_spaced<State, Context, I: IntoWidgetChildren<State, Context>>(
    get_spacing: impl Fn(&mut State, &Context) -> f32,
    children: I,
) -> impl Widget<State, Context> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::X,
        children: children.into_widget_children(),
        get_spacing,
        phantom: std::marker::PhantomData,
    }
}

pub fn column_spaced<State, Context, I: IntoWidgetChildren<State, Context>>(
    get_spacing: impl Fn(&mut State, &Context) -> f32,
    children: I,
) -> impl Widget<State, Context> {
    Consecutive {
        direction: Vec3::Y,
        children: children.into_widget_children(),
        get_spacing,
        phantom: std::marker::PhantomData,
    }
}

pub struct Consecutive<
    State,
    Context,
    Children: WidgetChildren<State, Context>,
    GetSpacing: Fn(&mut State, &Context) -> f32,
> {
    direction: Vec3,
    children: Children,
    get_spacing: GetSpacing,
    phantom: std::marker::PhantomData<(State, Context)>,
}

// This is written a bit more verbosely and less efficiently than necessary to accomodate rows, columns,
// and maybe eventually Z-axis stacks with the same code.
impl<
        State,
        Context,
        Children: WidgetChildren<State, Context>,
        GetSpacing: Fn(&mut State, &Context) -> f32,
    > Widget<State, Context> for Consecutive<State, Context, Children, GetSpacing>
{
    fn update(&mut self, data: &mut State, context: &mut Context) {
        self.children.update(data, context)
    }

    fn layout(
        &mut self,
        data: &mut State,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let spacing = (self.get_spacing)(data, context);

        let mut offset_in_direction = 0.;
        let mut other_dimension_size = Vec3::ZERO;
        self.children
            .create_children_and_layout(data, context, min_and_max_size);
        for &child_size in self.children.constraints_iter() {
            let amount_in_directon = child_size.dot(self.direction);
            let non_direction_size = child_size - (amount_in_directon * self.direction);
            other_dimension_size = other_dimension_size.max(non_direction_size);
            offset_in_direction += amount_in_directon;
        }
        offset_in_direction += spacing * self.children.len().saturating_sub(1) as f32;
        other_dimension_size + self.direction * offset_in_direction
    }

    fn draw(&mut self, data: &mut State, context: &mut Context, drawer: &mut Drawer, bounds: Box3) {
        let spacing = (self.get_spacing)(data, context);

        let constraint_size = bounds.size();
        let mut offset = bounds.min;

        // Simple logic for now, but could be more general.
        if self.direction.x < 0.0 {
            offset.x = bounds.max.x;
        }
        let size_not_along_direction =
            constraint_size - (constraint_size.dot(self.direction) * self.direction);
        self.children.draw(data, context, drawer, |constraints| {
            let child_size_along_direction = constraints.dot(self.direction).abs() * self.direction;

            let corner0 = offset;
            let corner1 = offset + child_size_along_direction + size_not_along_direction;
            let child_constraints = Box3::new(corner0.min(corner1), corner0.max(corner1));

            offset += child_size_along_direction + spacing * self.direction;

            child_constraints
        })
    }
}
