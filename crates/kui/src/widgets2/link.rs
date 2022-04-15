use crate::*;

pub fn link<
    State,
    Context: GetStandardStyle + GetFonts + GetStandardInput + GetEventHandlers<State>,
    ExtraState,
>(
    get_url: impl Fn(&mut State) -> &str + 'static,
    child: impl Widget<State, Context, ExtraState>,
) -> impl Widget<State, Context, ExtraState> {
    on_cursor_event(
        move |state| {
            let url = (get_url)(state);
            println!("CLICKED URL: {:?}", url);

            #[cfg(target_arch = "wasm32")]
            kwasm::libraries::eval(&format!("window.open(\"{}\")", url));
        },
        true,
        set_cursor_on_hover(
            kapp_platform_common::Cursor::PointingHand,
            fit(column_unspaced((
                child,
                height(
                    1.0,
                    fill(|_, _, context: &Context| context.standard_style().primary_text_color),
                ),
            ))),
        ),
    )
}
