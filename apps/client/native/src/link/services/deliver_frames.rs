use crate::link::{LinkShared, LinkState};
use std::sync::atomic::Ordering;
use std::time::Instant;
use wire::FrameKind;

pub(crate) fn deliver_frames(shared: &LinkShared, state: &mut LinkState, now: Instant) {
    while let Some(frame) = state.receiver.poll(now) {
        if frame.kind == FrameKind::Video && frame.keyframe {
            shared.key_frame_requested.store(false, Ordering::Release);
        }

        shared.queue.push_frame(frame);
    }
}
