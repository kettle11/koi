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
            open_url(url);
        },
        true,
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

pub fn open_url(url: &str) {
    println!("OPENING URL: {:?}", url);
    #[cfg(target_arch = "wasm32")]
    {
        thread_local! {
            static OPEN_URL: kwasm::JSObjectFromString = kwasm::JSObjectFromString::new(r#"
            function open_url (url_index) {
                let url = self.kwasm_get_object(url_index);
                window.open(url, '_blank');
                console.log('OPENING URL:' + url);
                console.trace();
            }
            open_url"#);
        }

        let js_url = kwasm::JSString::new(url);

        OPEN_URL.with(|v| v.call_raw(&[js_url.index()]));
    }
}
