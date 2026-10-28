use crate::capture::raise_thread_priority;
use windows::Win32::System::Threading::{
    GetCurrentThread, GetThreadPriority, THREAD_PRIORITY_HIGHEST,
};

#[test]
fn the_capture_thread_runs_at_the_highest_priority() {
    raise_thread_priority().unwrap();

    assert_eq!(
        unsafe { GetThreadPriority(GetCurrentThread()) },
        THREAD_PRIORITY_HIGHEST.0
    );
}
