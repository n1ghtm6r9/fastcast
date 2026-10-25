use crate::receiver::{FrameReceiver, ReceivedFrame};
use std::iter::from_fn;
use std::time::Instant;

pub fn deliveries(receiver: &mut FrameReceiver, now: Instant) -> Vec<ReceivedFrame> {
    from_fn(|| receiver.poll(now)).collect()
}
