use crate::duplication::NANOSECONDS_PER_SECOND;
use std::time::Duration;

pub fn ticks_as_duration(ticks: i64, frequency: i64) -> Option<Duration> {
    let ticks = u128::try_from(ticks).ok()?;
    let frequency = u128::try_from(frequency)
        .ok()
        .filter(|frequency| *frequency > 0)?;
    let nanoseconds = u64::try_from(ticks * NANOSECONDS_PER_SECOND / frequency).ok()?;

    Some(Duration::from_nanos(nanoseconds))
}
