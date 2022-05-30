use crate::*;

pub fn stack<State, Context, ExtraState, I: IntoWidgetChildren<State, Context, ExtraState>>(
    children: I,
) -> Consecutive<State, Context, ExtraState, I::WidgetChildren, fn(&mut State, &Context) -> f32> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec3::Z,
        children: children.into_widget_children(),
        get_spacing: |_, _| 0.0,
        wrap: true,
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
        wrap: true,
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
        wrap: true,
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
        wrap: true,
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
        wrap: true,
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
        wrap: true,
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
    wrap: bool,
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

        self.children
            .create_children_and_layout(data, extra_state, context, min_and_max_size);

        let mut child_position_calculator = ChildPositionCalculator::new(
            self.direction,
            spacing,
            Box3::new(min_and_max_size.min, min_and_max_size.max),
            self.wrap,
        );

        //let mut offset_in_wrap_direction = 0.0;

        for &child_size in self.children.constraints_iter() {
            let _child_bounds = child_position_calculator.get_child_bounds(child_size);
        }
        child_position_calculator.total_size()
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

        let mut child_position_calculator =
            ChildPositionCalculator::new(self.direction, spacing, bounds, self.wrap);

        self.children
            .draw(data, extra_state, context, drawer, |constraints| {
                child_position_calculator.get_child_bounds(*constraints)
            })
    }
}

struct ChildPositionCalculator {
    direction: Vec3,
    wrap_direction: Vec3,
    offset_in_direction: f32,
    offset_in_wrap_direction: f32,
    max_along_direction: f32,
    current_line_height: f32,
    spacing: f32,
    size_available_in_other_directions: Vec3,
    min: Vec3,
    total_bounds: Box3,
    wrap: bool,
}

impl ChildPositionCalculator {
    pub fn new(direction: Vec3, spacing: f32, min_and_max: Box3, wrap: bool) -> Self {
        let wrap_direction = if direction == Vec3::Y {
            Vec3::X
        } else {
            Vec3::Y
        };
        let max_along_direction = min_and_max.max.dot(direction);

        let available_size = min_and_max.size();
        let size_available_in_other_directions =
            available_size - available_size.dot(direction) * direction;

        Self {
            direction,
            wrap_direction,
            offset_in_direction: 0.0,
            offset_in_wrap_direction: 0.0,
            max_along_direction,
            current_line_height: 0.0,
            spacing,
            size_available_in_other_directions,
            min: min_and_max.min,
            total_bounds: Box3::new(min_and_max.min, min_and_max.min),
            wrap,
        }
    }

    pub fn get_child_bounds(&mut self, child_size: Vec3) -> Box3 {
        let amount_in_directon = child_size.dot(self.direction);

        if self.wrap && self.offset_in_direction + amount_in_directon > self.max_along_direction {
            self.offset_in_direction = 0.0;
            self.offset_in_wrap_direction += self.current_line_height + self.spacing;
            self.current_line_height = 0.0;
        }
        let min = self.offset_in_direction * self.direction
            + self.offset_in_wrap_direction * self.wrap_direction
            + self.min;
        self.offset_in_direction += amount_in_directon;
        self.current_line_height = self
            .current_line_height
            .max(child_size.dot(self.wrap_direction));

        // Widgets that have 0 size do not introduce additional spacing.
        if amount_in_directon > 0.0 {
            self.offset_in_direction += self.spacing;
        }

        let child_bounds = Box3::new(
            min,
            min + child_size.max(self.size_available_in_other_directions),
        );

        // The size reported for layout is different than the size used for drawing.
        self.total_bounds = self.total_bounds.join(Box3::new(min, min + child_size));
        child_bounds
    }

    pub fn total_size(&self) -> Vec3 {
        self.total_bounds.size()
    }
}
