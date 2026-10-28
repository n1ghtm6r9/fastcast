#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) fn raise_thread_priority(priority: i32) -> bool {
    unsafe { libc::setpriority(libc::PRIO_PROCESS, libc::gettid() as libc::id_t, priority) == 0 }
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub(crate) fn raise_thread_priority(_priority: i32) -> bool {
    false
}
