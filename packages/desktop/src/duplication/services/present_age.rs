use crate::duplication::ticks_as_duration;
use std::time::Duration;
use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

pub fn present_age(presented: i64) -> Option<Duration> {
    if presented <= 0 {
        return None;
    }

    let mut now = 0;
    let mut frequency = 0;

    unsafe {
        QueryPerformanceCounter(&mut now).ok()?;
        QueryPerformanceFrequency(&mut frequency).ok()?;
    }

    ticks_as_duration(now - presented, frequency)
}
