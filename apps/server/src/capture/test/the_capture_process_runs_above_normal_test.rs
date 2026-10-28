use crate::capture::raise_process_priority;
use windows::Win32::System::Threading::{
    ABOVE_NORMAL_PRIORITY_CLASS, GetCurrentProcess, GetPriorityClass,
};

#[test]
fn the_capture_process_runs_above_normal() {
    raise_process_priority().unwrap();

    assert_eq!(
        unsafe { GetPriorityClass(GetCurrentProcess()) },
        ABOVE_NORMAL_PRIORITY_CLASS.0
    );
}
