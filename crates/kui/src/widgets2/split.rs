use crate::*;

pub fn split_horizontal<Data, Context, ExtraState>(
    get_split: impl Fn(&mut Data, &mut ExtraState, &Context) -> f32,
    child_a: impl Widget<Data, Context, ExtraState>,
    child_b: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Split {
        child_a,
        child_b,
        axis: Vec3::X,
        get_split,
        phantom: std::marker::PhantomData,
    }
}

pub fn split_vertical<Data, Context, ExtraState>(
    get_split: impl Fn(&mut Data, &mut ExtraState, &Context) -> f32,
    child_a: impl Widget<Data, Context, ExtraState>,
    child_b: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Split {
        child_a,
        child_b,
        axis: Vec3::Y,
        get_split,
        phantom: std::marker::PhantomData,
    }
}

pub struct Split<
    Data,
    Context,
    ExtraState,
    ChildA: Widget<Data, Context, ExtraState>,
    ChildB: Widget<Data, Context, ExtraState>,
    GetSplit: Fn(&mut Data, &mut ExtraState, &Context) -> f32,
> {
    child_a: ChildA,
    child_b: ChildB,
    axis: Vec3,
    get_split: GetSplit,
    phantom: std::marker::PhantomData<fn() -> (Data, Context, ExtraState)>,
}

impl<
        Data,
        Context,
        ExtraState,
        ChildA: Widget<Data, Context, ExtraState>,
        ChildB: Widget<Data, Context, ExtraState>,
        GetSplit: Fn(&mut Data, &mut ExtraState, &Context) -> f32,
    > Widget<Data, Context, ExtraState>
    for Split<Data, Context, ExtraState, ChildA, ChildB, GetSplit>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.child_a
            .layout(state, extra_state, context, min_and_max_size);
        self.child_b
            .layout(state, extra_state, context, min_and_max_size);
        min_and_max_size.max
    }

    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let size = constraints.size();
        let size_along_axis = size.dot(self.axis) * self.axis;
        let size_not_along_axis = size - size_along_axis;

        let split = (self.get_split)(state, extra_state, context);
        let size_a = size_not_along_axis + size_along_axis * split;
        let size_b = size_not_along_axis + size_along_axis * (1.0 - split);

        let child_a_max = constraints.min + size_a;
        self.child_a.draw(
            state,
            extra_state,
            context,
            drawer,
            Box3::new(constraints.min, child_a_max),
        );

        self.child_b.draw(
            state,
            extra_state,
            context,
            drawer,
            Box3::new(child_a_max.dot(self.axis) * self.axis, child_a_max + size_b),
        )
    }
}
