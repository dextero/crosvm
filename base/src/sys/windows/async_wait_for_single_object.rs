// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ffi::c_void;
use std::future::Future;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::marker::PhantomData;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::null_mut;
use std::sync::MutexGuard;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

use sync::Mutex;
use winapi::shared::ntdef::FALSE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::threadpoollegacyapiset::UnregisterWaitEx;
use winapi::um::winbase::RegisterWaitForSingleObject;
use winapi::um::winbase::INFINITE;
use winapi::um::winnt::BOOLEAN;
use winapi::um::winnt::PVOID;
use winapi::um::winnt::WT_EXECUTEONLYONCE;

use crate::error;
use crate::warn;
use crate::AsRawDescriptor;
use crate::Descriptor;

/// Async wrapper around `RegisterWaitForSingleObject`. See the official documentation of that
/// function for a list of supported object types.
///
/// The implementation is not tied to any specific async runtime. `Waker::wake` gets invoked from
/// an OS maintained thread pool when the object becomes readable.
pub async fn async_wait_for_single_object(source: &impl AsRawDescriptor) -> Result<()> {
    WaitForHandle::new(source).await
}

/// Inner state shared between the future struct & the kernel invoked waiter callback.
struct WaitForHandleInner {
    wait_state: WaitState,
    wait_object: Descriptor,
    waker: Option<Waker>,
}
impl WaitForHandleInner {
    fn new() -> WaitForHandleInner {
        WaitForHandleInner {
            wait_state: WaitState::New,
            wait_object: Descriptor(null_mut::<c_void>()),
            waker: None,
        }
    }
}

/// Future's state.
#[derive(Clone, Copy, PartialEq, Eq)]
enum WaitState {
    New,
    Sleeping,
    Woken,
    Aborted,
    Finished,
    Failed,
}

struct WaitForHandle<'a, T: AsRawDescriptor> {
    handle: Descriptor,
    inner: Mutex<WaitForHandleInner>,
    _marker: PhantomData<&'a T>,
    _pinned_marker: PhantomPinned,
}

impl<'a, T> WaitForHandle<'a, T>
where
    T: AsRawDescriptor,
{
    fn new(source: &'a T) -> WaitForHandle<'a, T> {
        WaitForHandle {
            handle: Descriptor(source.as_raw_descriptor()),
            inner: Mutex::new(WaitForHandleInner::new()),
            _marker: PhantomData,
            _pinned_marker: PhantomPinned,
        }
    }
}

impl<T> Future for WaitForHandle<'_, T>
where
    T: AsRawDescriptor,
{
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let inner_for_callback = &self.inner as *const _ as *mut Mutex<WaitForHandleInner>;
        let mut inner = self.inner.lock();
        match inner.wait_state {
            WaitState::New => {
                // SAFETY:
                // Safe because:
                //      a) the callback only runs when WaitForHandle is alive (we cancel it on
                //         drop).
                //      b) inner & its children are owned by WaitForHandle.
                let err = unsafe {
                    RegisterWaitForSingleObject(
                        &mut inner.wait_object as *mut _ as *mut *mut c_void,
                        self.handle.0,
                        Some(wait_for_handle_waker),
                        inner_for_callback as *mut c_void,
                        INFINITE,
                        WT_EXECUTEONLYONCE,
                    )
                };
                if err == 0 {
                    return Poll::Ready(Err(Error::last_os_error()));
                }

                inner.wait_state = WaitState::Sleeping;
                inner.waker = Some(cx.waker().clone());
                Poll::Pending
            }
            WaitState::Sleeping => {
                // In case we are polled with a different waker which won't be woken by the existing
                // waker, we'll have to update to the new waker.
                if inner
                    .waker
                    .as_ref()
                    .map(|w| !w.will_wake(cx.waker()))
                    .unwrap_or(true)
                {
                    inner.waker = Some(cx.waker().clone());
                }
                Poll::Pending
            }
            WaitState::Woken => {
                inner.wait_state = WaitState::Finished;

                // SAFETY:
                // Safe because:
                // a) we know a wait was registered and hasn't been unregistered yet.
                // b) the callback is not queued because we set WT_EXECUTEONLYONCE, and we know
                //    it has already completed.
                unsafe { unregister_wait(inner.wait_object) }

                Poll::Ready(Ok(()))
            }
            WaitState::Aborted => {
                Poll::Ready(Err(Error::new(ErrorKind::Other, "operation aborted")))
            }
            WaitState::Finished => panic!("polled an already completed WaitForHandle future."),
            WaitState::Failed => {
                panic!("WaitForHandle future's waiter callback hit unexpected behavior.")
            }
        }
    }
}

impl<T> Drop for WaitForHandle<'_, T>
where
    T: AsRawDescriptor,
{
    fn drop(&mut self) {
        // We cannot hold the lock over the call to unregister_wait, otherwise we could deadlock
        // with the callback trying to access the same data. It is sufficient to just verify
        // (without mutual exclusion beyond the data access itself) that we have exited the New
        // state before attempting to unregister. This works because once we have exited New, we
        // cannot ever re-enter that state, and we know for sure that inner.wait_object is a valid
        // wait object.
        let (current_state, wait_object) = {
            let inner = self.inner.lock();
            (inner.wait_state, inner.wait_object)
        };

        if current_state != WaitState::New && current_state != WaitState::Finished {
            // SAFETY:
            // Safe because self.descriptor is valid in any state except New or Finished.
            //
            // Note: this method call is critical for supplying the safety guarantee relied upon by
            // wait_for_handle_waker. Upon return, it ensures that wait_for_handle_waker is not
            // running and won't be scheduled again, which makes it safe to drop
            // self.inner_for_callback (wait_for_handle_waker has a non owning pointer
            // to self.inner_for_callback).
            unsafe { unregister_wait(wait_object) }
        }
    }
}

/// Safe portion of the RegisterWaitForSingleObject callback.
fn process_wait_state_change(
    mut state: MutexGuard<WaitForHandleInner>,
    wait_fired: bool,
) -> Option<Waker> {
    let mut waker = None;
    state.wait_state = match state.wait_state {
        WaitState::Sleeping => {
            let new_state = if wait_fired {
                WaitState::Woken
            } else {
                // This should never happen.
                error!("wait_for_handle_waker did not wake due to wait firing.");
                WaitState::Aborted
            };

            match state.waker.take() {
                Some(w) => {
                    waker = Some(w);
                    new_state
                }
                None => {
                    error!("wait_for_handler_waker called, but no waker available.");
                    WaitState::Failed
                }
            }
        }
        _ => {
            error!("wait_for_handle_waker called with state != sleeping.");
            WaitState::Failed
        }
    };
    waker
}

/// # Safety
/// a) inner_ptr is valid whenever this function can be called. This is guaranteed by WaitForHandle,
///    which cannot be dropped until this function has finished running & is no longer queued for
///    execution because the Drop impl calls UnregisterWaitEx, which blocks on that condition.
unsafe extern "system" fn wait_for_handle_waker(inner_ptr: PVOID, timer_or_wait_fired: BOOLEAN) {
    let inner = inner_ptr as *const Mutex<WaitForHandleInner>;
    let inner_locked = (*inner).lock();
    let waker = process_wait_state_change(
        inner_locked,
        /* wait_fired= */ timer_or_wait_fired == FALSE,
    );

    // We wake *after* releasing the lock to avoid waking up a thread that then will go back to
    // sleep because the lock it needs is currently held.
    if let Some(w) = waker {
        w.wake()
    }
}

/// # Safety
/// a) desc must be a valid wait handle from RegisterWaitForSingleObject.
unsafe fn unregister_wait(desc: Descriptor) {
    if UnregisterWaitEx(desc.0, INVALID_HANDLE_VALUE) == 0 {
        warn!(
            "WaitForHandle: failed to clean up RegisterWaitForSingleObject wait handle: {}",
            Error::last_os_error()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::time::Duration;

    use super::*;
    use crate::Event;

    struct SimpleWaker {
        wake_tx: mpsc::Sender<()>,
    }

    impl futures::task::ArcWake for SimpleWaker {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            let _ = arc_self.wake_tx.send(());
        }
    }

    #[test]
    fn test_unsignaled_event() {
        let (wake_tx, _wake_rx) = mpsc::channel();
        let waker = futures::task::waker(Arc::new(SimpleWaker { wake_tx }));
        let mut cx = Context::from_waker(&waker);

        let evt = Event::new().unwrap();
        let mut fut = std::pin::pin!(async { async_wait_for_single_object(&evt).await.unwrap() });
        // Assert we make it to the pending state. This means we've registered a wait.
        assert_eq!(fut.as_mut().poll(&mut cx), Poll::Pending);

        // If this test doesn't crash trying to drop the future, it is considered successful.
    }

    #[test]
    fn test_signaled_event() {
        let (wake_tx, wake_rx) = mpsc::channel();
        let waker = futures::task::waker(Arc::new(SimpleWaker { wake_tx }));
        let mut cx = Context::from_waker(&waker);

        let evt = Event::new().unwrap();
        let mut fut = std::pin::pin!(async { async_wait_for_single_object(&evt).await.unwrap() });
        // Should be pending.
        assert_eq!(fut.as_mut().poll(&mut cx), Poll::Pending);
        // Should still be pending.
        assert_eq!(fut.as_mut().poll(&mut cx), Poll::Pending);
        // Signal, wait for wake, then the future should be ready.
        evt.signal().unwrap();
        wake_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("timeout waiting for wake");
        assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready(()));
    }

    #[test]
    fn test_already_signaled_event() {
        let (wake_tx, wake_rx) = mpsc::channel();
        let waker = futures::task::waker(Arc::new(SimpleWaker { wake_tx }));
        let mut cx = Context::from_waker(&waker);

        let evt = Event::new().unwrap();
        evt.signal().unwrap();
        let mut fut = std::pin::pin!(async { async_wait_for_single_object(&evt).await.unwrap() });
        // First call is always pending and registers the wait.
        assert_eq!(fut.as_mut().poll(&mut cx), Poll::Pending);
        // Wait for wake, then the future should be ready.
        wake_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("timeout waiting for wake");
        assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready(()));
    }
}
