use crate::support::TestViewer;
use std::time::{Duration, Instant};
use wire::{Packet, Welcome, decode_datagram};

pub fn next_welcome(viewer: &TestViewer, timeout: Duration) -> Option<(u32, Welcome)> {
    let deadline = Instant::now() + timeout;

    loop {
        let bytes = viewer.next_datagram(deadline.saturating_duration_since(Instant::now()))?;

        if let Some(datagram) = decode_datagram(&bytes)
            && let Packet::Welcome(welcome) = datagram.packet
        {
            return Some((datagram.session, welcome));
        }
    }
}
