use std::cell::RefCell;

use crate::*;

pub fn link<
    State,
    Context: GetStandardStyle + GetFonts + GetStandardInput + GetEventHandlers<State>,
    ExtraState,
>(
    get_url: impl Fn(&mut State) -> &str + 'static,
    child: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    on_click(
        move |state| {
            let url = (get_url)(state);
            println!("CLICKED URL: {:?}", url);

            #[cfg(target_arch = "wasm32")]
            kwasm::libraries::eval(&format!("window.open(\"{}\")", url));
        },
        set_cursor_on_hover(
            kapp_platform_common::Cursor::PointingHand,
            fit(column_unspaced((
                child,
                conditional(
                    |_, context| context.standard_input().element_hovered,
                    height(
                        1.0,
                        fill(|_, _, context: &Context| context.standard_style().primary_text_color),
                    ),
                ),
            ))),
        ),
    )
}

pub fn track_hover<
    State,
    Context: GetStandardStyle + GetStandardInput + GetEventHandlers<State>,
    ExtraState,
>(
    child: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    let cursor_event_state = Rc::new(RefCell::new(CursorEventState {
        hovered: false,
        clicked: false,
    }));

    let state_value_0 = cursor_event_state.clone();

    OnCursorEvent {
        child_widget: child,
        bounding_rect: Box3::ZERO,
        on_event: Rc::new(move |event, pointer_event_info, _| {
            cursor_event_state.borrow_mut().hovered = pointer_event_info.in_hitbox;

            match event {
                kapp_platform_common::Event::PointerDown { .. } => {
                    if pointer_event_info.in_hitbox {
                        cursor_event_state.borrow_mut().clicked = true
                    }
                }
                kapp_platform_common::Event::PointerUp { .. } => {
                    cursor_event_state.borrow_mut().clicked = false
                }
                _ => {}
            }
        }),
        cursor_event_state: state_value_0,
        handle_event: false,
        phantom: std::marker::PhantomData,
    }
}

pub fn on_click<
    State,
    Context: GetStandardStyle + GetStandardInput + GetEventHandlers<State>,
    ExtraState,
>(
    on_click: impl Fn(&mut State) + 'static,
    child: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    let cursor_event_state = Rc::new(RefCell::new(CursorEventState {
        hovered: false,
        clicked: false,
    }));

    let state_value_0 = cursor_event_state.clone();

    OnCursorEvent {
        handle_event: true,
        child_widget: child,
        bounding_rect: Box3::ZERO,
        on_event: Rc::new(move |event, pointer_event_info, state| {
            cursor_event_state.borrow_mut().hovered = pointer_event_info.in_hitbox;

            match event {
                kapp_platform_common::Event::PointerDown { .. } => {
                    if pointer_event_info.in_hitbox {
                        cursor_event_state.borrow_mut().clicked = true;
                        (on_click)(state)
                    }
                }
                kapp_platform_common::Event::PointerUp { .. } => {
                    cursor_event_state.borrow_mut().clicked = false
                }
                _ => {}
            }
        }),
        cursor_event_state: state_value_0,
        phantom: std::marker::PhantomData,
    }
}

struct CursorEventState {
    clicked: bool,
    hovered: bool,
}
pub struct OnCursorEvent<State, Context, ExtraState, Child: Widget<State, Context, ExtraState>> {
    child_widget: Child,
    bounding_rect: Box3,
    on_event: Rc<dyn Fn(&kapp_platform_common::Event, PointerEventInfo, &mut State) + 'static>,
    cursor_event_state: Rc<RefCell<CursorEventState>>,
    handle_event: bool,
    phantom: std::marker::PhantomData<fn() -> (Context, State, ExtraState)>,
}

impl<
        State,
        Context: GetStandardInput + GetEventHandlers<State>,
        ExtraState,
        Child: Widget<State, Context, ExtraState>,
    > Widget<State, Context, ExtraState> for OnCursorEvent<State, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let cursor_event_state = self.cursor_event_state.borrow_mut();
        let standard_input = context.standard_input_mut();
        standard_input.button_clicked = cursor_event_state.clicked;
        standard_input.element_hovered = cursor_event_state.hovered;

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
        let cursor_event_state = self.cursor_event_state.borrow_mut();
        let standard_input = context.standard_input_mut();
        standard_input.button_clicked = cursor_event_state.clicked;
        standard_input.element_hovered = cursor_event_state.hovered;

        let size = self.bounding_rect.size().min(constraints.size());
        self.bounding_rect = Box3::new_with_min_corner_and_size(constraints.min, size);
        self.child_widget
            .draw(state, extra_state, context, drawer, constraints);
        context.event_handlers_mut().add_pointer_event_handler(
            self.bounding_rect,
            self.handle_event,
            Some(self.on_event.clone()),
        )
    }
}

pub fn set_cursor_on_hover<
    State,
    Context: GetStandardStyle + GetStandardInput + GetEventHandlers<State>,
    ExtraState,
>(
    cursor: kapp_platform_common::Cursor,
    child: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    track_hover(SetCursorOnHover {
        child_widget: child,
        cursor,
        phantom: std::marker::PhantomData,
    })
}
pub struct SetCursorOnHover<State, Context, ExtraState, Child: Widget<State, Context, ExtraState>> {
    child_widget: Child,
    cursor: kapp_platform_common::Cursor,
    phantom: std::marker::PhantomData<fn() -> (Context, State, ExtraState)>,
}

impl<
        State,
        Context: GetStandardInput + GetEventHandlers<State>,
        ExtraState,
        Child: Widget<State, Context, ExtraState>,
    > Widget<State, Context, ExtraState> for SetCursorOnHover<State, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.child_widget
            .layout(state, extra_state, context, min_and_max_size)
    }
    fn draw(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let standard_input = context.standard_input_mut();

        if standard_input.element_hovered {
            standard_input.cursor = self.cursor;
        }
        self.child_widget
            .draw(state, extra_state, context, drawer, constraints)
    }
}
