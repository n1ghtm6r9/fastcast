use crate::support::TestViewer;
use std::time::{Duration, Instant};
use wire::{Packet, decode_datagram};

pub fn next_bye(viewer: &TestViewer, timeout: Duration) -> Option<u32> {
    let deadline = Instant::now() + timeout;

    loop {
        let bytes = viewer.next_datagram(deadline.saturating_duration_since(Instant::now()))?;

        if let Some(datagram) = decode_datagram(&bytes)
            && datagram.packet == Packet::Bye
        {
            return Some(datagram.session);
        }
    }
}
