use crate::*;

pub fn stack<State, Context, ExtraState, I: IntoWidgetChildren<State, Context, ExtraState>>(
    children: I,
) -> Consecutive<State, Context, ExtraState, I::WidgetChildren, fn(&mut State, &Context) -> f32> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::Z,
        children: children.into_widget_children(),
        get_spacing: |_, _| 0.0,
        phantom: std::marker::PhantomData,
    }
}

pub fn column<
    State,
    Context: GetStandardStyle,
    ExtraState,
    I: IntoWidgetChildren<State, Context, ExtraState>,
>(
    children: I,
) -> Consecutive<State, Context, ExtraState, I::WidgetChildren, fn(&mut State, &Context) -> f32> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::Y,
        children: children.into_widget_children(),
        get_spacing: |_, c| c.standard_style().padding,
        phantom: std::marker::PhantomData,
    }
}

pub fn column_with_spacing<
    State,
    Context: GetStandardStyle,
    ExtraState,
    I: IntoWidgetChildren<State, Context, ExtraState>,
    GetSpacing: Fn(&mut State, &Context) -> f32,
>(
    get_spacing: GetSpacing,
    children: I,
) -> Consecutive<State, Context, ExtraState, I::WidgetChildren, GetSpacing> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::Y,
        children: children.into_widget_children(),
        get_spacing,
        phantom: std::marker::PhantomData,
    }
}

pub fn row<
    State,
    Context: GetStandardStyle,
    ExtraState,
    I: IntoWidgetChildren<State, Context, ExtraState>,
>(
    children: I,
) -> Consecutive<State, Context, ExtraState, I::WidgetChildren, fn(&mut State, &Context) -> f32> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::X,
        children: children.into_widget_children(),
        get_spacing: |_, c| c.standard_style().padding,
        phantom: std::marker::PhantomData,
    }
}

pub fn row_unspaced<
    State,
    Context,
    ExtraState,
    I: IntoWidgetChildren<State, Context, ExtraState>,
>(
    children: I,
) -> impl Widget<State, Context, ExtraState> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::X,
        children: children.into_widget_children(),
        get_spacing: |_, _| 0.0,
        phantom: std::marker::PhantomData,
    }
}

pub fn column_unspaced<
    State,
    Context,
    ExtraState,
    I: IntoWidgetChildren<State, Context, ExtraState>,
>(
    children: I,
) -> impl Widget<State, Context, ExtraState> {
    Consecutive {
        direction: Vec3::Y,
        children: children.into_widget_children(),
        get_spacing: |_, _| 0.0,
        phantom: std::marker::PhantomData,
    }
}

pub struct Consecutive<
    State,
    Context,
    ExtraState,
    Children: WidgetChildren<State, Context, ExtraState>,
    GetSpacing: Fn(&mut State, &Context) -> f32,
> {
    direction: Vec3,
    children: Children,
    get_spacing: GetSpacing,
    phantom: std::marker::PhantomData<(State, Context, ExtraState)>,
}

// This is written a bit more verbosely and less efficiently than necessary to accomodate rows, columns,
// and maybe eventually Z-axis stacks with the same code.
impl<
        State,
        Context,
        ExtraState,
        Children: WidgetChildren<State, Context, ExtraState>,
        GetSpacing: Fn(&mut State, &Context) -> f32,
    > Widget<State, Context, ExtraState>
    for Consecutive<State, Context, ExtraState, Children, GetSpacing>
{
    fn layout(
        &mut self,
        data: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let spacing = (self.get_spacing)(data, context);

        let mut offset_in_direction = 0.;
        let mut other_dimension_size = Vec3::ZERO;
        self.children
            .create_children_and_layout(data, extra_state, context, min_and_max_size);
        let mut sized_children: usize = 0;
        for &child_size in self.children.constraints_iter() {
            let amount_in_directon = child_size.dot(self.direction);
            if amount_in_directon > 0.0 {
                sized_children += 1;
            }
            let non_direction_size = child_size - (amount_in_directon * self.direction);
            other_dimension_size = other_dimension_size.max(non_direction_size);
            offset_in_direction += amount_in_directon;
        }
        offset_in_direction += spacing * sized_children.saturating_sub(1) as f32;
        other_dimension_size + self.direction * offset_in_direction
    }

    fn draw(
        &mut self,
        data: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let spacing = (self.get_spacing)(data, context);

        let constraint_size = bounds.size();
        let mut offset = bounds.min;

        let size_not_along_direction =
            constraint_size - (constraint_size.dot(self.direction) * self.direction);
        self.children
            .draw(data, extra_state, context, drawer, |constraints| {
                let amount_along_direction = constraints.dot(self.direction).abs();
                let child_size_along_direction = amount_along_direction * self.direction;

                let corner0 = offset;
                let corner1 = offset + child_size_along_direction + size_not_along_direction;
                let child_constraints = Box3::new(corner0.min(corner1), corner0.max(corner1));

                if amount_along_direction > 0.0 {
                    offset += child_size_along_direction + spacing * self.direction;
                }
                child_constraints
            })
    }
}
