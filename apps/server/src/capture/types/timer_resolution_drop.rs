use crate::capture::{TIMER_PERIOD_MILLISECONDS, TimerResolution};
use windows::Win32::Media::timeEndPeriod;

impl Drop for TimerResolution {
    fn drop(&mut self) {
        unsafe { timeEndPeriod(TIMER_PERIOD_MILLISECONDS) };
    }
}
