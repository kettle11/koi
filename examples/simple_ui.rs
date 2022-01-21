use koi::*;
use kui::*;

fn standard_frame<Data, Context: GetStandardStyle>(
    child: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    stack((
        fill(|c: &Context| c.standard_style().background_color),
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
        button(|_| println!("CLICKED"), text("Hello")),
    )));

    let mut fonts = Fonts::empty();
    fonts.load_default_fonts();
    let header_font = fonts
        .load_system_font(&[Family::Name("Big Caslon")])
        .unwrap();

    let style = StandardStyle::default();
    let style = dark_authoratative_style();

    run_simple_ui((), style, fonts, ui)
}
