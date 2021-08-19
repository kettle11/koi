use koi::*;

// Custom components need to derive "Component".
#[derive(Component, Clone)]
struct Thingy;

fn main() {
    App::new()
        .setup_and_run(|world: &mut World| {
            // Setup things here.
            let thingy = world.spawn(Thingy);
            
            // Run the World with this mutable closure.
            move |event: Event, _: &mut World| {
                match event {
                    Event::FixedUpdate => {
                        println!("Hello!: {:?}", thingy)
                    },
                    Event::Draw => {
                        // Things that occur before rendering can go here.
                    }
                }
            }
        });
}
