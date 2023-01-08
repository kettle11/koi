# koi

A game engine.

Developed in the open but not yet fit for public use. `koi` is currently designed for my active projects but I'd like to eventually make it useful to others as well.

## Projects I've built with `koi`

[bloom3d](https://bloom3d.com)

[SIMD terrain generator](https://ianjk.com/terrain_generator/)

[Last of the Sky Folk](https://ianjk.com/ld50/)

**Expect frequent build breaks!**

Most parts are incomplete, code quality varies dramatically, and there are lots of bugs to fix.

Runs on Mac, Windows, and Web.

Everything is subject to change.

## How to run examples

Install Rust: <https://www.rust-lang.org/tools/install>

On Mac and Windows run `cargo run --example hello`

For Web:

* install `devserver` with `cargo install devserver`
* run `./run.sh hello`
* Navigate your browser to `localhost:8080`

## What works / doesn't work?

Everything is subject to massive change, but some parts are more functional than others.

Presently the core loop, user input, windowing, ECS, audio, and rendering are quite usable. Rendering will continue to change substantially but it already works for many purposes.

The user-interface (UI) code is *nearly* in an interesting and useful state but not it's quite there yet.

The "physics" code doesn't work at all. It's very work in progress.

# Crates

## Stand-alone

`kapp`: Windowing, input, and OpenGL context creation for Windows, Mac, and Web.

`kgltf`: GlTf loader autogenerated from the GlTf specification schema.

`kecs`: Archetype-based ECS that serves as the backbone of `koi`.

`kmath`: A tiny math library that uses const generics for generic math types.

`kserde`: Json serialization / deserialization. May support other formats in the future.

`kaudio`: Audio backend for Mac, Windows, and Web. (Presently does nothing on Windows)

## Tailored to `koi`

`kgraphics`: A wrapper around OpenGL / WebGL to make it a bit more ergonomic. Very tailored to `koi`'s specific needs.

`klog`: A `log!` macro that does the same thing as `println` but it also logs to the console on web.

`kreflect`: Incomplete Rust parser to be used by other proc-macros in `koi` crates.

`ktasks`: A multithreaded task system that works on native and web. Needs improvement.

`kwasm`: Rather hacky ways to interact with web APIs.

## Example

This example creates a cube that can be controlled with the arrow keys and a camera that can view the cube.

```rust
use koi::*;

#[derive(Component, Clone)]
struct Controlled;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.

        // Spawn a camera and make it look towards the origin.
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0.0, 4.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            Camera::new(),
            CameraControls::new(),
        ));

        // Spawn a cube that we can control
        world.spawn((Transform::new(), Mesh::CUBE, Material::UNLIT, Controlled));

        move |event: Event, world: &mut World| {
            match event {
                Event::FixedUpdate => {
                    // Perform physics and game related updates here.

                    // Control the cube.
                    (|input: &Input, mut things_to_move: Query<(&mut Transform, &Controlled)>| {
                        for (transform, _) in &mut things_to_move {
                            if input.key(Key::Left) {
                                transform.position -= Vec3::X * 0.1;
                            }
                            if input.key(Key::Right) {
                                transform.position += Vec3::X * 0.1;
                            }
                            if input.key(Key::Up) {
                                transform.position -= Vec3::Z * 0.1;
                            }
                            if input.key(Key::Down) {
                                transform.position += Vec3::Z * 0.1;
                            }
                        }
                    })
                    .run(world)
                }
                Event::Draw => {
                    // Things that occur before rendering can go here.
                }
                _ => {}
            }

            // Do not consume the event and allow other systems to respond to it.
            false
        }
    });
}

```
