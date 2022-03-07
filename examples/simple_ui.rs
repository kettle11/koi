use koi::*;

fn standard_frame<Data, Context: GetStandardStyle + GetStandardInput>(
    child: impl Widget<Data, Context>,
) -> impl Widget<Data, Context> {
    stack((
        fill(|_, c: &Context| c.standard_style().background_color),
        padding(|_| 50., child),
    ))
}

fn main() {
    let ui = standard_frame(column((
        heading("Counter App"),
        row((text("Count:"), text(|count: &mut i64| count.to_string()))),
        button("Increment", |count| *count += 1),
        button("Decrement", |count| *count -= 1),
    )));

    run_simple_ui(0, StandardStyle::default(), Fonts::default(), ui)
}
