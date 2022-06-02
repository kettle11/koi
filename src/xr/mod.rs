use crate::*;
use core::cell::RefCell;

#[cfg(target_arch = "wasm32")]
mod web_xr;

#[cfg(target_arch = "wasm32")]
type InnerXR = web_xr::WebXR;

#[cfg(not(target_arch = "wasm32"))]
mod do_nothing_xr_backend;

#[cfg(not(target_arch = "wasm32"))]
type InnerXR = do_nothing_xr_backend::DoNothingXrBackend;

pub fn xr_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_xr.system()],
        #[cfg(target_arch = "wasm32")]
        additional_control_flow: vec![Box::new(web_xr::xr_control_flow)],
        ..Default::default()
    }
}
thread_local!(
    static USER_EVENT_SENDER: RefCell<Option<kapp::UserEventSender>> = RefCell::new(None);
);

pub type XR = NotSendSync<InnerXR>;

fn setup_xr(world: &mut World) {
    log!("INITIALIZING XR");
    let application = world
        .get_single_component_mut::<NotSendSync<kapp::Application>>()
        .unwrap();

    USER_EVENT_SENDER.with(|s| {
        s.replace(Some(application.get_user_event_sender()));
    });
    if let Ok(xr) = InnerXR::initialize() {
        world.spawn((Name("XR".to_string()), NotSendSync::new(xr)));
    } else {
        log!("XR IS UNSUPPORTED");
    }
}

/// Attach this to a transform to have its local transform updated to match
/// the currently attached XR device head transform.
#[derive(Component, Clone)]
pub struct XRHead;

/// Attach this to a transform to have its local transform updated to match
/// an XRController position.
#[derive(Component, Clone)]
pub struct XRController {
    pub id: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct ButtonState {
    pub value: f32,
    pub pressed: bool,
    pub touched: bool,
}

pub trait XRBackendTrait {
    fn start(&mut self);
    fn stop(&mut self);
    fn running(&self) -> bool;
    fn button_state(&self, controller_index: usize, button_index: usize) -> bool;
    fn button_just_pressed(&self, controller_index: usize, button_index: usize) -> bool;
    fn button_just_released(&self, controller_index: usize, button_index: usize) -> bool;
}
