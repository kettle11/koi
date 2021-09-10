# koi

A game engine. 

Developed in the open but not yet fit for public use. `koi` is currently designed for my active projects but I'd like to eventually make it useful to others as well.

Most parts are incomplete, code quality varies dramatically, and there are lots of bugs to fix. 

Runs on Mac, Windows, and Web.

Everything is subject to change.

# Crates

## Stand-alone:

`kapp`: Windowing, input, and OpenGL context creation for Windows, Mac, and Web.

`kgltf`: GlTf loader autogenerated from the GlTf specification schema. 

`kecs`: Archetype-based ECS that serves as the backbone of `koi`.

`kmath`: A tiny math library that uses const generics for generic math types.

`kserde`: Json serialization / deserialization. May support other formats in the future.

`kaudio`: Audio backend for Mac, Windows, and Web. (Presently does nothing on windows)


## Tailored to `koi`

`kgraphics`: A wrapper around OpenGL / WebGL to make it a bit more ergonomic. Very tailored to `koi`'s specific needs.

`klog`: A `log!` macro that does the same thing as `println` but it also logs to the console on web.

`kreflect`: Incomplete Rust parser to be used by other proc-macros in `koi` crates. 

`wasm_set_stack_pointer`: A hack needed to set the stack pointer of a new thread on web *without* preprocessing the Wasm binary.

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
        let mut transform = Transform::new_with_position(Vec3::new(0.0, 4.0, 3.0));
        transform.look_at(Vec3::ZERO, Vec3::Y);
        world.spawn((transform, Camera::new(), CameraControls::new()));

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
        }
    });
}
```
