# kApp

kApp is a pure Rust window and input library for macOS, Web, and Windows.

kApp strives to be unsurprising, quick to build, and straightforward to maintain.

A clean build of kApp on macOS takes  about 3.5 seconds.

**kApp is a work in progress.**

kApp is being improved slowly and steadily as issues come up. It is usable as is, but some functionality is missing and everything is subject to change. If you try it out and run into a problem open an issue and please consider contributing!

Currently, to keep the scope manageable, kapp only aims to support the latest of MacOS, Windows, and web browsers. kApp's first priority is consistency and quality for the current platforms, but other platforms may be considered in the future.

Linux support is an eventual goal and an area where contributions and collaboration would be very welcome.

## Example

```rust
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().build().unwrap();

    event_loop.run(move |event| match event {
        Event::WindowCloseRequested { .. } => app.quit(),
        Event::Draw { .. } => {
            // Render something here.
        }
        _ => {}
    });
}
```

## Features

* Create windows
* Mouse input
* Keyboard input
* Event timestamps

## License
`kapp` is licensed under *MIT* or *Apache 2.0* or *Zlib*.

## Similar Projects

The following projects were valuable resources that inspired kApp.

[Winit](https://github.com/rust-windowing/winit)

[Makepad](https://github.com/makepad/makepad)

[Glutin](https://github.com/rust-windowing/glutin)

[SDL2](https://www.libsdl.org/download-2.0.php)

[Sokol](https://github.com/floooh/sokol)

[GLFW](https://www.glfw.org/)
