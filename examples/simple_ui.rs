use koi::*;

fn main() {
    let ui = padding(column((
        heading("Counter App"),
        row((text("Count:"), text(|count: &mut f32| count.to_string()))),
        button("Increment", |count| *count += 1.0),
        button("Decrement", |count| *count -= 1.0),
        slider(|count| count, 0.0, 100.0),
    )));

    run_simple_ui(0.0_f32, StandardStyle::default(), Fonts::default(), ui)
}
