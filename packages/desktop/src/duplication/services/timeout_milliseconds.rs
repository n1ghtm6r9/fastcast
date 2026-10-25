use crate::duplication::NANOSECONDS_PER_MILLISECOND;
use std::time::Duration;

pub fn timeout_milliseconds(timeout: Duration) -> u32 {
    let milliseconds = timeout.as_nanos().div_ceil(NANOSECONDS_PER_MILLISECOND);

    u32::try_from(milliseconds).unwrap_or(u32::MAX)
}
