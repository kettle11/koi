use crate::*;
use kwasm::*;
use std::convert::TryFrom;

pub struct WebXR {
    start_xr: JSObjectDynamic,
    end_xr: JSObjectDynamic,
    get_device_transform: JSObjectDynamic,
    get_view_info: JSObjectDynamic,
    get_view_count: JSObjectDynamic,
}

impl WebXR {
    pub fn initialize() -> Result<Self, ()> {
        let js = JSObjectFromString::new(include_str!("koi_xr.js"));
        let xr_supported = js.get_property("xr_supported");

        if xr_supported.call().unwrap().get_value_u32() == 0 {
            Err(())
        } else {
            let start_xr = js.get_property("start_xr");
            let end_xr = js.get_property("end_xr");
            let get_device_transform = js.get_property("get_device_transform");
            let get_view_info = js.get_property("get_view_info");

            Ok(Self {
                start_xr,
                end_xr,
                get_device_transform,
                get_view_info,
                get_view_count: js.get_property("get_view_count"),
            })
        }
    }

    pub fn start(&mut self) {
        self.start_xr.call();
    }

    pub fn stop(&mut self) {
        self.end_xr.call();
    }

    pub fn get_device_transform(&mut self) -> Mat4 {
        self.get_device_transform.call();
        kwasm::DATA_FROM_HOST.with(|d| unsafe {
            let d = d.borrow();
            let data: &[f32] = std::slice::from_raw_parts(d.as_ptr() as *const f32, 16);
            Mat4::try_from(data).unwrap()
        })
    }

    pub fn draw(&mut self) {
        let view_count = self.get_view_count.call().unwrap().get_value_u32();
        
        // Need to get view information here and call appropriate render functions.
        log!("VIEW COUNT: {:?}", view_count);
        log!("DRAWING XR!");
    }
}

// For now arbitrary IDs will be used to differntiate custom user events.
pub(crate) const XR_EVENT_ID: usize = 8434232;

pub(super) fn on_kapp_events(xr: &mut XR, events: &KappEvents) {
    match events.last() {
        Some(KappEvent::UserEvent {
            id: XR_EVENT_ID,
            data,
        }) => {
            log!("XR EVENT!");
            xr.draw();
        }
        _ => {}
    }
}

/// Begin rendering an XR frame.
/// Issues a KappEvent::UserEvent to wake up the main event loop.
#[no_mangle]
extern "C" fn koi_begin_xr_frame() {
    log!("BEGIN XR FRAME!");
    super::USER_EVENT_SENDER.with(|s| {
        s.borrow().as_ref().unwrap().send(XR_EVENT_ID, 0);
    })
}
