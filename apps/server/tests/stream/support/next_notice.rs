use crate::support::{ARRIVAL_LIMIT, TestViewer};
use std::time::Instant;
use wire::{Packet, decode_datagram};

pub fn next_notice(viewer: &TestViewer) -> Option<String> {
    let give_up = Instant::now() + ARRIVAL_LIMIT;

    while let Some(bytes) = viewer.next_datagram(give_up.saturating_duration_since(Instant::now()))
    {
        if let Some(Packet::Notice(text)) = decode_datagram(&bytes).map(|datagram| datagram.packet)
        {
            return Some(text.to_owned());
        }
    }

    None
}
