use crate::*;
use std::task::{Context, Poll, Waker};
use std::{future::Future, sync::Arc};
use std::{pin::Pin, sync::Mutex};

thread_local! {
    static FETCH_FUNCTION: JSObjectFromString = JSObjectFromString::new(include_str!("fetch.js"));
}

pub async fn fetch(path: &str) -> Result<Vec<u8>, ()> {
    FetchFuture {
        inner: Arc::new(Mutex::new(Inner {
            path: path.to_string(),
            running: false,
            result: None,
            waker: None,
        })),
    }
    .await
}

struct Inner {
    path: String,
    running: bool,
    result: Option<Vec<u8>>,
    waker: Option<Waker>,
}

struct FetchFuture {
    // This needs to be shared with a closure passed to the host
    // that fills in the result and drops the closure later.
    inner: Arc<Mutex<Inner>>,
}

impl<'a> Future for FetchFuture {
    type Output = Result<Vec<u8>, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();

        // Begin the task.
        if !inner.running {
            let raw_ptr = Arc::into_raw(self.inner.clone());
            inner.running = true;

            let js_string = JSString::new(&inner.path);

            FETCH_FUNCTION
                .with(|f| f.call_raw(&JSObject::NULL, &[js_string.index(), raw_ptr as u32]));
        }

        if let Some(v) = inner.result.take() {
            Poll::Ready(Ok(v))
        } else {
            inner.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

/// Called by the host to reserve scratch space to pass data into kwasm.
/// returns a pointer to the allocated data.
#[no_mangle]
extern "C" fn kwasm_complete_fetch(inner_data: u32) {
    unsafe {
        let arc = Arc::<Mutex<Inner>>::from_raw(inner_data as *const Mutex<Inner>);

        let waker = {
            let mut inner = arc.lock().unwrap();

            DATA_FROM_HOST.with(|d| {
                let data = d.take();
                inner.result = Some(data);
            });
            inner.waker.take().unwrap()
        };

        // Drop the lock before we wake the task that will also try to access the lock.
        waker.wake(); // Wake up our task.
    }
}
