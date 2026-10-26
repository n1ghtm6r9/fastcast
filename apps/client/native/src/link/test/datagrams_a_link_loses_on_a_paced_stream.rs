use crate::link::test::{ReceivedPacket, connected_link, spin_for};
use std::thread;
use std::time::Duration;
use wire::FrameKind;

pub(crate) fn datagrams_a_link_loses_on_a_paced_stream(seed: u64, count: usize) -> u64 {
    let (mut server, _link, peer) = connected_link();
    let mut state = seed.wrapping_mul(7919).wrapping_add(12345);

    for _ in 0..count {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        spin_for(Duration::from_micros((state >> 33) % 4000));
        server.send_frame(peer, FrameKind::Video, true, &[1], 0);
    }

    thread::sleep(Duration::from_millis(100));

    let feedback = server
        .packets_within(Duration::from_millis(100))
        .into_iter()
        .rev()
        .find_map(|packet| match packet {
            ReceivedPacket::Feedback { feedback, .. } => Some(feedback),
            _ => None,
        })
        .unwrap();

    u64::from(feedback.highest_sequence) + 1 - u64::from(feedback.received_packets)
}
