use super::keys_web;

use std::time::Duration;

use kapp_platform_common::*;

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
    event_receiver::set_callback(Box::new(callback));
}

/*
thread_local! {
    static CALLBACK: RefCell<Option<Box<dyn FnMut(Event)>>> = RefCell::new(None);
}
*/

fn send_event(event: Event) {
    event_receiver::send_event(event);
   // CALLBACK.with(|c| (c.borrow_mut().as_mut().unwrap())(event))
}

#[no_mangle]
pub extern "C" fn kapp_on_window_resized(width: u32, height: u32) {
    send_event(Event::WindowResized {
        width,
        height,
        window_id: WindowId::new(0 as *mut std::ffi::c_void),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_animation_frame() {
    // Need to check for client resize here.
    // By comparing canvas width to its client width
    send_event(Event::Draw {
        window_id: WindowId::new(0 as *mut std::ffi::c_void),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_pointer_move(x: f64, y: f64, pointer_enum: u32, time_stamp: f64) {
    send_event(Event::PointerMoved {
        x,
        y,
        source: pointer_source_from_u32(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_mouse_move(delta_x: f64, delta_y: f64, time_stamp: f64) {
    send_event(Event::MouseMotion {
        delta_x,
        delta_y,
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_pointer_down(
    x: f64,
    y: f64,
    pointer_enum: u32,
    button: f64,
    time_stamp: f64,
) {
    send_event(Event::PointerDown {
        button: button_from_f64(button), // This is incorrect
        x,
        y,
        source: pointer_source_from_u32(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_pointer_up(
    x: f64,
    y: f64,
    pointer_enum: u32,
    button: f64,
    time_stamp: f64,
) {
    send_event(Event::PointerUp {
        button: button_from_f64(button), // This is incorrect
        x,
        y,
        source: pointer_source_from_u32(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_key_down(time_stamp: f64) {
    kwasm::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let key = std::str::from_utf8(&d).unwrap();
        send_event(Event::KeyDown {
            key: keys_web::virtual_keycode_to_key(key),
            timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
        })
    })
}

#[no_mangle]
pub extern "C" fn kapp_on_key_up(time_stamp: f64) {
    kwasm::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let key = std::str::from_utf8(&d).unwrap();
        send_event(Event::KeyUp {
            key: keys_web::virtual_keycode_to_key(key),
            timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
        })
    });
}
#[no_mangle]
pub extern "C" fn kapp_on_key_repeat(time_stamp: f64) {
    kwasm::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let key = std::str::from_utf8(&d).unwrap();
        send_event(Event::KeyRepeat {
            key: keys_web::virtual_keycode_to_key(key),
            timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
        });
    });
}

#[no_mangle]
pub extern "C" fn kapp_character_received(_time_stamp: f64) {
    kwasm::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let data = std::str::from_utf8(&d).unwrap();
        let character = data.chars().next().unwrap();
        send_event(Event::CharacterReceived { character })
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_scroll(delta_x: f64, delta_y: f64, time_stamp: f64) {
    send_event(Event::Scroll {
        delta_x,
        delta_y,
        window_id: WindowId::new(0 as *mut std::ffi::c_void),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

// Note that 'feel' adjustments are made on the Javascript side to make this match
// Mac platform behavior. But that may be a bad idea.
#[no_mangle]
pub extern "C" fn kapp_on_pinch(delta: f64, time_stamp: f64) {
    send_event(Event::PinchGesture {
        delta,
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

fn pointer_source_from_u32(f: u32) -> PointerSource {
    match f {
        1 => PointerSource::Mouse,
        2 => PointerSource::Pen,
        3 => PointerSource::Touch,
        _ => PointerSource::Unknown,
    }
}

fn button_from_f64(f: f64) -> PointerButton {
    match f as u32 {
        0 => PointerButton::Primary,
        1 => PointerButton::Auxillary,
        2 => PointerButton::Secondary,
        3 => PointerButton::Extra1,
        4 => PointerButton::Extra2,
        _ => PointerButton::Unknown,
    }
}
