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
    /*
        pub setup_systems: Vec<System>,
        pub pre_fixed_update_systems: Vec<System>,
        pub fixed_update_systems: Vec<System>,
        pub draw_systems: Vec<System>,
        pub end_of_frame_systems: Vec<System>,
    } */
    Plugin {
        setup_systems: vec![setup_xr.system()],
        fixed_update_systems: vec![update_xr_transforms.system()],
        ..Default::default()
    }
}
thread_local!(
    static USER_EVENT_SENDER: RefCell<Option<kapp::UserEventSender>> = RefCell::new(None);
);

/// Begin rendering an XR frame.
#[no_mangle]
extern "C" fn koi_begin_xr_frame() {
    USER_EVENT_SENDER.with(|s| {
        s.borrow().as_ref().unwrap().send(0, 0);
    })
}

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

#[derive(Clone)]
pub struct MultiviewView {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    /// Transform relative to the [MultiViewCamera]
    transform: Transform,
    projection_matrix: Mat4,
}

/// A camera that renders to multiple viewports at once.
/// Used for [XR].
#[derive(Component, Clone)]
pub struct MultiViewCamera {
    views: Vec<MultiviewView>,
}

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
