use crate::*;
use kwasm::*;
use std::convert::TryFrom;

pub struct WebXR {
    start_xr: JSObjectDynamic,
    end_xr: JSObjectDynamic,
    get_device_transform: JSObjectDynamic,
    get_view_info: JSObjectDynamic,
    get_view_count: JSObjectDynamic,
    get_xr_framebuffer: JSObjectDynamic,
    running: bool,
    framebuffer: Option<Framebuffer>,
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
            let get_xr_framebuffer = js.get_property("get_xr_framebuffer");

            Ok(Self {
                start_xr,
                end_xr,
                get_device_transform,
                get_view_info,
                get_view_count: js.get_property("get_view_count"),
                get_xr_framebuffer,
                running: false,
                framebuffer: None,
            })
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            self.running = true;
            self.start_xr.call();
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            self.running = false;
            self.end_xr.call();
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    fn get_device_transform(&mut self) -> Mat4 {
        self.get_device_transform.call();
        kwasm::DATA_FROM_HOST.with(|d| unsafe {
            let d = d.borrow();
            let data: &[f32] = std::slice::from_raw_parts(d.as_ptr() as *const f32, 16);
            Mat4::try_from(data).unwrap()
        })
    }

    /*
    fn draw(&mut self) {
        let view_count = self.get_view_count.call().unwrap().get_value_u32();

        // Need to get view information here and call appropriate render functions.
        log!("VIEW COUNT: {:?}", view_count);
        log!("DRAWING XR!");
    }
    */
}

// For now arbitrary IDs will be used to differntiate custom user events.
pub(crate) const XR_EVENT_ID: usize = 8434232;

pub(super) fn xr_control_flow(koi_state: &mut KoiState, event: KappEvent) -> bool {
    match event {
        KappEvent::UserEvent {
            id: XR_EVENT_ID,
            data,
        } => {
            // Update the current thing being rendered.
            (|xr: &mut XR, graphics: &mut Graphics| {
                if xr.framebuffer.is_none() {
                    xr.framebuffer = unsafe {
                        Some(
                            xr.get_xr_framebuffer
                                .call()
                                .map_or(Default::default(), |f| Framebuffer::from_js_object(f)),
                        )
                    };
                }

                graphics.current_target_framebuffer = xr.framebuffer.clone().unwrap();
                graphics.current_camera_target = Some(CameraTarget::XRDevice(0));
                graphics.primary_camera_target = CameraTarget::XRDevice(0);

                let view_count = xr.get_view_count.call().unwrap().get_value_u32();
                graphics.override_views.clear();
                for view_index in 0..view_count {
                    xr.get_view_info.call_raw(&[view_index]);
                    kwasm::DATA_FROM_HOST.with(|data| unsafe {
                        let data = data.borrow_mut();
                        let data: &[f32] =
                            std::slice::from_raw_parts(data.as_ptr() as *const f32, 16 * 2 + 4);
                        let offset_transform = Mat4::try_from(&data[0..16]).unwrap();
                        let projection_matrix = Mat4::try_from(&data[16..32]).unwrap();
                        let viewport = &data[32..36];

                        let multiview_view = CameraView {
                            output_rectangle: BoundingBox::new_with_min_corner_and_size(
                                Vec2::new(viewport[0], viewport[1]),
                                Vec2::new(viewport[2], viewport[3]),
                            ),
                            offset_transform,
                            projection_matrix,
                        };
                        graphics.override_views.push(multiview_view)
                    });
                }
            })
            .run(&mut koi_state.world)
            .unwrap();

            // Update any XR related components in the World
            (|xr: &mut XR, mut xr_heads: Query<(&mut Transform, &XRHead, Option<&mut Camera>)>| {
                // Update the location of the head.
                let device_transform = xr.get_device_transform();
                for (transform, _, camera) in &mut xr_heads {
                    *transform = Transform::from_mat4(device_transform);
                }
            })
            .run(&mut koi_state.world)
            .unwrap();

            // Issue a draw request from the XR device.
            koi_state.draw();
            return true;
        }
        KappEvent::Draw { .. } => {
            let xr = koi_state.world.get_single_component_mut::<XR>().unwrap();
            // If we're running XR suppress regular draw events.
            if xr.running {
                return true;
            }
        }
        _ => {}
    }
    return false;
}

/// Begin rendering an XR frame.
/// Issues a KappEvent::UserEvent to wake up the main event loop.
#[no_mangle]
extern "C" fn koi_begin_xr_frame() {
    // log!("BEGIN XR FRAME!");
    super::USER_EVENT_SENDER.with(|s| {
        s.borrow().as_ref().unwrap().send(XR_EVENT_ID, 0);
    })
}
