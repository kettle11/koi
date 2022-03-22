use std::cell::RefCell;

use crate::*;

pub fn button<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetFonts + GetEventHandlers<State>,
    ExtraState,
>(
    text: impl Into<TextSource<State>>,
    on_click: fn(&mut State),
) -> impl Widget<State, Context, ExtraState> {
    button_with_child(crate::text(text), on_click)
}

pub fn button_with_child<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State>,
    ExtraState,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
    on_click: fn(&mut State),
) -> impl Widget<State, Context, ExtraState> {
    let child_widget = fit(stack((
        rounded_fill(
            |_, _, c: &Context| {
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
    ExtraState,
    EditState: 'static + Copy + PartialEq,
>(
    child: impl Widget<State, Context, ExtraState>,
    get_state: fn(&mut State) -> &mut EditState,
    state_value: impl Fn(&mut State) -> EditState + Clone + 'static,
) -> impl Widget<State, Context, ExtraState> {
    let state_value_0 = state_value.clone();
    button_base(
        fit(stack((
            rounded_fill(
                move |state, _, c: &Context| {
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
    ExtraState,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State>,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
    on_click: impl Fn(&mut State) + 'static,
) -> impl Widget<State, Context, ExtraState> {
    let clicked = Rc::new(RefCell::new(false));

    ButtonBase {
        child_widget,
        bounding_rect: Box3::ZERO,
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

pub struct ButtonBase<State, Context, ExtraState, Child: Widget<State, Context, ExtraState>> {
    child_widget: Child,
    bounding_rect: Box3,
    on_click: Rc<dyn Fn(&kapp_platform_common::Event, PointerEventInfo, &mut State) + 'static>,
    clicked: Rc<RefCell<bool>>,
    phantom: std::marker::PhantomData<fn() -> (Context, State, ExtraState)>,
}

impl<
        State,
        Context: GetStandardInput + GetEventHandlers<State>,
        ExtraState,
        Child: Widget<State, Context, ExtraState>,
    > Widget<State, Context, ExtraState> for ButtonBase<State, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        context.standard_input_mut().button_clicked = *self.clicked.borrow_mut();
        let child_size = self
            .child_widget
            .layout(state, extra_state, context, min_and_max_size);
        self.bounding_rect = Box3 {
            min: Vec3::ZERO,
            max: child_size.min(min_and_max_size.max),
        };
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
        context.standard_input_mut().button_clicked = *self.clicked.borrow_mut();
        let size = self.bounding_rect.size().min(constraints.size());
        self.bounding_rect = Box3::new_with_min_corner_and_size(constraints.min, size);
        self.child_widget
            .draw(state, extra_state, context, drawer, constraints);
        context
            .event_handlers_mut()
            .add_pointer_event_handler(self.bounding_rect, Some(self.on_click.clone()))
    }
}
