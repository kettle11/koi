use crate::*;

// In the future this should be generalized.
pub fn conditional<State, Context>(
    get_condition: impl Fn(&mut State, &Context) -> bool,
    child: impl Widget<State, Context>,
) -> impl Widget<State, Context> {
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
    GetConditional: Fn(&mut State, &Context) -> bool,
    Child: Widget<State, Context>,
> {
    child: Child,
    child_size: Vec3,
    get_condition: GetConditional,
    child_visible: bool,
    phantom: std::marker::PhantomData<fn() -> (State, Context)>,
}

impl<
        State,
        Context,
        GetConditional: Fn(&mut State, &Context) -> bool,
        Child: Widget<State, Context>,
    > Widget<State, Context> for Conditional<State, Context, GetConditional, Child>
{
    fn layout(
        &mut self,
        state: &mut State,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let child_size = if self.child_visible {
            self.child.layout(state, context, min_and_max_size)
        } else {
            Vec3::ZERO
        };
        self.child_size = child_size;
        child_size
    }
    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        if self.child_visible {
            self.child.draw(state, context, drawer, constraints);
        }
    }
    fn update(&mut self, state: &mut State, context: &mut Context) {
        self.child_visible = (self.get_condition)(state, context);
        if self.child_visible {
            self.child.update(state, context)
        }
    }
}
