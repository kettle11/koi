use koi::*;
use kui::*;

fn standard_frame<Data, Context: GetStandardStyle + GetStandardInput>(
    child: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    stack((
        fill(|_, c: &Context| c.standard_style().background_color),
        padding(|_| 50., child),
    ))
}

fn dark_authoratative_style() -> StandardStyle {
    StandardStyle {
        heading_text_size: 100.,
        heading_font: Font::from_index(1),
        primary_text_color: Color::WHITE,
        primary_color: Color::new_from_bytes(20, 20, 20, 255),
        background_color: Color::new_from_bytes(20, 20, 20, 255),
        ..Default::default()
    }
}

fn main() {
    let ui = standard_frame(column((
        heading("Application"),
        button("Hello", |_| println!("CLICKED")),
    )));

    let mut fonts = Fonts::empty();
    fonts.load_default_fonts();

    let style = StandardStyle::default();

    run_simple_ui((), style, fonts, ui)
}
