use crate::*;

pub fn button<State, Context: GetStandardInput + GetStandardStyle>(
    on_click: fn(&mut State),
    child_widget: impl Widget<State, Context>,
) -> impl Widget<State, Context> {
    ButtonBase {
        child_widget: fit(stack((
            outlined_rounded_fill(
                |c: &Context| c.standard_style().primary_color,
                |c: &Context| c.standard_style().primary_variant_color,
                |c| c.standard_style().rounding,
            ),
            padding(|c| c.standard_style().padding, child_widget),
        ))),
        bounding_rect: Box2::ZERO,
        on_click,
        phantom: std::marker::PhantomData,
    }
}

pub fn button_base<State, Context: GetStandardInput>(
    on_click: fn(&mut State),
    child_widget: impl Widget<State, Context>,
) -> impl Widget<State, Context> {
    ButtonBase {
        child_widget,
        bounding_rect: Box2::ZERO,
        on_click,
        phantom: std::marker::PhantomData,
    }
}
pub struct ButtonBase<State, Context, Child: Widget<State, Context>> {
    child_widget: Child,
    bounding_rect: Box2,
    on_click: fn(&mut State),
    phantom: std::marker::PhantomData<fn() -> Context>,
}

impl<State, Context: GetStandardInput, Child: Widget<State, Context>> Widget<State, Context>
    for ButtonBase<State, Context, Child>
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
    fn layout(&mut self, state: &mut State, context: &mut Context) -> Vec3 {
        let child_size = self.child_widget.layout(state, context);
        self.bounding_rect = Box2 {
            min: Vec2::ZERO,
            max: child_size.xy(),
        };
        child_size
    }
    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let size = self.bounding_rect.size().min(constraints.size().xy());
        self.bounding_rect = Box2::new_with_min_corner_and_size(constraints.min.xy(), size);
        self.child_widget.draw(state, context, drawer, constraints);
    }
}
