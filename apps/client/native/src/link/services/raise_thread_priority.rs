#[cfg(any(target_os = "linux", target_os = "android"))]
use crate::link::LINK_THREAD_PRIORITY;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) fn raise_thread_priority() {
    unsafe {
        libc::setpriority(
            libc::PRIO_PROCESS,
            libc::gettid() as libc::id_t,
            LINK_THREAD_PRIORITY,
        );
    }
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub(crate) fn raise_thread_priority() {}
