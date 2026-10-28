use crate::stream::MILLISECONDS_PER_SECOND;
use std::time::Duration;

pub fn milliseconds_of(duration: Duration) -> f64 {
    duration.as_secs_f64() * MILLISECONDS_PER_SECOND
}
