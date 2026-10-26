use crate::link::test::{connected_link, spin_for};
use std::time::Duration;
use wire::{FrameKind, decode_datagram};

pub(crate) fn datagrams_a_link_loses_on_a_paced_stream(seed: u64, count: usize) -> u64 {
    let (mut server, _link, peer) = connected_link();
    let mut state = seed.wrapping_mul(7919).wrapping_add(12345);
    let mut last_sequence = 0;

    for _ in 0..count {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        spin_for(Duration::from_micros((state >> 33) % 4000));

        let datagrams = server.send_frame(peer, FrameKind::Video, true, &[1], 0);

        last_sequence = decode_datagram(datagrams.last().unwrap()).unwrap().sequence;
    }

    let feedback = server.last_feedback_reaching(last_sequence);

    (u64::from(last_sequence) + 1).saturating_sub(u64::from(feedback.received_packets))
}
