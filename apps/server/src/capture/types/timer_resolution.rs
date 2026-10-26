use crate::capture::TIMER_PERIOD_MILLISECONDS;
use windows::Win32::Media::timeBeginPeriod;

pub struct TimerResolution;

impl TimerResolution {
    pub fn one_millisecond() -> TimerResolution {
        unsafe { timeBeginPeriod(TIMER_PERIOD_MILLISECONDS) };

        TimerResolution
    }
}
