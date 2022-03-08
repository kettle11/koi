use crate::*;

pub enum Alignment {
    Start,
    End,
}
// In the future this should be generalized.
pub fn align<Data, Context, ExtraState>(
    horizontal: Alignment,
    vertical: Alignment,
    child: impl Widget<Data, Context, ExtraState>,
) -> impl Widget<Data, Context, ExtraState> {
    Align {
        child,
        child_size: Vec3::ZERO,
        align_direction: Vec3::new(
            match horizontal {
                Alignment::Start => 0.0,
                Alignment::End => 1.0,
            },
            match vertical {
                Alignment::Start => 0.0,
                Alignment::End => 1.0,
            },
            0.0,
        ),
        phantom: std::marker::PhantomData,
    }
}

struct Align<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>> {
    child: Child,
    child_size: Vec3,
    align_direction: Vec3,
    phantom: std::marker::PhantomData<fn() -> (Data, ExtraState, Context)>,
}

impl<Data, Context, ExtraState, Child: Widget<Data, Context, ExtraState>>
    Widget<Data, Context, ExtraState> for Align<Data, Context, ExtraState, Child>
{
    fn layout(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let child_size = self
            .child
            .layout(state, extra_state, context, min_and_max_size);
        self.child_size = child_size;
        child_size
    }
    fn draw(
        &mut self,
        state: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Box3,
    ) {
        let inverse = Vec3::new(
            if self.align_direction.x == 0.0 {
                1.0
            } else {
                0.0
            },
            if self.align_direction.y == 0.0 {
                1.0
            } else {
                0.0
            },
            if self.align_direction.z == 0.0 {
                1.0
            } else {
                0.0
            },
        );
        let min = constraints.min.mul_by_component(inverse)
            + constraints.max.mul_by_component(self.align_direction)
            - self.child_size.mul_by_component(self.align_direction);

        let max = min + self.child_size;

        self.child
            .draw(state, extra_state, context, drawer, Box3 { min, max })
    }
}
