use crate::*;

pub fn fill<State, Context: GetStandardInput>(
    color: impl Fn(&mut State, &Context) -> Color,
) -> impl Widget<State, Context> {
    Fill {
        color,
        rounding: |_, _| 0.0,
        bounding_rect: Box2::ZERO,
        consume_pointer_events: true,
        phantom: std::marker::PhantomData,
    }
}

pub fn fill_pass_through<State, Context: GetStandardInput>(
    color: impl Fn(&mut State, &Context) -> Color,
) -> impl Widget<State, Context> {
    Fill {
        color,
        rounding: |_, _| 0.0,
        bounding_rect: Box2::ZERO,
        consume_pointer_events: false,
        phantom: std::marker::PhantomData,
    }
}

pub fn rounded_fill<State, Context: GetStandardInput>(
    color: impl Fn(&mut State, &Context) -> Color,
    rounding: impl Fn(&mut State, &Context) -> f32,
) -> impl Widget<State, Context> {
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
    GetColor: Fn(&mut State, &Context) -> Color,
    GetRounding: Fn(&mut State, &Context) -> f32,
> {
    pub color: GetColor,
    pub rounding: GetRounding,
    pub bounding_rect: Box2,
    pub consume_pointer_events: bool,
    phantom: std::marker::PhantomData<fn() -> (State, Context)>,
}

impl<
        State,
        Context: GetStandardInput,
        GetColor: Fn(&mut State, &Context) -> Color,
        GetRounding: Fn(&mut State, &Context) -> f32,
    > Fill<State, Context, GetColor, GetRounding>
{
    pub fn pass_through_events(mut self) -> Self {
        self.consume_pointer_events = false;
        self
    }
}

impl<
        State,
        Context: GetStandardInput,
        GetColor: Fn(&mut State, &Context) -> Color,
        GetRounding: Fn(&mut State, &Context) -> f32,
    > Widget<State, Context> for Fill<State, Context, GetColor, GetRounding>
{
    fn layout(
        &mut self,
        _data: &mut State,
        _context: &mut Context,
        _min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        Vec3::ZERO
    }
    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        drawer.standard().rounded_rectangle(
            bounds,
            Vec4::fill((self.rounding)(state, context)),
            (self.color)(state, context),
        );
        self.bounding_rect = Box2 {
            min: bounds.min.xy(),
            max: bounds.max.xy(),
        };
    }
    fn update(&mut self, _: &mut State, context: &mut Context) {
        // If this fill is inside a button (which clones Context) it won't be able to access the standard context mutably.
        // Just don't consume events in that case.
        if self.consume_pointer_events {
            let standard_input = context.standard_input_mut();
            for (handled, event) in standard_input.input_events_iter() {
                match event {
                    kapp_platform_common::Event::PointerDown { x, y, .. }
                    | kapp_platform_common::Event::PointerUp { x, y, .. } => {
                        if self
                            .bounding_rect
                            .contains_point(Vec2::new(x as f32, y as f32))
                        {
                            println!("CONSUMED EVENT: {:?}", self.bounding_rect);
                            *handled = true;
                        }
                    }

                    _ => {}
                }
            }
        }
    }
}
