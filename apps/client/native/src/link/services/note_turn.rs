use crate::link::LinkShared;
use std::sync::atomic::Ordering;
use std::time::Instant;

pub(crate) fn note_turn(shared: &LinkShared, last_turn: &mut Instant, now: Instant) {
    let pause = now.saturating_duration_since(*last_turn);
    *last_turn = now;

    let pause_ms = u32::try_from(pause.as_millis()).unwrap_or(u32::MAX);
    shared
        .longest_pause_ms
        .fetch_max(pause_ms, Ordering::Relaxed);
}
