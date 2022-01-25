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
    get_input_count: JSObjectDynamic,
    get_input_info: JSObjectDynamic,
    get_button_count: JSObjectDynamic,
    get_button_info: JSObjectDynamic,
    running: bool,
    framebuffer: Option<Framebuffer>,
    /// Only setup for two controllers at the moment.
    controller_state: [ControllerState; 2],
}

#[derive(Debug)]
pub struct ControllerState {
    pub buttons: Vec<ButtonState>,
    pub previous_buttons: Vec<ButtonState>,
}

impl ControllerState {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            previous_buttons: Vec::new(),
        }
    }
}

impl WebXR {
    pub fn initialize() -> Result<Self, ()> {
        let js = JSObjectFromString::new(include_str!("koi_xr.js"));
        let xr_supported = js.get_property("xr_supported");

        if xr_supported.call().unwrap().get_value_u32() == 0 {
            Err(())
        } else {
            Ok(Self {
                start_xr: js.get_property("start_xr"),
                end_xr: js.get_property("end_xr"),
                get_device_transform: js.get_property("get_device_transform"),
                get_view_info: js.get_property("get_view_info"),
                get_view_count: js.get_property("get_view_count"),
                get_input_count: js.get_property("get_input_count"),
                get_input_info: js.get_property("get_input_info"),
                get_xr_framebuffer: js.get_property("get_xr_framebuffer"),
                get_button_count: js.get_property("get_button_count"),
                get_button_info: js.get_property("get_button_info"),
                running: false,
                framebuffer: None,
                controller_state: [ControllerState::new(), ControllerState::new()],
            })
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            log("STARTING XR!");
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

    fn get_device_transform(&self) -> Mat4 {
        self.get_device_transform.call();
        kwasm::DATA_FROM_HOST.with(|d| unsafe {
            let d = d.borrow();
            let data: &[f32] = std::slice::from_raw_parts(d.as_ptr() as *const f32, 16);
            Mat4::try_from(data).unwrap()
        })
    }

    fn get_controller_matrix(&self, index: usize) -> Mat4 {
        self.get_input_info.call_raw(&[index as u32]);
        kwasm::DATA_FROM_HOST.with(|d| unsafe {
            let d = d.borrow();
            let data: &[f32] = std::slice::from_raw_parts(d.as_ptr() as *const f32, 16);
            Mat4::try_from(data).unwrap()
        })
    }

    fn update_controller_state(&mut self) {
        fn update_controller_state_inner(
            get_button_count: &JSObject,
            get_button_info: &JSObject,
            controller_index: usize,
            controller_state: &mut ControllerState,
        ) {
            std::mem::swap(
                &mut controller_state.buttons,
                &mut controller_state.previous_buttons,
            );
            controller_state.buttons.clear();
            let button_count = get_button_count
                .call_raw(&[controller_index as u32])
                .unwrap()
                .get_value_u32();
            for button_index in 0..button_count {
                get_button_info.call_raw(&[controller_index as u32, button_index as u32]);
                let button_data = kwasm::DATA_FROM_HOST.with(|d| unsafe {
                    let d = d.borrow();
                    let data: &[f32] = std::slice::from_raw_parts(d.as_ptr() as *const f32, 3);
                    ButtonState {
                        value: data[0],
                        pressed: data[1] == 1.0,
                        touched: data[2] == 1.0,
                    }
                });
                controller_state.buttons.push(button_data);
            }
        }
        update_controller_state_inner(
            &self.get_button_count,
            &self.get_button_info,
            0,
            &mut self.controller_state[0],
        );
        update_controller_state_inner(
            &self.get_button_count,
            &self.get_button_info,
            1,
            &mut self.controller_state[1],
        )
    }

    /*
    pub fn get_button_state(&self, controller_index: usize, button_index: usize) -> ButtonState {
        self.get_button_info
            .call_raw(&[controller_index as u32, button_index as u32]);
        kwasm::DATA_FROM_HOST.with(|d| unsafe {
            let d = d.borrow();
            let data: &[f32] = std::slice::from_raw_parts(d.as_ptr() as *const f32, 3);
            ButtonState {
                value: data[0],
                pressed: data[1] == 1.0,
                touched: data[2] == 1.0,
            }
        })
    }
    */

    pub fn button_state(&self, controller_index: usize, button_index: usize) -> bool {
        if let Some(this_state) = self.controller_state[controller_index]
            .buttons
            .get(button_index)
        {
            this_state.pressed
        } else {
            false
        }
    }

    pub fn button_just_pressed(&self, controller_index: usize, button_index: usize) -> bool {
        if let Some(last_state) = self.controller_state[controller_index]
            .previous_buttons
            .get(button_index)
        {
            if let Some(this_state) = self.controller_state[controller_index]
                .buttons
                .get(button_index)
            {
                return !last_state.pressed && this_state.pressed;
            }
        }
        false
    }

    pub fn button_just_released(&self, controller_index: usize, button_index: usize) -> bool {
        if let Some(last_state) = self.controller_state[controller_index]
            .previous_buttons
            .get(button_index)
        {
            if let Some(this_state) = self.controller_state[controller_index]
                .buttons
                .get(button_index)
            {
                return last_state.pressed && !this_state.pressed;
            }
        }
        false
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
            data: 1,
        } => {
            // Disable XR and reset values.
            (|xr: &mut XR, window: &mut NotSendSync<kapp::Window>, graphics: &mut Graphics| {
                xr.running = false;
                xr.framebuffer = None;
                graphics.override_views.clear();
                graphics.primary_camera_target = CameraTarget::Window(window.id);

                // Begin the main render loop again.
                window.request_redraw();
            })
            .run(&mut koi_state.world);
        }
        KappEvent::UserEvent {
            id: XR_EVENT_ID,
            data: 0,
        } => {
            let device_transform = (|xr: &XR| xr.get_device_transform()).run(&mut koi_state.world);
            let device_inverse = device_transform.inversed();
            // Update the current thing being rendered.
            (|xr: &mut XR, graphics: &mut Graphics| {
                xr.update_controller_state();

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

                        // offset from head position
                        let offset_transform = offset_transform * device_inverse;

                        let projection_matrix = Mat4::try_from(&data[16..32]).unwrap();
                        let viewport = &data[32..36];

                        let multiview_view = GraphicsViewInfo {
                            output_rectangle: Box2::new_with_min_corner_and_size(
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
            .run(&mut koi_state.world);

            // Update any XR related components in the World
            // For now just update all cameras to match the head position
            (|mut xr_heads: Query<(&mut Transform, &mut Camera)>| {
                // Update the location of the head.
                for (transform, _) in &mut xr_heads {
                    *transform = Transform::from_mat4(device_transform);
                }
            })
            .run(&mut koi_state.world);

            (|xr: &XR, mut xr_controllers: Query<(&mut Transform, &XRController)>| {
                // Update the location of the controller.
                for (transform, controller) in &mut xr_controllers {
                    let controller_matrix = xr.get_controller_matrix(controller.id);
                    let scale = transform.scale;
                    *transform = Transform::from_mat4(controller_matrix);

                    // Only take rotation and position from the transform.
                    transform.scale = scale;
                }
            })
            .run(&mut koi_state.world);

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

#[no_mangle]
extern "C" fn koi_end_xr() {
    log!("ENDING XR!");
    super::USER_EVENT_SENDER.with(|s| {
        s.borrow().as_ref().unwrap().send(XR_EVENT_ID, 1);
    })
}
