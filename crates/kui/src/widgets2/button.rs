use crate::*;

pub fn button<
    State: 'static,
    Context: GetStandardInput
        + GetStandardStyle
        + GetFonts
        + GetEventHandlers<State>
        + GetAnimationValueTrait,
    ExtraState,
>(
    text: impl Into<TextSource<State>>,
    on_click: fn(&mut State),
) -> impl Widget<State, Context, ExtraState> {
    button_with_child(crate::text(text), on_click)
}

pub fn button_with_child_inner<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State> + GetAnimationValueTrait,
    ExtraState,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
    on_up: bool,
    on_click: fn(&mut State),
) -> impl Widget<State, Context, ExtraState> {
    let child_widget = hover_animation(fit(stack((
        rounded_fill(
            |_, _, c: &Context| {
                if c.standard_input().button_clicked {
                    c.standard_style().disabled_color
                } else {
                    Color::interpolate(
                        c.standard_style().primary_color,
                        c.standard_style().disabled_color,
                        c.animation_value().min(0.5),
                    )
                    //c.standard_style().primary_color
                }
            },
            |_, c| c.standard_style().rounding,
        ),
        padding(child_widget),
    ))));
    button_base(child_widget, on_click, on_up)
}

pub fn button_with_child<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State> + GetAnimationValueTrait,
    ExtraState,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
    on_click: fn(&mut State),
) -> impl Widget<State, Context, ExtraState> {
    button_with_child_inner(child_widget, false, on_click)
}

pub fn bool_button<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State> + GetAnimationValueTrait,
    ExtraState,
>(
    child: impl Widget<State, Context, ExtraState>,
    get_state: fn(&mut State) -> &mut bool,
) -> impl Widget<State, Context, ExtraState> {
    hover_animation(button_base(
        fit(stack((
            rounded_fill(
                move |state, _, c: &Context| {
                    let selected = *(get_state)(state);
                    if c.standard_input().button_clicked || selected {
                        c.standard_style().disabled_color
                    } else {
                        Color::interpolate(
                            c.standard_style().primary_color,
                            c.standard_style().disabled_color,
                            c.animation_value().min(0.5),
                        )
                    }
                },
                |_, c| c.standard_style().rounding,
            ),
            padding(child),
        ))),
        move |state| {
            let new_value = (get_state)(state);
            *new_value = !*new_value
        },
        false,
    ))
}

pub fn toggle_button<
    State: 'static,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State> + GetAnimationValueTrait,
    ExtraState,
    EditState: 'static + Copy + PartialEq,
>(
    child: impl Widget<State, Context, ExtraState>,
    get_state: fn(&mut State) -> &mut EditState,
    state_value: impl Fn(&mut State) -> EditState + Clone + 'static,
) -> impl Widget<State, Context, ExtraState> {
    let state_value_0 = state_value.clone();

    hover_animation(button_base(
        fit(stack((
            rounded_fill(
                move |state, _, c: &Context| {
                    let current_state = (state_value_0)(state);
                    let selected = *get_state(state) == current_state;
                    if c.standard_input().button_clicked || selected {
                        c.standard_style().disabled_color
                    } else {
                        Color::interpolate(
                            c.standard_style().primary_color,
                            c.standard_style().disabled_color,
                            c.animation_value().min(0.5),
                        )
                    }
                },
                |_, c| c.standard_style().rounding,
            ),
            padding(child),
        ))),
        move |state| {
            let new_value = (state_value)(state);
            let edit_state = get_state(state);
            *edit_state = new_value;
        },
        false,
    ))
}

pub fn button_base<
    State,
    ExtraState,
    Context: GetStandardInput + GetStandardStyle + GetEventHandlers<State>,
>(
    child_widget: impl Widget<State, Context, ExtraState>,
    on_click: impl Fn(&mut State) + 'static,
    on_up: bool,
) -> impl Widget<State, Context, ExtraState> {
    crate::on_cursor_event(
        on_click,
        true,
        on_up,
        set_cursor_on_hover(kapp_platform_common::Cursor::PointingHand, child_widget),
    )
}
