use crate::*;

pub fn animation<Data, Context: GetAnimationValueTrait, ExtraState>(
    get_target_value: impl Fn(&mut Data, &mut ExtraState, &Context) -> f32,
    speed_seconds: f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Animation {
        child,
        current_t: 0.0,
        target_t: 0.0,
        rate_change_t: speed_seconds,
        get_animation_value: get_target_value,
        phantom: std::marker::PhantomData,
    }
}

pub fn toggle_animation<Data, Context: GetAnimationValueTrait, ExtraState>(
    get_state: impl Fn(&mut Data, &mut ExtraState, &Context) -> bool,
    speed_seconds: f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Animation {
        child,
        current_t: 0.0,
        target_t: 0.0,
        rate_change_t: speed_seconds,
        get_animation_value: move |data, extra_state, context| {
            if (get_state)(data, extra_state, context) {
                1.0
            } else {
                0.0
            }
        },
        phantom: std::marker::PhantomData,
    }
}

pub fn hover_animation<
    Data,
    Context: GetAnimationValueTrait + GetStandardInput + GetStandardStyle + GetEventHandlers<Data>,
    ExtraState,
>(
    speed_seconds: f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    track_hover(toggle_animation(
        |_, _, c| c.standard_input().element_hovered,
        speed_seconds,
        child,
    ))
}

pub struct Animation<
    Data,
    Context: GetAnimationValueTrait,
    ExtraState,
    Child: Widget<Data, Context, ExtraState>,
    GetAnimationValue: Fn(&mut Data, &mut ExtraState, &Context) -> f32,
> {
    current_t: f32,
    target_t: f32,
    rate_change_t: f32,
    get_animation_value: GetAnimationValue,
    child: Child,
    phantom: std::marker::PhantomData<fn() -> (Data, ExtraState, Context)>,
}

impl<
        Data,
        Context: GetAnimationValueTrait,
        ExtraState,
        Child: Widget<Data, Context, ExtraState>,
        GetAnimationValue: Fn(&mut Data, &mut ExtraState, &Context) -> f32,
    > Widget<Data, Context, ExtraState>
    for Animation<Data, Context, ExtraState, Child, GetAnimationValue>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.target_t = (self.get_animation_value)(state, extra_state, context);
        if self.current_t < self.target_t {
            context.set_needs_redraw();
            self.current_t += context.delta_time_seconds() / self.rate_change_t;
            if self.current_t > self.target_t {
                self.current_t = self.target_t;
            }
        } else if self.current_t > self.target_t {
            context.set_needs_redraw();
            self.current_t -= context.delta_time_seconds() / self.rate_change_t;
            if self.current_t < self.target_t {
                self.current_t = self.target_t;
            }
        }

        let old_animation_value = context.animation_value();
        *context.animation_value_mut() = self.current_t;

        let layout_result = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        *context.animation_value_mut() = old_animation_value;
        layout_result
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let old_animation_value = context.animation_value();
        *context.animation_value_mut() = self.current_t;
        self.child
            .draw(state, extra_state, context, drawer, constraints);
        *context.animation_value_mut() = old_animation_value;
    }
}

pub fn animation_curve<Data, Context: GetAnimationValueTrait, ExtraState>(
    get_animation_curve: impl Fn(&mut Data, &mut ExtraState, &Context) -> fn(f32) -> f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    AnimationCurve {
        get_animation_curve,
        child,
        phantom: std::marker::PhantomData,
    }
}

pub struct AnimationCurve<
    Data,
    Context: GetAnimationValueTrait,
    ExtraState,
    Child: Widget<Data, Context, ExtraState>,
    GetAnimationCurve: Fn(&mut Data, &mut ExtraState, &Context) -> fn(f32) -> f32,
> {
    get_animation_curve: GetAnimationCurve,
    child: Child,
    phantom: std::marker::PhantomData<fn() -> (Data, ExtraState, Context)>,
}

impl<
        Data,
        Context: GetAnimationValueTrait,
        ExtraState,
        Child: Widget<Data, Context, ExtraState>,
        GetAnimationCurve: Fn(&mut Data, &mut ExtraState, &Context) -> fn(f32) -> f32,
    > Widget<Data, Context, ExtraState>
    for AnimationCurve<Data, Context, ExtraState, Child, GetAnimationCurve>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let old_animation_value = context.animation_value();
        *context.animation_value_mut() =
            ((self.get_animation_curve)(state, extra_state, context))(old_animation_value);

        let layout_result = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        *context.animation_value_mut() = old_animation_value;
        layout_result
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let old_animation_value = context.animation_value();
        *context.animation_value_mut() =
            ((self.get_animation_curve)(state, extra_state, context))(old_animation_value);
        self.child
            .draw(state, extra_state, context, drawer, constraints);
        *context.animation_value_mut() = old_animation_value;
    }
}
