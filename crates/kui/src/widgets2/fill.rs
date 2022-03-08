use crate::*;

pub fn fill<State, Context: GetStandardInput + GetEventHandlers<State>, ExtraState>(
    color: impl Fn(&mut State, &mut ExtraState, &Context) -> Color,
) -> impl Widget<State, Context, ExtraState> {
    Fill {
        color,
        rounding: |_, _| 0.0,
        bounding_rect: Box2::ZERO,
        consume_pointer_events: true,
        phantom: std::marker::PhantomData,
    }
}

pub fn fill_pass_through<State, Context: GetStandardInput + GetEventHandlers<State>, ExtraState>(
    color: impl Fn(&mut State, &mut ExtraState, &Context) -> Color,
) -> impl Widget<State, Context, ExtraState> {
    Fill {
        color,
        rounding: |_, _| 0.0,
        bounding_rect: Box2::ZERO,
        consume_pointer_events: false,
        phantom: std::marker::PhantomData,
    }
}

pub fn rounded_fill<State, Context: GetStandardInput + GetEventHandlers<State>, ExtraState>(
    color: impl Fn(&mut State, &mut ExtraState, &Context) -> Color,
    rounding: impl Fn(&mut State, &Context) -> f32,
) -> impl Widget<State, Context, ExtraState> {
    Fill {
        color,
        rounding,
        bounding_rect: Box2::ZERO,
        consume_pointer_events: true,
        phantom: std::marker::PhantomData,
    }
}

pub struct Fill<
    State,
    Context,
    ExtraState,
    GetColor: Fn(&mut State, &mut ExtraState, &Context) -> Color,
    GetRounding: Fn(&mut State, &Context) -> f32,
> {
    pub color: GetColor,
    pub rounding: GetRounding,
    pub bounding_rect: Box2,
    pub consume_pointer_events: bool,
    phantom: std::marker::PhantomData<fn() -> (State, Context, ExtraState)>,
}

impl<
        State,
        Context: GetStandardInput,
        ExtraState,
        GetColor: Fn(&mut State, &mut ExtraState, &Context) -> Color,
        GetRounding: Fn(&mut State, &Context) -> f32,
    > Fill<State, Context, ExtraState, GetColor, GetRounding>
{
    pub fn pass_through_events(mut self) -> Self {
        self.consume_pointer_events = false;
        self
    }
}

impl<
        State,
        Context: GetStandardInput + GetEventHandlers<State>,
        ExtraState,
        GetColor: Fn(&mut State, &mut ExtraState, &Context) -> Color,
        GetRounding: Fn(&mut State, &Context) -> f32,
    > Widget<State, Context, ExtraState>
    for Fill<State, Context, ExtraState, GetColor, GetRounding>
{
    fn layout(
        &mut self,
        _data: &mut State,
        _extra_state: &mut ExtraState,
        _context: &mut Context,
        _min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        Vec3::ZERO
    }
    fn draw(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        if self.consume_pointer_events {
            context
                .event_handlers_mut()
                .add_pointer_event_handler(bounds, None);
        }
        drawer.standard().rounded_rectangle(
            bounds,
            Vec4::fill((self.rounding)(state, context)),
            (self.color)(state, extra_state, context),
        );
        self.bounding_rect = Box2 {
            min: bounds.min.xy(),
            max: bounds.max.xy(),
        };
    }
}
