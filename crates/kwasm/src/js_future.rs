use crate::*;
use std::any::Any;
use std::task::{Context, Poll, Waker};
use std::{future::Future, sync::Arc};
use std::{pin::Pin, sync::Mutex};

#[no_mangle]
extern "C" fn kwasm_promise_begin(js_future_inner: u32) -> u32 {
    let arc: Arc<Mutex<JSFutureInner>> =
        unsafe { Arc::from_raw(js_future_inner as *mut std::ffi::c_void as *mut _) };
    let function_to_run = {
        // The main thread can't block, so do busy loop until it gets an opportunity to.
        // This should never be contested for long at all.
        let run_on_promise_thread = loop {
            if let Ok(mut inner) = arc.try_lock() {
                break inner.run_on_promise_thread.take().unwrap();
            }
        };
        (run_on_promise_thread)()
    };

    // Leak this arc because it will be called again when completing
    let _ = Arc::into_raw(arc);
    unsafe { function_to_run.leak() }
}
#[no_mangle]
extern "C" fn kwasm_promise_complete(js_future_inner: u32, result: u32) {
    let result = unsafe { JSObject::new_raw(result) };
    let arc: Arc<Mutex<JSFutureInner>> =
        unsafe { Arc::from_raw(js_future_inner as *mut std::ffi::c_void as *mut _) };

    let waker = {
        let on_completion = {
            let inner = arc.lock().unwrap();
            inner.on_completion
        };

        let result = (on_completion)(result);

        let mut inner = arc.lock().unwrap();
        inner.result = Some(result.unwrap());
        inner.waker.take().unwrap()
    };

    waker.wake();
}

struct JSFutureInner {
    running: bool,
    /// Must return a JS function that accepts 0 args and returns a promise.
    run_on_promise_thread: Option<Box<dyn Fn() -> JSObjectDynamic + Send + Sync>>,
    on_completion: fn(JSObjectDynamic) -> Option<Box<dyn Any + Send + Sync>>,
    result: Option<Box<dyn Any + Send + Sync>>,
    waker: Option<Waker>,
}

pub struct JSFuture {
    // This needs to be shared with a closure passed to the host
    // that fills in the result and drops the closure later.
    inner: Arc<Mutex<JSFutureInner>>,
}

impl JSFuture {
    pub fn new(
        run_on_promise_thread: impl Fn() -> JSObjectDynamic + 'static + Send + Sync,
        on_completion: fn(JSObjectDynamic) -> Option<Box<dyn Any + Send + Sync>>,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(JSFutureInner {
                running: false,
                run_on_promise_thread: Some(Box::new(run_on_promise_thread)),
                on_completion,
                result: None,
                waker: None,
            })),
        }
    }
}

impl<'a> Future for JSFuture {
    type Output = Box<dyn Any + Send + Sync>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // Begin the task.
        let inner_pointer = {
            let mut inner = self.inner.try_lock().unwrap();
            if !inner.running {
                inner.running = true;
                let inner_pointer = Arc::into_raw(self.inner.clone());
                Some(inner_pointer)
            } else {
                None
            }
        };

        // If the task is not yet running start it.
        if let Some(inner_pointer) = inner_pointer {
            unsafe {
                kwasm_run_promise(inner_pointer as u32);
            }
        }

        let mut inner = self.inner.try_lock().unwrap();
        if let Some(v) = inner.result.take() {
            Poll::Ready(v)
        } else {
            inner.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
