use crate::*;

fn narrow_context<Data, OuterContext, InnerContext>(
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
pub struct ButtonContext<Context> {
    context: Context,
    clicked: bool,
}

pub fn button<State, Context: GetStandardInput + GetStandardStyle + Clone + GetFonts>(
    on_click: fn(&mut State),
    text: impl Into<TextSource<State>>,
) -> impl Widget<State, Context> {
    button_with_child(on_click, crate::text(text))
}

pub fn button_with_child<State, Context: GetStandardInput + GetStandardStyle + Clone>(
    on_click: fn(&mut State),
    child_widget: impl Widget<State, Context>,
) -> impl Widget<State, Context> {
    ButtonBase {
        child_widget: fit(stack((
            outlined_rounded_fill(
                |c: &ButtonContext<Context>| c.context.standard_style().primary_variant_color,
                |c| {
                    if c.clicked {
                        c.context.standard_style().disabled_color
                    } else {
                        c.context.standard_style().primary_color
                    }
                },
                |c| c.context.standard_style().rounding,
            ),
            padding(
                |c: &ButtonContext<Context>| c.context.standard_style().padding,
                narrow_context(
                    |c: &mut ButtonContext<Context>| &mut c.context,
                    child_widget,
                ),
            ),
        ))),
        bounding_rect: Box2::ZERO,
        on_click,
        clicked: false,
        phantom: std::marker::PhantomData,
    }
}

pub struct ButtonBase<State, Context, Child: Widget<State, ButtonContext<Context>>> {
    child_widget: Child,
    bounding_rect: Box2,
    on_click: fn(&mut State),
    clicked: bool,
    phantom: std::marker::PhantomData<fn() -> Context>,
}

impl<State, Context: GetStandardInput + Clone, Child: Widget<State, ButtonContext<Context>>>
    Widget<State, Context> for ButtonBase<State, Context, Child>
{
    fn update(&mut self, state: &mut State, context: &mut Context) {
        let standard_input = context.standard_input_mut();

        for (handled, event) in standard_input.input_events_iter() {
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
                kapp_platform_common::Event::PointerUp {
                    button: kapp_platform_common::PointerButton::Primary,
                    ..
                } => self.clicked = false,
                _ => {}
            }
        }

        let mut context = ButtonContext {
            context: context.clone(),
            clicked: self.clicked,
        };
        // Todo: Check for input here and handle click event.
        self.child_widget.update(state, &mut context);
    }
    fn layout(
        &mut self,
        state: &mut State,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let mut context = ButtonContext {
            context: context.clone(),
            clicked: self.clicked,
        };
        let child_size = self
            .child_widget
            .layout(state, &mut context, min_and_max_size);
        self.bounding_rect = Box2 {
            min: Vec2::ZERO,
            max: child_size.xy(),
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
        let mut context = ButtonContext {
            context: context.clone(),
            clicked: self.clicked,
        };
        let size = self.bounding_rect.size().min(constraints.size().xy());
        self.bounding_rect = Box2::new_with_min_corner_and_size(constraints.min.xy(), size);
        self.child_widget
            .draw(state, &mut context, drawer, constraints);
    }
}
