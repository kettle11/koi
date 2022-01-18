use koi::*;
use kui::{rectangle, StandardConstraints, StandardStyle, Widget};

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        let mut style = StandardStyle::new();

        // let mut data = "Hello";

        // Load a default font.
        style
            .new_font(include_bytes!("../Inter-Regular.otf"))
            .unwrap();

        let mut root_widget = kui::column((
            kui::heading(|d: &_| "Hello".to_string()),
            kui::heading(|d: &_| "Hi there!".to_string()),
        ));

        let mut ui_manager = UIManager::new(world, kui::StandardConstraints::default());

        move |event: Event, world| {
            match event {
                Event::Draw => {
                    ui_manager.draw(world);
                    ui_manager.update(world, &mut style, &mut root_widget);
                }
                _ => {}
            }
            false
        }
    });
}

struct Thingy<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> Thingy<T> {
    fn do_thing(&mut self, state: T) {}
}
struct Wrapper<'a, T> {
    t: &'a mut T,
}

/*
fn call_with_wrapper<'a, State>(
    state: &'a mut State,
    widget: &mut dyn Widget<Wrapper<'a, State>, StandardConstraints, kui::Drawer>,
) {
    let mut wrapper = Wrapper { t: state };
    widget.update_layout_draw(
        &mut wrapper,
        &mut kui::Drawer::new(),
        StandardConstraints::default(),
    )
}
*/
