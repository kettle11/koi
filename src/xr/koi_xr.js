let xr_session = null;
let gl = null;
let xr_reference_space = null;

// Called when we've successfully acquired a XRSession. In response we
// will set up the necessary session state and kick off the frame loop.
function on_session_started(session) {
    xr_session = session;

    // Listen for the sessions 'end' event so we can respond if the user
    // or UA ends the session for any reason.
    session.addEventListener('end', onSessionEnded);

    // Create a WebGL context to render with, initialized to be compatible
    // with the XRDisplay we're presenting to.
    let canvas = document.getElementById('canvas');
    gl = canvas.getContext('webgl2', {
        alpha: false,
        desynchronized: false,
        antialias: true,
        depth: true,
    });
    gl.makeXRCompatible().then(() => {

        // Use the new WebGL context to create a XRWebGLLayer and set it as the
        // sessions baseLayer. This allows any content rendered to the layer to
        // be displayed on the XRDevice.
        session.updateRenderState({ baseLayer: new XRWebGLLayer(session, gl) });

        // Get a reference space, which is required for querying poses. In this
        // case an 'local' reference space means that all poses will be relative
        // to the location where the XRDevice was first detected.
        session.requestReferenceSpace('viewer').then((refSpace) => {
            xr_reference_space = refSpace;

            // Inform the session that we're ready to begin drawing.
            session.requestAnimationFrame(on_xr_frame);
        });
    }
    );
}

let last_frame = null;
let last_pose = null;
let xr_framebuffer = null;
let gl_layer = null;

// XRFrame: https://developer.mozilla.org/en-US/docs/Web/API/XRFrame
function on_xr_frame(time, frame) {
    last_frame = frame;

    let session = frame.session;

    // Inform the session that we're ready for the next frame.
    session.requestAnimationFrame(on_xr_frame);

    // Get the XRDevice pose relative to the reference space we created
    // earlier.
    let pose = frame.getViewerPose(xr_reference_space);
    last_pose = pose;

    gl_layer = session.renderState.baseLayer;
    xr_framebuffer = gl_layer.framebuffer;

    // Getting the pose may fail if, for example, tracking is lost. So we
    // have to check to make sure that we got a valid pose before attempting
    // to render with it. If not in this case we'll just leave the
    // framebuffer cleared, so tracking loss means the scene will simply
    // disappear.

    self.kwasm_exports.koi_begin_xr_frame();


    if (pose) {
        // If we do have a valid pose, bind the WebGL layer's framebuffer,
        // which is where any content to be displayed on the XRDevice must be
        // rendered.
        /*
        gl.bindFramebuffer(gl.FRAMEBUFFER, glLayer.framebuffer);

        // Update the clear color so that we can observe the color in the
        // headset changing over time.
        gl.clearColor(Math.cos(time / 2000),
            Math.cos(time / 4000),
            Math.cos(time / 6000), 1.0);

        // Clear the framebuffer
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
        */
        //console.log(pose);
        // Normally you'd loop through each of the views reported by the frame
        // and draw them into the corresponding viewport here, but we're
        // keeping this sample slim so we're not bothering to draw any
        // geometry.
        /*for (let view of pose.views) {
          let viewport = glLayer.getViewport(view);
          gl.viewport(viewport.x, viewport.y,
                      viewport.width, viewport.height);
          // Draw a scene using view.projectionMatrix as the projection matrix
          // and view.transform to position the virtual camera. If you need a
          // view matrix, use view.transform.inverse.matrix.
        }*/
    }
}

// Called either when the user has explicitly ended the session by calling
// session.end() or when the UA has ended the session for any reason.
// At this point the session object is no longer usable and should be
// discarded.
function onSessionEnded(event) {
    xr_session = null;
    gl = null;
}

function pass_4x4_matrix_to_wasm(matrix) {
    let pointer = self.kwasm_exports.kwasm_reserve_space(16 * 4);
    const client_data = new Float32Array(self.kwasm_memory.buffer, pointer, 16);
    client_data.set(matrix);
}

let kxr = {
    xr_supported() {
        if (navigator.xr) {
            console.log("XR IS SUPPORTED");
            return 1;
        } else {
            return 0;
        }
    },
    start_xr() {
        console.log("START XR CALLED!");
        if (navigator.xr) {
            if (!xr_session) {
                navigator.xr.isSessionSupported('immersive-vr').then((supported) => {
                    console.log("STARTING SESSION!");
                    if (supported) {
                        navigator.xr.requestSession('immersive-vr').then(on_session_started);
                    } else {
                        navigator.xr.requestSession('inline').then(on_session_started);
                    }
                })
            }
        }
    },
    end_xr() {
        if (xr_session) {
            xr_session.end();
            xr_session = null;
        }
    },
    get_view_count() {
        return last_pose.views.length;
    },
    get_view_info(view_index) {
        // https://developer.mozilla.org/en-US/docs/Web/API/XRView
        let view = last_pose.views[view_index];
        let projection_matrix = view.projectionMatrix;
        let transform_matrix = view.transform.matrix;

        // Two matrices and a viewport rectangle.
        let floats_count = 16 * 2 + 4;
        let pointer = self.kwasm_exports.kwasm_reserve_space(floats_count * 4);
        let viewport = gl_layer.getViewport(view);

        const client_data = new Float32Array(self.kwasm_memory.buffer, pointer, floats_count);
        let offset = 0;
        client_data.set(transform_matrix, offset);

        offset += 16;
        client_data.set(projection_matrix, offset);

        offset += 16;
        client_data.set([viewport.x, viewport.y, viewport.width, viewport.height], offset);
        offset += 4;
    },
    get_device_transform() {
        pass_4x4_matrix_to_wasm(last_pose.transform.matrix)
    },
    get_xr_framebuffer() {
        return xr_framebuffer;
    },
};
kxr