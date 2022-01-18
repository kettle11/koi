use koi::*;
use kui::{rectangle, StandardStyle, Widget};

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        /*
        let ui = kui::row(kui::for_each(
            |data: &mut World, f| {
                (|mut transforms: Query<&mut Transform>| {
                    for transform in &mut transforms {
                        f(transform)
                    }
                })
                .run(data);
            },
            || kui::rectangle(Vec2::fill(100.), Color::RED),
        ));
        */
        //let ui = kui::row((
        //     //rectangle(Vec2::fill(100.), Color::RED),
        //     // rectangle(Vec2::fill(200.), Color::BLUE),
        //     // rectangle(Vec2::fill(300.), Color::GREEN),
        //     // kui::heading("Hello"),
        // ));
        //let mut ui = kui::heading("hello");
        // let ui = kui::rectangle(Vec2::fill(200.), Color::BLUE);

        let mut ui = UI::new(world, kui::StandardConstraints::default());
        let mut widget_root = rectangle(Vec2::fill(200.), Color::RED);
        let standard_style = StandardStyle::default();
        move |event: Event, world: &mut World| {
            //let mut drawer = kui::Drawer::new();
            //widget_root.update_layout_draw(state, &mut drawer, kui::StandardConstraints::default());

            //ui.update(&mut (), &mut widget_root);

            //
            // // match event {
            // //     Event::Draw => {
            //
            let state_wrapper = StandardState::new(world, &mut standard_style);

            let mut v = 10.;
            for _ in 0..3 {
                //let mut state = &mut v;
                ui.update(&mut state_wrapper, &mut widget_root);
            }
            // ui.draw(world);
            //    }
            //    _ => {}
            //}
            false
        }
    });
}

fn move_into_closure<T>(t: T) -> T {
    t
}
