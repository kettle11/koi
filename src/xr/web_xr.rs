use crate::*;
use kwasm::*;
use std::convert::TryFrom;

pub struct WebXR {
    start_xr: JSObjectDynamic,
    end_xr: JSObjectDynamic,
    get_device_transform: JSObjectDynamic,
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

            Ok(Self {
                start_xr,
                end_xr,
                get_device_transform,
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
}
