use crate::*;

// In the future this should be generalized.
pub fn conditional<State, Context, ExtraState>(
    get_condition: impl Fn(&mut State, &Context) -> bool,
    child: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    Conditional {
        child,
        child_size: Vec3::ZERO,
        get_condition,
        child_visible: false,
        phantom: std::marker::PhantomData,
    }
}

struct Conditional<
    State,
    Context,
    ExtraState,
    GetConditional: Fn(&mut State, &Context) -> bool,
    Child: Widget<State, Context, ExtraState>,
> {
    child: Child,
    child_size: Vec3,
    get_condition: GetConditional,
    child_visible: bool,
    phantom: std::marker::PhantomData<fn() -> (State, Context, ExtraState)>,
}

impl<
        State,
        Context,
        ExtraState,
        GetConditional: Fn(&mut State, &Context) -> bool,
        Child: Widget<State, Context, ExtraState>,
    > Widget<State, Context, ExtraState>
    for Conditional<State, Context, ExtraState, GetConditional, Child>
{
    fn layout(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.child_visible = (self.get_condition)(state, context);
        let child_size = if self.child_visible {
            self.child
                .layout(state, extra_state, context, min_and_max_size)
        } else {
            self.child.layout(
                state,
                extra_state,
                context,
                MinAndMaxSize {
                    min: Vec3::ZERO,
                    max: Vec3::ZERO,
                },
            );
            Vec3::ZERO
        };
        self.child_size = child_size;
        child_size
    }
    fn draw(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,

        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        if self.child_visible {
            self.child
                .draw(state, extra_state, context, drawer, constraints);
        }
    }
}
