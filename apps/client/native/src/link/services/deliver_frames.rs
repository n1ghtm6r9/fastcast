use crate::link::{LinkShared, LinkState};
use std::time::Instant;

pub(crate) fn deliver_frames(shared: &LinkShared, state: &mut LinkState, now: Instant) {
    while let Some(frame) = state.receiver.poll(now) {
        shared.queue.push_frame(frame);
    }
}
