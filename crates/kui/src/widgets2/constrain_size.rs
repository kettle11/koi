use crate::*;

/// Limits the childs size
pub fn max_size<Data, Context, ExtraState>(
    max_size: Vec3,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSize {
        child,
        min_size: Vec3::ZERO,
        max_size,
        phantom: std::marker::PhantomData,
    }
}

/// Limits the childs size
pub fn max_width<Data, Context, ExtraState>(
    width: f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSize {
        child,
        min_size: Vec3::ZERO,
        max_size: Vec3::new(width, f32::MAX, f32::MAX),
        phantom: std::marker::PhantomData,
    }
}

/// Limits the childs size
pub fn min_width<Data, Context, ExtraState>(
    width: f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSize {
        child,
        min_size: Vec3::new(width, 0.0, 0.0),
        max_size: Vec3::MAX,
        phantom: std::marker::PhantomData,
    }
}

/// Limits the childs size
pub fn min_height<Data, Context, ExtraState>(
    height: f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSize {
        child,
        min_size: Vec3::new(0.0, height, 0.0),
        max_size: Vec3::MAX,
        phantom: std::marker::PhantomData,
    }
}

pub fn min_size<Data, Context, ExtraState>(
    min_size: Vec3,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSize {
        child,
        min_size,
        max_size: Vec3::MAX,
        phantom: std::marker::PhantomData,
    }
}

pub fn width<Data, Context, ExtraState>(
    width: f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSize {
        child,
        min_size: Vec3::new(width, 0.0, 0.0),
        max_size: Vec3::new(width, f32::MAX, f32::MAX),
        phantom: std::marker::PhantomData,
    }
}

pub fn height<Data, Context, ExtraState>(
    height: f32,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSize {
        child,
        min_size: Vec3::new(0.0, height, 0.0),
        max_size: Vec3::new(f32::MAX, height, f32::MAX),
        phantom: std::marker::PhantomData,
    }
}

pub fn exact_size<Data, Context, ExtraState>(
    size: Vec3,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSize {
        child,
        min_size: size,
        max_size: size,
        phantom: std::marker::PhantomData,
    }
}

pub struct ConstrainSize<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    min_size: Vec3,
    max_size: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for ConstrainSize<Data, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        mut min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        min_and_max_size.max = min_and_max_size.max.min(self.max_size).max(self.min_size);
        let child_size = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        child_size.min(self.max_size).max(self.min_size)
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let box_size = constraints.size().min(self.max_size);
        let constraints = Box3 {
            min: constraints.min,
            max: constraints.min + box_size,
        };
        self.child
            .draw(state, extra_state, context, drawer, constraints)
    }
}

pub fn exact_size_dynamic<Data, Context, ExtraState>(
    get_min_max: impl Fn(&mut Data, &mut ExtraState, &Context) -> Vec3,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    ConstrainSizeDynamic {
        child,
        get_min_max: move |d, e, c| {
            let size = get_min_max(d, e, c);
            MinAndMaxSize {
                min: size,
                max: size,
            }
        },
        phantom: std::marker::PhantomData,
    }
}

pub struct ConstrainSizeDynamic<
    Data,
    Context,
    ExtraState,
    Child: Widget<Data, Context, ExtraState>,
    GetMinMax: Fn(&mut Data, &mut ExtraState, &Context) -> MinAndMaxSize,
> {
    child: Child,
    get_min_max: GetMinMax,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<
        Data,
        Context,
        ExtraState,
        Child: Widget<Data, Context, ExtraState>,
        GetMinMax: Fn(&mut Data, &mut ExtraState, &Context) -> MinAndMaxSize,
    > Widget<Data, Context, ExtraState>
    for ConstrainSizeDynamic<Data, Context, ExtraState, Child, GetMinMax>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        mut min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let min_max = (self.get_min_max)(state, extra_state, context);
        min_and_max_size.max = min_and_max_size.max.min(min_max.max).max(min_max.min);
        let child_size = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        child_size.min(min_max.max).max(min_max.min)
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let min_max = (self.get_min_max)(state, extra_state, context);

        let box_size = constraints.size().min(min_max.max);
        let constraints = Box3 {
            min: constraints.min,
            max: constraints.min + box_size,
        };
        self.child
            .draw(state, extra_state, context, drawer, constraints)
    }
}
