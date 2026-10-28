use crate::duplication::present_age;
use std::time::Duration;
use windows::Win32::System::Performance::QueryPerformanceCounter;

#[test]
fn a_frame_presented_just_now_is_young() {
    let mut now = 0;
    unsafe { QueryPerformanceCounter(&mut now) }.expect("the performance counter runs");

    let age = present_age(now).expect("a frame presented just now has an age");

    assert!(age < Duration::from_millis(100), "{age:?}");
}
