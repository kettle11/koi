use crate::*;

pub fn button<
    State,
    Context: GetStandardInput,
    Constraints: GetStandardConstraints + Default + Copy + 'static,
    Drawer: GetStandardDrawer,
>(
    on_click: fn(&mut State),
    child_widget: impl Widget<State, Context, Constraints, Drawer>,
) -> impl Widget<State, Context, Constraints, Drawer> {
    ButtonBase {
        child_widget: padding(100., stack((fill(Color::RED), child_widget))),
        bounding_rect: Box2::ZERO,
        on_click,
        phantom: std::marker::PhantomData,
    }
}

pub fn button_base<
    State,
    Context: GetStandardInput,
    Constraints: GetStandardConstraints,
    Drawer,
>(
    on_click: fn(&mut State),
    child_widget: impl Widget<State, Context, Constraints, Drawer>,
) -> impl Widget<State, Context, Constraints, Drawer> {
    ButtonBase {
        child_widget,
        bounding_rect: Box2::ZERO,
        on_click,
        phantom: std::marker::PhantomData,
    }
}
pub struct ButtonBase<
    State,
    Context,
    Constraints: GetStandardConstraints,
    Drawer,
    Child: Widget<State, Context, Constraints, Drawer>,
> {
    child_widget: Child,
    bounding_rect: Box2,
    on_click: fn(&mut State),
    phantom: std::marker::PhantomData<fn() -> (Context, Constraints, Drawer)>,
}

impl<
        State,
        Context: GetStandardInput,
        Constraints: GetStandardConstraints,
        Drawer,
        Child: Widget<State, Context, Constraints, Drawer>,
    > Widget<State, Context, Constraints, Drawer>
    for ButtonBase<State, Context, Constraints, Drawer, Child>
{
    fn update(&mut self, state: &mut State, context: &mut Context) {
        // Todo: Check for input here and handle click event.
        self.child_widget.update(state, context);

        let standard_input = context.standard_input();
        let clicked = standard_input.pointer_down
            && self
                .bounding_rect
                .contains_point(standard_input.pointer_position);
        if clicked {
            (self.on_click)(state)
        }
    }
    fn layout(&mut self, state: &mut State, context: &mut Context) -> Constraints {
        let child_constraints = self.child_widget.layout(state, context);
        let bounds = child_constraints.standard().bounds;
        self.bounding_rect = Box2 {
            min: bounds.min.xy(),
            max: bounds.max.xy(),
        };
        child_constraints
    }
    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Constraints,
    ) {
        let size = self
            .bounding_rect
            .size()
            .min(constraints.standard().bounds.size().xy());

        self.bounding_rect =
            Box2::new_with_min_corner_and_size(constraints.standard().bounds.min.xy(), size);
        self.child_widget.draw(state, context, drawer, constraints);
    }
}
