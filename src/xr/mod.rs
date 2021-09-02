use crate::*;
use core::cell::RefCell;

#[cfg(target_arch = "wasm32")]
mod web_xr;

#[cfg(target_arch = "wasm32")]
type InnerXR = web_xr::WebXR;

#[cfg(not(target_arch = "wasm32"))]
mod open_xr;

#[cfg(not(target_arch = "wasm32"))]
type InnerXR = open_xr::OpenXR;

pub fn xr_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_xr.system()],
        fixed_update_systems: vec![update_xr_transforms.system()],
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
        world.spawn(NotSendSync::new(xr));
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
pub struct XRController;

pub fn update_xr_transforms(
    xr: &mut XR,
    mut heads: Query<(&XRHead, &mut Transform)>,
    mut controllers: Query<(&XRController, &mut Transform)>,
) {
    // Query head transform from xr here

    for (_, transform) in &mut heads {
        // Todo
        // Update each head to the XR transform
    }

    // Query controller transforms here.

    for (_, transform) in &mut controllers {
        // Todo
        // Update each head to the XR transform
    }
}
