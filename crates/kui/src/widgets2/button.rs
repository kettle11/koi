use std::cell::RefCell;

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
pub fn button<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetFonts + GetEventHandlers<State>,
>(
    text: impl Into<TextSource<State>>,
    on_click: fn(&mut State),
) -> impl Widget<State, Context> {
    button_with_child(crate::text(text), on_click)
}

pub fn button_with_child<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State>,
>(
    child_widget: impl Widget<State, Context>,
    on_click: fn(&mut State),
) -> impl Widget<State, Context> {
    let child_widget = fit(stack((
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
    )));
    button_base(child_widget, on_click)
}

pub fn toggle_button<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State>,
    EditState: 'static + Copy + PartialEq,
>(
    child: impl Widget<State, Context>,
    get_state: fn(&mut State) -> &mut EditState,
    state_value: impl Fn(&mut State) -> EditState + Clone + 'static,
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

pub fn button_base<
    State,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State>,
>(
    child_widget: impl Widget<State, Context>,
    on_click: impl Fn(&mut State) + 'static,
) -> impl Widget<State, Context> {
    let clicked = Rc::new(RefCell::new(false));

    ButtonBase {
        child_widget,
        bounding_rect: Box2::ZERO,
        clicked: clicked.clone(),
        on_click: Rc::new(
            move |event: &kapp_platform_common::Event,
                  pointer_event_info: PointerEventInfo,
                  state: &mut State| match event {
                kapp_platform_common::Event::PointerDown { .. } => {
                    if pointer_event_info.in_hitbox {
                        *clicked.borrow_mut() = true;
                        on_click(state);
                    }
                }
                kapp_platform_common::Event::PointerUp { .. } => {
                    *clicked.borrow_mut() = false;
                }
                _ => {}
            },
        ),
        phantom: std::marker::PhantomData,
    }
}

pub struct ButtonBase<State, Context, Child: Widget<State, Context>> {
    child_widget: Child,
    bounding_rect: Box2,
    on_click: Rc<dyn Fn(&kapp_platform_common::Event, PointerEventInfo, &mut State) + 'static>,
    clicked: Rc<RefCell<bool>>,
    phantom: std::marker::PhantomData<fn() -> (Context, State)>,
}

impl<State, Context: GetStandardInput + GetEventHandlers<State>, Child: Widget<State, Context>>
    Widget<State, Context> for ButtonBase<State, Context, Child>
{
    fn layout(
        &mut self,
        state: &mut State,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        context.standard_input_mut().button_clicked = *self.clicked.borrow_mut();
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
        context.standard_input_mut().button_clicked = *self.clicked.borrow_mut();
        let size = self.bounding_rect.size().min(constraints.size().xy());
        self.bounding_rect = Box2::new_with_min_corner_and_size(constraints.min.xy(), size);
        self.child_widget.draw(state, context, drawer, constraints);
        context
            .event_handlers_mut()
            .add_pointer_event_handler(constraints, self.on_click.clone())
    }
}
