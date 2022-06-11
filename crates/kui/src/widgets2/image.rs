use crate::*;

pub fn image<State, Context, ExtraState>(
    get_image: impl Fn(&mut State, &mut ExtraState, &Context) -> ImageHandle,
) -> impl Widget<State, Context, ExtraState> {
    Image {
        get_image,
        phantom: std::marker::PhantomData,
    }
}

pub struct Image<
    State,
    Context,
    ExtraState,
    GetImage: Fn(&mut State, &mut ExtraState, &Context) -> ImageHandle,
> {
    pub get_image: GetImage,
    phantom: std::marker::PhantomData<fn() -> (State, Context, ExtraState)>,
}

impl<
        State,
        Context,
        ExtraState,
        GetImage: Fn(&mut State, &mut ExtraState, &Context) -> ImageHandle,
    > Widget<State, Context, ExtraState> for Image<State, Context, ExtraState, GetImage>
{
    fn layout(
        &mut self,
        _data: &mut State,
        _extra_state: &mut ExtraState,
        _context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        min_and_max_size.max
    }
    fn draw(
        &mut self,
        state: &mut State,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let image_handle = (self.get_image)(state, extra_state, context);
        drawer.image(bounds, &image_handle)
    }
}
