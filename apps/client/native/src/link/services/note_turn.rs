use crate::link::{LinkShared, LinkState};
use std::sync::atomic::Ordering;
use std::time::Instant;

pub(crate) fn note_turn(shared: &LinkShared, state: &mut LinkState, now: Instant) {
    let pause = now.saturating_duration_since(state.last_turn);
    state.last_turn = now;

    let pause_ms = u32::try_from(pause.as_millis()).unwrap_or(u32::MAX);
    shared
        .longest_pause_ms
        .fetch_max(pause_ms, Ordering::Relaxed);
}
