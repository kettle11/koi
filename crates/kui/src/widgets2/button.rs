use crate::*;

pub fn narrow_context<Data, OuterContext, InnerContext>(
    narrow_context: fn(&mut OuterContext) -> &mut InnerContext,
    child: impl Widget<Data, InnerContext>,
) -> impl Widget<Data, OuterContext> {
    NarrowContext {
        narrow_context,
        child,
        phantom: std::marker::PhantomData,
    }
}

pub struct NarrowContext<Data, OuterContext, InnerContext, Child: Widget<Data, InnerContext>> {
    narrow_context: fn(&mut OuterContext) -> &mut InnerContext,
    child: Child,
    phantom: std::marker::PhantomData<Data>,
}
impl<Data, OuterContext, InnerContext, Child: Widget<Data, InnerContext>> Widget<Data, OuterContext>
    for NarrowContext<Data, OuterContext, InnerContext, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        context: &mut OuterContext,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let context = (self.narrow_context)(context);
        self.child.layout(state, context, min_and_max_size)
    }
    fn draw(
        &mut self,
        state: &mut Data,
        context: &mut OuterContext,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let context = (self.narrow_context)(context);
        self.child.draw(state, context, drawer, constraints)
    }
}
pub fn button<State, Context: GetStandardInput + GetStandardStyle + GetFonts>(
    text: impl Into<TextSource<State>>,
    on_click: fn(&mut State),
) -> impl Widget<State, Context> {
    button_with_child(crate::text(text), on_click)
}

pub fn button_with_child<State, Context: GetStandardInput + GetStandardStyle>(
    child_widget: impl Widget<State, Context>,
    on_click: fn(&mut State),
) -> impl Widget<State, Context> {
    ButtonBase {
        child_widget: fit(stack((
            rounded_fill(
                |_, c: &Context| {
                    if c.standard_input().button_clicked {
                        c.standard_style().disabled_color
                    } else {
                        c.standard_style().primary_color
                    }
                },
                |_, c| c.standard_style().rounding,
            ),
            padding(|c: &Context| c.standard_style().padding, child_widget),
        ))),
        bounding_rect: Box2::ZERO,
        on_click,
        clicked: false,
        phantom: std::marker::PhantomData,
    }
}

pub fn toggle_button<
    State,
    Context: GetStandardInput + GetStandardStyle,
    EditState: 'static + Copy + PartialEq,
>(
    child: impl Widget<State, Context>,
    get_state: fn(&mut State) -> &mut EditState,
    state_value: impl Fn(&mut State) -> EditState + Clone,
) -> impl Widget<State, Context> {
    let state_value_0 = state_value.clone();
    button_base(
        fit(stack((
            rounded_fill(
                move |state, c: &Context| {
                    let current_state = (state_value_0)(state);
                    let selected = *get_state(state) == current_state;
                    if c.standard_input().button_clicked || selected {
                        c.standard_style().disabled_color
                    } else {
                        c.standard_style().primary_color
                    }
                },
                |_, c| c.standard_style().rounding,
            ),
            padding(|c: &Context| c.standard_style().padding, child),
        ))),
        move |state| {
            let new_value = (state_value)(state);
            let edit_state = get_state(state);
            *edit_state = new_value;
        },
    )
}

pub fn button_base<State, Context: GetStandardInput + GetStandardStyle>(
    child_widget: impl Widget<State, Context>,
    on_click: impl Fn(&mut State),
) -> impl Widget<State, Context> {
    ButtonBase {
        child_widget,
        bounding_rect: Box2::ZERO,
        on_click,
        clicked: false,
        phantom: std::marker::PhantomData,
    }
}

pub struct ButtonBase<State, Context, Child: Widget<State, Context>, OnClick: Fn(&mut State)> {
    child_widget: Child,
    bounding_rect: Box2,
    on_click: OnClick,
    clicked: bool,
    phantom: std::marker::PhantomData<fn() -> (Context, State)>,
}

impl<State, Context: GetStandardInput, Child: Widget<State, Context>, OnClick: Fn(&mut State)>
    Widget<State, Context> for ButtonBase<State, Context, Child, OnClick>
{
    fn update(&mut self, state: &mut State, context: &mut Context) {
        let standard_input = context.standard_input_mut();

        for (handled, event) in standard_input.input_events_iter() {
            if *handled {
                continue;
            }
            match event {
                kapp_platform_common::Event::PointerDown {
                    x,
                    y,
                    button: kapp_platform_common::PointerButton::Primary,
                    ..
                } => {
                    if self
                        .bounding_rect
                        .contains_point(Vec2::new(x as f32, y as f32))
                    {
                        if !self.clicked {
                            self.clicked = true;
                            (self.on_click)(state)
                        }
                        *handled = true;
                    }
                }
                kapp_platform_common::Event::PointerMoved { x, y, .. } => {
                    if self
                        .bounding_rect
                        .contains_point(Vec2::new(x as f32, y as f32))
                    {
                        *handled = true;
                    }
                }
                kapp_platform_common::Event::PointerUp {
                    button: kapp_platform_common::PointerButton::Primary,
                    ..
                } => {
                    self.clicked = false;
                }
                _ => {}
            }
        }

        // Todo: Check for input here and handle click event.
        self.child_widget.update(state, context);
    }
    fn layout(
        &mut self,
        state: &mut State,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        context.standard_input_mut().button_clicked = self.clicked;
        let child_size = self.child_widget.layout(state, context, min_and_max_size);
        self.bounding_rect = Box2 {
            min: Vec2::ZERO,
            max: child_size.xy().min(min_and_max_size.max.xy()),
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
        context.standard_input_mut().button_clicked = self.clicked;
        let size = self.bounding_rect.size().min(constraints.size().xy());
        self.bounding_rect = Box2::new_with_min_corner_and_size(constraints.min.xy(), size);
        self.child_widget.draw(state, context, drawer, constraints);
    }
}
