use std::cell::RefCell;

use crate::*;

pub trait Slideable: Copy + Clone + 'static {
    fn slide(min: Self, max: Self, v: f32) -> Self;
    fn get_percent(self, min: Self, max: Self) -> f32;
    fn clamp_slideable(self, min: Self, max: Self) -> Self;
}

impl Slideable for f32 {
    fn slide(min: Self, max: Self, v: f32) -> Self {
        (max - min) * v
    }
    fn get_percent(self, min: Self, max: Self) -> f32 {
        (self - min) / (max - min)
    }
    fn clamp_slideable(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Slideable for i32 {
    fn slide(min: Self, max: Self, v: f32) -> Self {
        ((max - min) as f32 * v) as Self
    }
    fn get_percent(self, min: Self, max: Self) -> f32 {
        (self as f32 - min as f32) / (max - min) as f32
    }
    fn clamp_slideable(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Slideable for i64 {
    fn slide(min: Self, max: Self, v: f32) -> Self {
        ((max - min) as f32 * v) as Self
    }
    fn get_percent(self, min: Self, max: Self) -> f32 {
        (self as f32 - min as f32) / (max - min) as f32
    }
    fn clamp_slideable(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Slideable for usize {
    fn slide(min: Self, max: Self, v: f32) -> Self {
        ((max - min) as f32 * v) as Self
    }
    fn get_percent(self, min: Self, max: Self) -> f32 {
        (self as f32 - min as f32) / (max - min) as f32
    }
    fn clamp_slideable(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

pub fn slider<
    State: 'static,
    ExtraState,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State>,
    T: Slideable,
>(
    current: fn(&mut State) -> &mut T,
    min: T,
    max: T,
) -> impl Widget<State, Context, ExtraState> {
    let clicked = Rc::new(RefCell::new(SliderSharedState {
        hitbox: Box2::ZERO,
        clicked: false,
    }));

    Slider {
        min,
        max,
        current,
        child_handle_width: 0.0,
        backdrop_child: center(expand_horizontal(height(
            10.,
            rounded_fill_pass_through(
                |_, _, c: &Context| c.standard_style().primary_variant_color,
                |_, c| c.standard_style().rounding,
            ),
        ))),
        handle_child: center(exact_size(
            Vec3::new(24., 24., 1.0),
            rounded_fill_pass_through(
                |_, _, c: &Context| c.standard_style().disabled_color,
                |_, _| 12.,
            ),
        )),
        clicked: clicked.clone(),
        on_click: Rc::new(
            move |event: &kapp_platform_common::Event,
                  pointer_event_info: PointerEventInfo,
                  state: &mut State| {
                let mut shared_state = clicked.borrow_mut();
                let mut cursor_position_moved = None;
                match event {
                    kapp_platform_common::Event::PointerDown { x, y, .. } => {
                        if pointer_event_info.in_hitbox {
                            shared_state.clicked = true;
                            cursor_position_moved = Some(Vec2::new(*x as f32, *y as f32));
                        }
                    }
                    kapp_platform_common::Event::PointerMoved { x, y, .. } => {
                        cursor_position_moved = Some(Vec2::new(*x as f32, *y as f32));
                    }
                    kapp_platform_common::Event::PointerUp { .. } => {
                        shared_state.clicked = false;
                    }
                    _ => {}
                }

                if shared_state.clicked {
                    if let Some(cursor_position_moved) = cursor_position_moved {
                        let bounds: Box2 = shared_state.hitbox;
                        let v = (cursor_position_moved.x - bounds.min.x) / bounds.size().x;
                        let current_value = (current)(state);
                        *current_value = T::slide(min, max, v).clamp_slideable(min, max);
                    }
                }
            },
        ),
        phantom: std::marker::PhantomData,
    }
}

struct SliderSharedState {
    hitbox: Box2,
    clicked: bool,
}
pub struct Slider<
    State,
    Context,
    ExtraState,
    BackdropChild: Widget<State, Context, ExtraState>,
    HandleChild: Widget<State, Context, ExtraState>,
    T: Slideable,
> {
    backdrop_child: BackdropChild,
    handle_child: HandleChild,
    child_handle_width: f32,
    min: T,
    max: T,
    current: fn(&mut State) -> &mut T,
    on_click: Rc<dyn Fn(&kapp_platform_common::Event, PointerEventInfo, &mut State) + 'static>,
    clicked: Rc<RefCell<SliderSharedState>>,
    phantom: std::marker::PhantomData<fn() -> (Context, State, ExtraState)>,
}

impl<
        State,
        Context: GetStandardInput + GetEventHandlers<State>,
        ExtraState,
        BackdropChild: Widget<State, Context, ExtraState>,
        HandleChild: Widget<State, Context, ExtraState>,
        T: Slideable,
    > Widget<State, Context, ExtraState>
    for Slider<State, Context, ExtraState, BackdropChild, HandleChild, T>
{
    fn layout(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        // let backdrop_size =
        self.backdrop_child
            .layout(state, extra_state, context, min_and_max_size);
        let handle_size = self
            .handle_child
            .layout(state, extra_state, context, min_and_max_size);
        self.child_handle_width = handle_size.x;
        handle_size
    }
    fn draw(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        self.clicked.borrow_mut().hitbox = Box2::new(constraints.min.xy(), constraints.max.xy());

        self.backdrop_child
            .draw(state, extra_state, context, drawer, constraints);

        let v = (self.current)(state).clamp_slideable(self.min, self.max);
        let percent = v.get_percent(self.min, self.max);
        let size = constraints.size();

        let min = constraints.min + Vec3::X * (size.x * percent - self.child_handle_width / 2.0);
        let handle_constraints = Box3 {
            min: min.max(constraints.min),
            max: (Vec3::new(
                min.x + self.child_handle_width,
                constraints.max.y,
                constraints.max.z,
            ))
            .min(constraints.max),
        };
        self.handle_child
            .draw(state, extra_state, context, drawer, handle_constraints);
        context
            .event_handlers_mut()
            .add_pointer_event_handler(constraints, Some(self.on_click.clone()))
    }
}
