use crate::journal::TimeOfDay;
#[cfg(not(windows))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(windows)]
use windows::Win32::System::SystemInformation::GetLocalTime;

#[cfg(windows)]
pub fn local_time_of_day() -> TimeOfDay {
    let now = unsafe { GetLocalTime() };

    TimeOfDay {
        hour: now.wHour,
        minute: now.wMinute,
        second: now.wSecond,
        millisecond: now.wMilliseconds,
    }
}

#[cfg(not(windows))]
pub fn local_time_of_day() -> TimeOfDay {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let second_of_day = since_epoch.as_secs() % 86_400;

    TimeOfDay {
        hour: (second_of_day / 3_600) as u16,
        minute: (second_of_day / 60 % 60) as u16,
        second: (second_of_day % 60) as u16,
        millisecond: since_epoch.subsec_millis() as u16,
    }
}
